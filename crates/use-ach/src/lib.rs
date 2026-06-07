#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common ACH primitives.
pub mod prelude {
    pub use crate::{
        AchAccountType, AchAddendaIndicator, AchCompanyId, AchEntry, AchEntryDirection, AchError,
        AchIndividualId, AchStandardEntryClass, AchTraceNumber, AchTransactionCode,
    };
}

/// Conservative ACH Standard Entry Class vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AchStandardEntryClass {
    /// Corporate credit or debit entry.
    Ccd,
    /// Corporate trade exchange entry.
    Ctx,
    /// Prearranged payment and deposit entry.
    Ppd,
    /// Telephone-initiated entry.
    Tel,
    /// Internet-initiated or mobile entry.
    Web,
    /// Accounts receivable check conversion entry.
    Arc,
    /// Back office conversion entry.
    Boc,
    /// Point-of-purchase entry.
    Pop,
    /// Re-presented check entry.
    Rck,
    /// International ACH transaction entry.
    Iat,
}

impl AchStandardEntryClass {
    /// Returns the three-character SEC code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ccd => "CCD",
            Self::Ctx => "CTX",
            Self::Ppd => "PPD",
            Self::Tel => "TEL",
            Self::Web => "WEB",
            Self::Arc => "ARC",
            Self::Boc => "BOC",
            Self::Pop => "POP",
            Self::Rck => "RCK",
            Self::Iat => "IAT",
        }
    }

    /// Creates a SEC value from a three-character code.
    ///
    /// # Errors
    ///
    /// Returns [`AchError::InvalidStandardEntryClass`] when the code is not in this crate's
    /// conservative SEC vocabulary.
    pub fn from_code(value: impl AsRef<str>) -> Result<Self, AchError> {
        let value = value.as_ref().trim();
        if value.eq_ignore_ascii_case("CCD") {
            Ok(Self::Ccd)
        } else if value.eq_ignore_ascii_case("CTX") {
            Ok(Self::Ctx)
        } else if value.eq_ignore_ascii_case("PPD") {
            Ok(Self::Ppd)
        } else if value.eq_ignore_ascii_case("TEL") {
            Ok(Self::Tel)
        } else if value.eq_ignore_ascii_case("WEB") {
            Ok(Self::Web)
        } else if value.eq_ignore_ascii_case("ARC") {
            Ok(Self::Arc)
        } else if value.eq_ignore_ascii_case("BOC") {
            Ok(Self::Boc)
        } else if value.eq_ignore_ascii_case("POP") {
            Ok(Self::Pop)
        } else if value.eq_ignore_ascii_case("RCK") {
            Ok(Self::Rck)
        } else if value.eq_ignore_ascii_case("IAT") {
            Ok(Self::Iat)
        } else {
            Err(AchError::InvalidStandardEntryClass)
        }
    }
}

impl fmt::Display for AchStandardEntryClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AchStandardEntryClass {
    type Err = AchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_code(value)
    }
}

/// Broad ACH account type vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AchAccountType {
    /// Checking account.
    Checking,
    /// Savings account.
    Savings,
    /// Loan account.
    Loan,
}

impl AchAccountType {
    /// Returns a stable lowercase account type label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
            Self::Loan => "loan",
        }
    }
}

impl fmt::Display for AchAccountType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// ACH entry direction vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AchEntryDirection {
    /// Credit entry.
    Credit,
    /// Debit entry.
    Debit,
}

impl AchEntryDirection {
    /// Returns a stable lowercase direction label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Credit => "credit",
            Self::Debit => "debit",
        }
    }
}

impl fmt::Display for AchEntryDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Conservative ACH transaction-code vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AchTransactionCode {
    /// Credit destined for a checking account.
    CheckingCredit,
    /// Prenotification credit destined for a checking account.
    CheckingPrenoteCredit,
    /// Debit destined for a checking account.
    CheckingDebit,
    /// Prenotification debit destined for a checking account.
    CheckingPrenoteDebit,
    /// Credit destined for a savings account.
    SavingsCredit,
    /// Prenotification credit destined for a savings account.
    SavingsPrenoteCredit,
    /// Debit destined for a savings account.
    SavingsDebit,
    /// Prenotification debit destined for a savings account.
    SavingsPrenoteDebit,
    /// Credit destined for a loan account.
    LoanCredit,
    /// Prenotification credit destined for a loan account.
    LoanPrenoteCredit,
}

impl AchTransactionCode {
    /// Returns the two-digit NACHA transaction code as a number.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::CheckingCredit => 22,
            Self::CheckingPrenoteCredit => 23,
            Self::CheckingDebit => 27,
            Self::CheckingPrenoteDebit => 28,
            Self::SavingsCredit => 32,
            Self::SavingsPrenoteCredit => 33,
            Self::SavingsDebit => 37,
            Self::SavingsPrenoteDebit => 38,
            Self::LoanCredit => 52,
            Self::LoanPrenoteCredit => 53,
        }
    }

    /// Creates a transaction code from a numeric NACHA transaction code.
    ///
    /// # Errors
    ///
    /// Returns [`AchError::InvalidTransactionCode`] when the code is not in this crate's
    /// conservative transaction-code vocabulary.
    pub const fn from_code(code: u8) -> Result<Self, AchError> {
        match code {
            22 => Ok(Self::CheckingCredit),
            23 => Ok(Self::CheckingPrenoteCredit),
            27 => Ok(Self::CheckingDebit),
            28 => Ok(Self::CheckingPrenoteDebit),
            32 => Ok(Self::SavingsCredit),
            33 => Ok(Self::SavingsPrenoteCredit),
            37 => Ok(Self::SavingsDebit),
            38 => Ok(Self::SavingsPrenoteDebit),
            52 => Ok(Self::LoanCredit),
            53 => Ok(Self::LoanPrenoteCredit),
            _ => Err(AchError::InvalidTransactionCode),
        }
    }

    /// Returns the account type implied by the transaction code.
    #[must_use]
    pub const fn account_type(self) -> AchAccountType {
        match self {
            Self::CheckingCredit
            | Self::CheckingPrenoteCredit
            | Self::CheckingDebit
            | Self::CheckingPrenoteDebit => AchAccountType::Checking,
            Self::SavingsCredit
            | Self::SavingsPrenoteCredit
            | Self::SavingsDebit
            | Self::SavingsPrenoteDebit => AchAccountType::Savings,
            Self::LoanCredit | Self::LoanPrenoteCredit => AchAccountType::Loan,
        }
    }

    /// Returns the direction implied by the transaction code.
    #[must_use]
    pub const fn direction(self) -> AchEntryDirection {
        match self {
            Self::CheckingCredit
            | Self::CheckingPrenoteCredit
            | Self::SavingsCredit
            | Self::SavingsPrenoteCredit
            | Self::LoanCredit
            | Self::LoanPrenoteCredit => AchEntryDirection::Credit,
            Self::CheckingDebit
            | Self::CheckingPrenoteDebit
            | Self::SavingsDebit
            | Self::SavingsPrenoteDebit => AchEntryDirection::Debit,
        }
    }

    /// Returns whether the transaction code is a prenotification code.
    #[must_use]
    pub const fn is_prenote(self) -> bool {
        matches!(
            self,
            Self::CheckingPrenoteCredit
                | Self::CheckingPrenoteDebit
                | Self::SavingsPrenoteCredit
                | Self::SavingsPrenoteDebit
                | Self::LoanPrenoteCredit
        )
    }
}

impl fmt::Display for AchTransactionCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:02}", self.code())
    }
}

impl FromStr for AchTransactionCode {
    type Err = AchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();
        let bytes = value.as_bytes();
        if bytes.len() != 2 || !bytes.iter().all(u8::is_ascii_digit) {
            return Err(AchError::InvalidTransactionCode);
        }

        let code = ((bytes[0] - b'0') * 10) + (bytes[1] - b'0');
        Self::from_code(code)
    }
}

/// ACH addenda indicator vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AchAddendaIndicator {
    /// No addenda record is attached.
    NoAddenda,
    /// One or more addenda records are attached.
    Addenda,
}

impl AchAddendaIndicator {
    /// Returns the single-character NACHA addenda indicator.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAddenda => "0",
            Self::Addenda => "1",
        }
    }

    /// Returns whether addenda are present.
    #[must_use]
    pub const fn has_addenda(self) -> bool {
        matches!(self, Self::Addenda)
    }

    /// Creates an addenda indicator from `0` or `1`.
    ///
    /// # Errors
    ///
    /// Returns [`AchError::InvalidAddendaIndicator`] when the value is not `0` or `1`.
    pub fn from_code(value: impl AsRef<str>) -> Result<Self, AchError> {
        match value.as_ref().trim() {
            "0" => Ok(Self::NoAddenda),
            "1" => Ok(Self::Addenda),
            _ => Err(AchError::InvalidAddendaIndicator),
        }
    }
}

impl fmt::Display for AchAddendaIndicator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AchAddendaIndicator {
    type Err = AchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_code(value)
    }
}

/// A validated 15-digit ACH trace number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AchTraceNumber(String);

impl AchTraceNumber {
    /// Creates an ACH trace number from 15 digits.
    ///
    /// # Errors
    ///
    /// Returns [`AchError::InvalidTraceNumberLength`] when the trimmed input is not 15 bytes and
    /// [`AchError::InvalidTraceNumberCharacter`] when any byte is not a digit.
    pub fn new(value: impl AsRef<str>) -> Result<Self, AchError> {
        let value = value.as_ref().trim();
        if value.len() != 15 {
            return Err(AchError::InvalidTraceNumberLength);
        }

        if !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(AchError::InvalidTraceNumberCharacter);
        }

        Ok(Self(value.to_owned()))
    }

    /// Returns the full 15-digit trace number.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the first eight digits of the trace number.
    #[must_use]
    pub fn odfi_identification(&self) -> &str {
        &self.0[..8]
    }

    /// Returns the final seven digits of the trace number.
    #[must_use]
    pub fn sequence_number(&self) -> &str {
        &self.0[8..]
    }
}

impl AsRef<str> for AchTraceNumber {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AchTraceNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AchTraceNumber {
    type Err = AchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A conservatively validated ACH company identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AchCompanyId(String);

impl AchCompanyId {
    /// Creates an ACH company identifier from 1 to 10 conservative ASCII characters.
    ///
    /// # Errors
    ///
    /// Returns [`AchError::EmptyCompanyId`] when the trimmed input is empty,
    /// [`AchError::CompanyIdTooLong`] when the input is longer than 10 bytes, and
    /// [`AchError::InvalidCompanyIdCharacter`] when the input contains unsupported characters.
    pub fn new(value: impl AsRef<str>) -> Result<Self, AchError> {
        validate_identifier(
            value.as_ref(),
            10,
            AchError::EmptyCompanyId,
            AchError::CompanyIdTooLong,
            AchError::InvalidCompanyIdCharacter,
        )
        .map(Self)
    }

    /// Returns the company identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AchCompanyId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AchCompanyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AchCompanyId {
    type Err = AchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A conservatively validated ACH individual identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AchIndividualId(String);

impl AchIndividualId {
    /// Creates an ACH individual identifier from 1 to 15 conservative ASCII characters.
    ///
    /// # Errors
    ///
    /// Returns [`AchError::EmptyIndividualId`] when the trimmed input is empty,
    /// [`AchError::IndividualIdTooLong`] when the input is longer than 15 bytes, and
    /// [`AchError::InvalidIndividualIdCharacter`] when the input contains unsupported characters.
    pub fn new(value: impl AsRef<str>) -> Result<Self, AchError> {
        validate_identifier(
            value.as_ref(),
            15,
            AchError::EmptyIndividualId,
            AchError::IndividualIdTooLong,
            AchError::InvalidIndividualIdCharacter,
        )
        .map(Self)
    }

    /// Returns the individual identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AchIndividualId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AchIndividualId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AchIndividualId {
    type Err = AchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Lightweight ACH entry metadata composed from validated primitives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AchEntry {
    standard_entry_class: AchStandardEntryClass,
    transaction_code: AchTransactionCode,
    trace_number: AchTraceNumber,
    company_id: AchCompanyId,
    individual_id: AchIndividualId,
    addenda_indicator: AchAddendaIndicator,
}

impl AchEntry {
    /// Creates ACH entry metadata with no addenda indicator attached.
    #[must_use]
    pub const fn new(
        standard_entry_class: AchStandardEntryClass,
        transaction_code: AchTransactionCode,
        trace_number: AchTraceNumber,
        company_id: AchCompanyId,
        individual_id: AchIndividualId,
    ) -> Self {
        Self {
            standard_entry_class,
            transaction_code,
            trace_number,
            company_id,
            individual_id,
            addenda_indicator: AchAddendaIndicator::NoAddenda,
        }
    }

    /// Returns the standard entry class.
    #[must_use]
    pub const fn standard_entry_class(&self) -> AchStandardEntryClass {
        self.standard_entry_class
    }

    /// Returns the transaction code.
    #[must_use]
    pub const fn transaction_code(&self) -> AchTransactionCode {
        self.transaction_code
    }

    /// Returns the trace number.
    #[must_use]
    pub const fn trace_number(&self) -> &AchTraceNumber {
        &self.trace_number
    }

    /// Returns the company identifier.
    #[must_use]
    pub const fn company_id(&self) -> &AchCompanyId {
        &self.company_id
    }

    /// Returns the individual identifier.
    #[must_use]
    pub const fn individual_id(&self) -> &AchIndividualId {
        &self.individual_id
    }

    /// Returns the addenda indicator.
    #[must_use]
    pub const fn addenda_indicator(&self) -> AchAddendaIndicator {
        self.addenda_indicator
    }

    /// Sets the addenda indicator.
    #[must_use]
    pub const fn with_addenda_indicator(mut self, addenda_indicator: AchAddendaIndicator) -> Self {
        self.addenda_indicator = addenda_indicator;
        self
    }
}

/// Errors returned by ACH primitives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AchError {
    /// The SEC code is outside this crate's conservative vocabulary.
    InvalidStandardEntryClass,
    /// The transaction code is outside this crate's conservative vocabulary.
    InvalidTransactionCode,
    /// The addenda indicator was not `0` or `1`.
    InvalidAddendaIndicator,
    /// ACH trace numbers must be exactly 15 digits.
    InvalidTraceNumberLength,
    /// ACH trace numbers must contain only digits.
    InvalidTraceNumberCharacter,
    /// The company identifier was empty after trimming whitespace.
    EmptyCompanyId,
    /// The company identifier was longer than 10 bytes.
    CompanyIdTooLong,
    /// The company identifier contained an unsupported character.
    InvalidCompanyIdCharacter,
    /// The individual identifier was empty after trimming whitespace.
    EmptyIndividualId,
    /// The individual identifier was longer than 15 bytes.
    IndividualIdTooLong,
    /// The individual identifier contained an unsupported character.
    InvalidIndividualIdCharacter,
}

impl fmt::Display for AchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStandardEntryClass => {
                formatter.write_str("ACH standard entry class is unsupported")
            },
            Self::InvalidTransactionCode => {
                formatter.write_str("ACH transaction code is unsupported")
            },
            Self::InvalidAddendaIndicator => {
                formatter.write_str("ACH addenda indicator must be 0 or 1")
            },
            Self::InvalidTraceNumberLength => {
                formatter.write_str("ACH trace number must be exactly 15 digits")
            },
            Self::InvalidTraceNumberCharacter => {
                formatter.write_str("ACH trace number must contain only digits")
            },
            Self::EmptyCompanyId => formatter.write_str("ACH company identifier cannot be empty"),
            Self::CompanyIdTooLong => {
                formatter.write_str("ACH company identifier cannot exceed 10 bytes")
            },
            Self::InvalidCompanyIdCharacter => {
                formatter.write_str("ACH company identifier contains an unsupported character")
            },
            Self::EmptyIndividualId => {
                formatter.write_str("ACH individual identifier cannot be empty")
            },
            Self::IndividualIdTooLong => {
                formatter.write_str("ACH individual identifier cannot exceed 15 bytes")
            },
            Self::InvalidIndividualIdCharacter => {
                formatter.write_str("ACH individual identifier contains an unsupported character")
            },
        }
    }
}

impl Error for AchError {}

fn validate_identifier(
    value: &str,
    max_len: usize,
    empty_error: AchError,
    too_long_error: AchError,
    invalid_character_error: AchError,
) -> Result<String, AchError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(empty_error);
    }

    if value.len() > max_len {
        return Err(too_long_error);
    }

    if !value.bytes().all(is_identifier_byte) {
        return Err(invalid_character_error);
    }

    Ok(value.to_owned())
}

const fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.')
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use super::{
        AchAccountType, AchAddendaIndicator, AchCompanyId, AchEntry, AchEntryDirection, AchError,
        AchIndividualId, AchStandardEntryClass, AchTraceNumber, AchTransactionCode,
    };

    #[test]
    fn parses_and_displays_standard_entry_classes() -> Result<(), AchError> {
        assert_eq!(
            AchStandardEntryClass::from_code("ppd")?,
            AchStandardEntryClass::Ppd
        );
        assert_eq!(AchStandardEntryClass::Web.as_str(), "WEB");
        assert_eq!(AchStandardEntryClass::Ccd.to_string(), "CCD");
        assert_eq!(
            AchStandardEntryClass::from_code("XYZ"),
            Err(AchError::InvalidStandardEntryClass)
        );
        Ok(())
    }

    #[test]
    fn exposes_transaction_code_behavior() -> Result<(), AchError> {
        let credit = AchTransactionCode::from_code(22)?;
        let debit = AchTransactionCode::from_str("38")?;

        assert_eq!(credit.code(), 22);
        assert_eq!(credit.account_type(), AchAccountType::Checking);
        assert_eq!(credit.direction(), AchEntryDirection::Credit);
        assert!(!credit.is_prenote());
        assert_eq!(debit.account_type(), AchAccountType::Savings);
        assert_eq!(debit.direction(), AchEntryDirection::Debit);
        assert!(debit.is_prenote());
        assert_eq!(
            AchTransactionCode::from_code(99),
            Err(AchError::InvalidTransactionCode)
        );
        Ok(())
    }

    #[test]
    fn validates_trace_numbers() -> Result<(), AchError> {
        let trace = AchTraceNumber::new("123456780000001")?;

        assert_eq!(trace.as_str(), "123456780000001");
        assert_eq!(trace.odfi_identification(), "12345678");
        assert_eq!(trace.sequence_number(), "0000001");
        assert_eq!(
            AchTraceNumber::new("12345678000001"),
            Err(AchError::InvalidTraceNumberLength)
        );
        assert_eq!(
            AchTraceNumber::new("12345678000000A"),
            Err(AchError::InvalidTraceNumberCharacter)
        );
        Ok(())
    }

    #[test]
    fn validates_identifiers() -> Result<(), AchError> {
        let company_id = AchCompanyId::new(" 1234567890 ")?;
        let individual_id = AchIndividualId::new("EMPLOYEE-001")?;

        assert_eq!(company_id.as_str(), "1234567890");
        assert_eq!(individual_id.as_str(), "EMPLOYEE-001");
        assert_eq!(AchCompanyId::new(""), Err(AchError::EmptyCompanyId));
        assert_eq!(
            AchCompanyId::new("12345678901"),
            Err(AchError::CompanyIdTooLong)
        );
        assert_eq!(
            AchIndividualId::new("employee 001"),
            Err(AchError::InvalidIndividualIdCharacter)
        );
        Ok(())
    }

    #[test]
    fn supports_addenda_indicator() -> Result<(), AchError> {
        assert_eq!(
            AchAddendaIndicator::from_code("0")?,
            AchAddendaIndicator::NoAddenda
        );
        assert_eq!(
            AchAddendaIndicator::from_code("1")?,
            AchAddendaIndicator::Addenda
        );
        assert_eq!(AchAddendaIndicator::Addenda.as_str(), "1");
        assert!(AchAddendaIndicator::Addenda.has_addenda());
        assert_eq!(
            AchAddendaIndicator::from_code("2"),
            Err(AchError::InvalidAddendaIndicator)
        );
        Ok(())
    }

    #[test]
    fn creates_entry_metadata() -> Result<(), AchError> {
        let entry = AchEntry::new(
            AchStandardEntryClass::Ppd,
            AchTransactionCode::CheckingCredit,
            AchTraceNumber::new("123456780000001")?,
            AchCompanyId::new("1234567890")?,
            AchIndividualId::new("EMPLOYEE001")?,
        )
        .with_addenda_indicator(AchAddendaIndicator::Addenda);

        assert_eq!(entry.standard_entry_class(), AchStandardEntryClass::Ppd);
        assert_eq!(entry.transaction_code(), AchTransactionCode::CheckingCredit);
        assert_eq!(entry.trace_number().as_str(), "123456780000001");
        assert_eq!(entry.company_id().as_str(), "1234567890");
        assert_eq!(entry.individual_id().as_str(), "EMPLOYEE001");
        assert_eq!(entry.addenda_indicator(), AchAddendaIndicator::Addenda);
        Ok(())
    }
}
