#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_amount::Amount;
use use_transaction::TransactionDirection;

/// Common BAI2 primitives.
pub mod prelude {
    pub use crate::{
        AccountIdentifierRecord, AccountTrailerRecord, Bai2Error, ContinuationRecord,
        FileHeaderRecord, FileTrailerRecord, FundsTypeCode, GroupHeaderRecord, GroupTrailerRecord,
        NormalizedTransaction, RawRecord, RecordCode, TransactionDetailRecord, TransactionTypeCode,
        parse_line, parse_logical_records,
    };
}

/// Supported BAI2 record codes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RecordCode {
    /// `01` file header.
    FileHeader,
    /// `02` group header.
    GroupHeader,
    /// `03` account identifier.
    AccountIdentifier,
    /// `16` transaction detail.
    TransactionDetail,
    /// `49` account trailer.
    AccountTrailer,
    /// `88` continuation.
    Continuation,
    /// `98` group trailer.
    GroupTrailer,
    /// `99` file trailer.
    FileTrailer,
}

impl RecordCode {
    /// Parses a BAI2 record code.
    ///
    /// # Errors
    ///
    /// Returns [`Bai2Error::UnknownRecordCode`] when the code is not supported.
    pub fn new(value: impl AsRef<str>) -> Result<Self, Bai2Error> {
        match value.as_ref().trim() {
            "01" => Ok(Self::FileHeader),
            "02" => Ok(Self::GroupHeader),
            "03" => Ok(Self::AccountIdentifier),
            "16" => Ok(Self::TransactionDetail),
            "49" => Ok(Self::AccountTrailer),
            "88" => Ok(Self::Continuation),
            "98" => Ok(Self::GroupTrailer),
            "99" => Ok(Self::FileTrailer),
            other => Err(Bai2Error::UnknownRecordCode(other.to_string())),
        }
    }

    /// Returns the two-character BAI2 record code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileHeader => "01",
            Self::GroupHeader => "02",
            Self::AccountIdentifier => "03",
            Self::TransactionDetail => "16",
            Self::AccountTrailer => "49",
            Self::Continuation => "88",
            Self::GroupTrailer => "98",
            Self::FileTrailer => "99",
        }
    }
}

impl fmt::Display for RecordCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RecordCode {
    type Err = Bai2Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A parsed BAI2 record with raw fields preserved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawRecord {
    code: RecordCode,
    fields: Vec<String>,
}

impl RawRecord {
    /// Creates a raw record.
    #[must_use]
    pub const fn new(code: RecordCode, fields: Vec<String>) -> Self {
        Self { code, fields }
    }

    /// Returns the record code.
    #[must_use]
    pub const fn code(&self) -> RecordCode {
        self.code
    }

    /// Returns raw fields after the record code.
    #[must_use]
    pub fn fields(&self) -> &[String] {
        &self.fields
    }

    fn push_fields(&mut self, fields: Vec<String>) {
        self.fields.extend(fields);
    }
}

/// Parses a single slash-terminated BAI2 line.
///
/// # Errors
///
/// Returns [`Bai2Error`] when the line is empty, lacks a slash terminator, or uses an unsupported
/// record code.
pub fn parse_line(line: &str) -> Result<RawRecord, Bai2Error> {
    let line = line.trim();
    if line.is_empty() {
        return Err(Bai2Error::EmptyLine);
    }

    let Some(content) = line.strip_suffix('/') else {
        return Err(Bai2Error::MissingTerminator);
    };

    let mut parts = content.split(',');
    let code = parts.next().ok_or(Bai2Error::MissingRecordCode)?;
    let code = RecordCode::new(code)?;
    let fields = parts.map(|field| field.trim().to_string()).collect();
    Ok(RawRecord::new(code, fields))
}

/// Parses BAI2 lines and folds `88` continuation records into the previous logical record.
///
/// # Errors
///
/// Returns [`Bai2Error::OrphanContinuation`] when a continuation appears before any logical record,
/// plus any parsing error returned by [`parse_line`].
pub fn parse_logical_records(input: &str) -> Result<Vec<RawRecord>, Bai2Error> {
    let mut records: Vec<RawRecord> = Vec::new();

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let record = parse_line(line)?;
        if record.code() == RecordCode::Continuation {
            let Some(previous) = records.last_mut() else {
                return Err(Bai2Error::OrphanContinuation);
            };
            previous.push_fields(record.fields);
        } else {
            records.push(record);
        }
    }

    Ok(records)
}

/// BAI2 file header record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileHeaderRecord {
    sender_id: String,
    receiver_id: String,
    creation_date: String,
    creation_time: String,
    file_id: Option<String>,
}

impl FileHeaderRecord {
    /// Returns the sender identifier.
    #[must_use]
    pub fn sender_id(&self) -> &str {
        &self.sender_id
    }

    /// Returns the receiver identifier.
    #[must_use]
    pub fn receiver_id(&self) -> &str {
        &self.receiver_id
    }

    /// Returns the creation date field.
    #[must_use]
    pub fn creation_date(&self) -> &str {
        &self.creation_date
    }

    /// Returns the creation time field.
    #[must_use]
    pub fn creation_time(&self) -> &str {
        &self.creation_time
    }

    /// Returns the optional file identifier.
    #[must_use]
    pub fn file_id(&self) -> Option<&str> {
        self.file_id.as_deref()
    }
}

impl TryFrom<&RawRecord> for FileHeaderRecord {
    type Error = Bai2Error;

    fn try_from(record: &RawRecord) -> Result<Self, Self::Error> {
        ensure_code(record, RecordCode::FileHeader)?;
        Ok(Self {
            sender_id: required_field(record, 0, "sender_id")?.to_string(),
            receiver_id: required_field(record, 1, "receiver_id")?.to_string(),
            creation_date: required_field(record, 2, "creation_date")?.to_string(),
            creation_time: required_field(record, 3, "creation_time")?.to_string(),
            file_id: optional_field(record, 4),
        })
    }
}

/// BAI2 group header record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupHeaderRecord {
    receiver_id: String,
    originator_id: String,
    group_status: String,
    as_of_date: String,
    as_of_time: String,
}

impl TryFrom<&RawRecord> for GroupHeaderRecord {
    type Error = Bai2Error;

    fn try_from(record: &RawRecord) -> Result<Self, Self::Error> {
        ensure_code(record, RecordCode::GroupHeader)?;
        Ok(Self {
            receiver_id: required_field(record, 0, "receiver_id")?.to_string(),
            originator_id: required_field(record, 1, "originator_id")?.to_string(),
            group_status: required_field(record, 2, "group_status")?.to_string(),
            as_of_date: required_field(record, 3, "as_of_date")?.to_string(),
            as_of_time: required_field(record, 4, "as_of_time")?.to_string(),
        })
    }
}

impl GroupHeaderRecord {
    /// Returns the receiver identifier.
    #[must_use]
    pub fn receiver_id(&self) -> &str {
        &self.receiver_id
    }

    /// Returns the originator identifier.
    #[must_use]
    pub fn originator_id(&self) -> &str {
        &self.originator_id
    }

    /// Returns the group status field.
    #[must_use]
    pub fn group_status(&self) -> &str {
        &self.group_status
    }

    /// Returns the as-of date field.
    #[must_use]
    pub fn as_of_date(&self) -> &str {
        &self.as_of_date
    }

    /// Returns the as-of time field.
    #[must_use]
    pub fn as_of_time(&self) -> &str {
        &self.as_of_time
    }
}

/// BAI2 account identifier record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIdentifierRecord {
    customer_account_number: String,
    currency_code: Option<String>,
    summary_fields: Vec<String>,
}

impl TryFrom<&RawRecord> for AccountIdentifierRecord {
    type Error = Bai2Error;

    fn try_from(record: &RawRecord) -> Result<Self, Self::Error> {
        ensure_code(record, RecordCode::AccountIdentifier)?;
        Ok(Self {
            customer_account_number: required_field(record, 0, "customer_account_number")?
                .to_string(),
            currency_code: optional_field(record, 1),
            summary_fields: record.fields().get(2..).unwrap_or_default().to_vec(),
        })
    }
}

impl AccountIdentifierRecord {
    /// Returns the customer account number.
    #[must_use]
    pub fn customer_account_number(&self) -> &str {
        &self.customer_account_number
    }

    /// Returns the optional currency code field.
    #[must_use]
    pub fn currency_code(&self) -> Option<&str> {
        self.currency_code.as_deref()
    }

    /// Returns preserved summary fields.
    #[must_use]
    pub fn summary_fields(&self) -> &[String] {
        &self.summary_fields
    }
}

/// A raw BAI2 transaction type code.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransactionTypeCode(String);

impl TransactionTypeCode {
    /// Creates a raw transaction type code.
    ///
    /// # Errors
    ///
    /// Returns [`Bai2Error::MissingField`] when the code is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, Bai2Error> {
        let value = value.as_ref().trim();
        if value.is_empty() {
            return Err(Bai2Error::MissingField {
                record: RecordCode::TransactionDetail,
                field: "transaction_type_code",
            });
        }
        Ok(Self(value.to_string()))
    }

    /// Returns the raw transaction type code.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn direction(&self) -> Result<TransactionDirection, Bai2Error> {
        match self.0.as_bytes().first().copied() {
            Some(b'1' | b'2' | b'3') => Ok(TransactionDirection::Inflow),
            Some(b'4' | b'5' | b'6') => Ok(TransactionDirection::Outflow),
            _ => Err(Bai2Error::UnknownTransactionDirection(self.0.clone())),
        }
    }
}

/// A raw BAI2 funds type code.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FundsTypeCode(String);

impl FundsTypeCode {
    /// Creates a raw funds type code.
    ///
    /// # Errors
    ///
    /// Returns [`Bai2Error::MissingField`] when the code is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, Bai2Error> {
        let value = value.as_ref().trim();
        if value.is_empty() {
            return Err(Bai2Error::MissingField {
                record: RecordCode::TransactionDetail,
                field: "funds_type_code",
            });
        }
        Ok(Self(value.to_string()))
    }

    /// Returns the raw funds type code.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// BAI2 transaction detail record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionDetailRecord {
    transaction_type: TransactionTypeCode,
    amount: Amount,
    funds_type: Option<FundsTypeCode>,
    bank_reference: Option<String>,
    customer_reference: Option<String>,
    text: Option<String>,
}

impl TryFrom<&RawRecord> for TransactionDetailRecord {
    type Error = Bai2Error;

    fn try_from(record: &RawRecord) -> Result<Self, Self::Error> {
        ensure_code(record, RecordCode::TransactionDetail)?;
        let transaction_type =
            TransactionTypeCode::new(required_field(record, 0, "transaction_type_code")?)?;
        let amount = parse_amount(required_field(record, 1, "amount")?)?;
        let funds_type = match optional_field(record, 2) {
            Some(value) => Some(FundsTypeCode::new(value)?),
            None => None,
        };
        let text = record.fields().get(5..).and_then(|fields| {
            if fields.is_empty() {
                None
            } else {
                Some(fields.join(","))
            }
        });

        Ok(Self {
            transaction_type,
            amount,
            funds_type,
            bank_reference: optional_field(record, 3),
            customer_reference: optional_field(record, 4),
            text,
        })
    }
}

impl TransactionDetailRecord {
    /// Returns the transaction type code.
    #[must_use]
    pub const fn transaction_type(&self) -> &TransactionTypeCode {
        &self.transaction_type
    }

    /// Returns the transaction amount, interpreted as minor units with scale 2.
    #[must_use]
    pub const fn amount(&self) -> Amount {
        self.amount
    }

    /// Returns the optional funds type code.
    #[must_use]
    pub const fn funds_type(&self) -> Option<&FundsTypeCode> {
        self.funds_type.as_ref()
    }

    /// Returns the optional bank reference.
    #[must_use]
    pub fn bank_reference(&self) -> Option<&str> {
        self.bank_reference.as_deref()
    }

    /// Returns the optional customer reference.
    #[must_use]
    pub fn customer_reference(&self) -> Option<&str> {
        self.customer_reference.as_deref()
    }

    /// Returns the optional transaction text.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}

/// BAI2 continuation record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinuationRecord {
    fields: Vec<String>,
}

impl TryFrom<&RawRecord> for ContinuationRecord {
    type Error = Bai2Error;

    fn try_from(record: &RawRecord) -> Result<Self, Self::Error> {
        ensure_code(record, RecordCode::Continuation)?;
        Ok(Self {
            fields: record.fields().to_vec(),
        })
    }
}

impl ContinuationRecord {
    /// Returns continuation fields.
    #[must_use]
    pub fn fields(&self) -> &[String] {
        &self.fields
    }
}

/// BAI2 account trailer record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountTrailerRecord {
    account_control_total: Option<i128>,
    record_count: Option<usize>,
}

impl TryFrom<&RawRecord> for AccountTrailerRecord {
    type Error = Bai2Error;

    fn try_from(record: &RawRecord) -> Result<Self, Self::Error> {
        ensure_code(record, RecordCode::AccountTrailer)?;
        Ok(Self {
            account_control_total: optional_i128(record, 0)?,
            record_count: optional_usize(record, 1)?,
        })
    }
}

impl AccountTrailerRecord {
    /// Returns the optional account control total.
    #[must_use]
    pub const fn account_control_total(&self) -> Option<i128> {
        self.account_control_total
    }

    /// Returns the optional record count.
    #[must_use]
    pub const fn record_count(&self) -> Option<usize> {
        self.record_count
    }
}

/// BAI2 group trailer record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupTrailerRecord {
    group_control_total: Option<i128>,
    account_count: Option<usize>,
    record_count: Option<usize>,
}

impl TryFrom<&RawRecord> for GroupTrailerRecord {
    type Error = Bai2Error;

    fn try_from(record: &RawRecord) -> Result<Self, Self::Error> {
        ensure_code(record, RecordCode::GroupTrailer)?;
        Ok(Self {
            group_control_total: optional_i128(record, 0)?,
            account_count: optional_usize(record, 1)?,
            record_count: optional_usize(record, 2)?,
        })
    }
}

impl GroupTrailerRecord {
    /// Returns the optional group control total.
    #[must_use]
    pub const fn group_control_total(&self) -> Option<i128> {
        self.group_control_total
    }

    /// Returns the optional account count.
    #[must_use]
    pub const fn account_count(&self) -> Option<usize> {
        self.account_count
    }

    /// Returns the optional record count.
    #[must_use]
    pub const fn record_count(&self) -> Option<usize> {
        self.record_count
    }
}

/// BAI2 file trailer record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileTrailerRecord {
    file_control_total: Option<i128>,
    group_count: Option<usize>,
    record_count: Option<usize>,
}

impl TryFrom<&RawRecord> for FileTrailerRecord {
    type Error = Bai2Error;

    fn try_from(record: &RawRecord) -> Result<Self, Self::Error> {
        ensure_code(record, RecordCode::FileTrailer)?;
        Ok(Self {
            file_control_total: optional_i128(record, 0)?,
            group_count: optional_usize(record, 1)?,
            record_count: optional_usize(record, 2)?,
        })
    }
}

impl FileTrailerRecord {
    /// Returns the optional file control total.
    #[must_use]
    pub const fn file_control_total(&self) -> Option<i128> {
        self.file_control_total
    }

    /// Returns the optional group count.
    #[must_use]
    pub const fn group_count(&self) -> Option<usize> {
        self.group_count
    }

    /// Returns the optional record count.
    #[must_use]
    pub const fn record_count(&self) -> Option<usize> {
        self.record_count
    }

    /// Validates a parsed logical record count against the trailer record count when present.
    ///
    /// # Errors
    ///
    /// Returns [`Bai2Error::InvalidCount`] when the expected count is present and differs.
    pub const fn validate_record_count(&self, actual: usize) -> Result<(), Bai2Error> {
        match self.record_count {
            Some(expected) if expected != actual => Err(Bai2Error::InvalidCount),
            _ => Ok(()),
        }
    }
}

/// A normalized transaction detail value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedTransaction {
    transaction_type: TransactionTypeCode,
    amount: Amount,
    direction: TransactionDirection,
    bank_reference: Option<String>,
    customer_reference: Option<String>,
    text: Option<String>,
}

impl NormalizedTransaction {
    /// Normalizes a BAI2 transaction detail record.
    ///
    /// # Errors
    ///
    /// Returns [`Bai2Error::UnknownTransactionDirection`] when the raw type code cannot be mapped
    /// into a conservative inflow or outflow direction.
    pub fn from_detail(detail: &TransactionDetailRecord) -> Result<Self, Bai2Error> {
        Ok(Self {
            transaction_type: detail.transaction_type.clone(),
            amount: detail.amount,
            direction: detail.transaction_type.direction()?,
            bank_reference: detail.bank_reference.clone(),
            customer_reference: detail.customer_reference.clone(),
            text: detail.text.clone(),
        })
    }

    /// Returns the raw transaction type code.
    #[must_use]
    pub const fn transaction_type(&self) -> &TransactionTypeCode {
        &self.transaction_type
    }

    /// Returns the normalized amount.
    #[must_use]
    pub const fn amount(&self) -> Amount {
        self.amount
    }

    /// Returns the normalized direction.
    #[must_use]
    pub const fn direction(&self) -> TransactionDirection {
        self.direction
    }

    /// Returns the optional bank reference.
    #[must_use]
    pub fn bank_reference(&self) -> Option<&str> {
        self.bank_reference.as_deref()
    }

    /// Returns the optional customer reference.
    #[must_use]
    pub fn customer_reference(&self) -> Option<&str> {
        self.customer_reference.as_deref()
    }

    /// Returns the optional transaction text.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}

/// Errors returned by BAI2 parsing and validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Bai2Error {
    /// The input line was empty.
    EmptyLine,
    /// A BAI2 line did not end with `/`.
    MissingTerminator,
    /// A record code was missing.
    MissingRecordCode,
    /// The record code is unsupported.
    UnknownRecordCode(String),
    /// A record had an unexpected code.
    UnexpectedRecordCode {
        /// Expected record code.
        expected: RecordCode,
        /// Actual record code.
        actual: RecordCode,
    },
    /// A required field was missing.
    MissingField {
        /// Record code.
        record: RecordCode,
        /// Field name.
        field: &'static str,
    },
    /// An amount field was invalid.
    InvalidAmount,
    /// A count field was invalid or unexpected.
    InvalidCount,
    /// A continuation record appeared before a logical record.
    OrphanContinuation,
    /// A transaction type code could not be mapped into a direction.
    UnknownTransactionDirection(String),
}

impl fmt::Display for Bai2Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLine => formatter.write_str("BAI2 line cannot be empty"),
            Self::MissingTerminator => formatter.write_str("BAI2 line must end with /"),
            Self::MissingRecordCode => formatter.write_str("BAI2 record code is missing"),
            Self::UnknownRecordCode(code) => {
                write!(formatter, "unsupported BAI2 record code: {code}")
            },
            Self::UnexpectedRecordCode { expected, actual } => write!(
                formatter,
                "expected BAI2 record code {expected}, got {actual}"
            ),
            Self::MissingField { record, field } => {
                write!(formatter, "BAI2 record {record} missing field {field}")
            },
            Self::InvalidAmount => formatter.write_str("BAI2 amount field is invalid"),
            Self::InvalidCount => formatter.write_str("BAI2 count field is invalid"),
            Self::OrphanContinuation => {
                formatter.write_str("BAI2 continuation record has no parent")
            },
            Self::UnknownTransactionDirection(code) => write!(
                formatter,
                "BAI2 transaction type code {code} has unknown direction"
            ),
        }
    }
}

impl Error for Bai2Error {}

fn ensure_code(record: &RawRecord, expected: RecordCode) -> Result<(), Bai2Error> {
    if record.code() == expected {
        Ok(())
    } else {
        Err(Bai2Error::UnexpectedRecordCode {
            expected,
            actual: record.code(),
        })
    }
}

fn required_field<'a>(
    record: &'a RawRecord,
    index: usize,
    field: &'static str,
) -> Result<&'a str, Bai2Error> {
    let value = record
        .fields()
        .get(index)
        .map(String::as_str)
        .unwrap_or_default()
        .trim();
    if value.is_empty() {
        Err(Bai2Error::MissingField {
            record: record.code(),
            field,
        })
    } else {
        Ok(value)
    }
}

fn optional_field(record: &RawRecord, index: usize) -> Option<String> {
    record.fields().get(index).and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn optional_i128(record: &RawRecord, index: usize) -> Result<Option<i128>, Bai2Error> {
    optional_field(record, index)
        .map(|value| value.parse::<i128>().map_err(|_| Bai2Error::InvalidAmount))
        .transpose()
}

fn optional_usize(record: &RawRecord, index: usize) -> Result<Option<usize>, Bai2Error> {
    optional_field(record, index)
        .map(|value| value.parse::<usize>().map_err(|_| Bai2Error::InvalidCount))
        .transpose()
}

fn parse_amount(value: &str) -> Result<Amount, Bai2Error> {
    let minor_units = value
        .parse::<i128>()
        .map_err(|_| Bai2Error::InvalidAmount)?;
    Amount::from_minor_units(minor_units, 2).map_err(|_| Bai2Error::InvalidAmount)
}

#[cfg(test)]
mod tests {
    use use_transaction::TransactionDirection;

    use super::{
        Bai2Error, FileTrailerRecord, NormalizedTransaction, RawRecord, RecordCode,
        TransactionDetailRecord, parse_line, parse_logical_records,
    };

    #[test]
    fn parses_transaction_detail_line() -> Result<(), Box<dyn std::error::Error>> {
        let record = parse_line("16,475,12345,Z,bank-ref,customer-ref,payment/")?;
        let detail = TransactionDetailRecord::try_from(&record)?;
        let normalized = NormalizedTransaction::from_detail(&detail)?;

        assert_eq!(record.code(), RecordCode::TransactionDetail);
        assert_eq!(detail.amount().minor_units(), 12_345);
        assert_eq!(detail.bank_reference(), Some("bank-ref"));
        assert_eq!(normalized.direction(), TransactionDirection::Outflow);
        Ok(())
    }

    #[test]
    fn folds_continuation_records() -> Result<(), Box<dyn std::error::Error>> {
        let records = parse_logical_records(
            "16,475,12345,Z,bank-ref,customer-ref,first/\n88,second,third/\n",
        )?;
        let detail = TransactionDetailRecord::try_from(&records[0])?;

        assert_eq!(records.len(), 1);
        assert_eq!(detail.text(), Some("first,second,third"));
        Ok(())
    }

    #[test]
    fn rejects_invalid_record_code_and_orphan_continuation() {
        assert_eq!(
            parse_line("77,abc/"),
            Err(Bai2Error::UnknownRecordCode("77".to_string()))
        );
        assert_eq!(
            parse_logical_records("88,orphan/"),
            Err(Bai2Error::OrphanContinuation)
        );
    }

    #[test]
    fn validates_file_trailer_count() -> Result<(), Box<dyn std::error::Error>> {
        let trailer_record = RawRecord::new(
            RecordCode::FileTrailer,
            vec!["0".to_string(), "1".to_string(), "3".to_string()],
        );
        let trailer = FileTrailerRecord::try_from(&trailer_record)?;

        assert_eq!(trailer.validate_record_count(3), Ok(()));
        assert_eq!(
            trailer.validate_record_count(2),
            Err(Bai2Error::InvalidCount)
        );
        Ok(())
    }
}
