#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_amount::Amount;

/// Common transaction primitives.
pub mod prelude {
    pub use crate::{
        EffectiveDate, PostedDate, Transaction, TransactionDate, TransactionDirection,
        TransactionError, TransactionId, TransactionStatus,
    };
}

/// A non-empty transaction identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransactionId(String);

impl TransactionId {
    /// Creates a transaction identifier from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`TransactionError::EmptyIdentifier`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TransactionError> {
        non_empty_text(value, TransactionError::EmptyIdentifier).map(Self)
    }

    /// Returns the transaction identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TransactionId {
    type Err = TransactionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A transaction date in `YYYY-MM-DD` shape.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransactionDate(String);

impl TransactionDate {
    /// Creates a transaction date from `YYYY-MM-DD` shaped text.
    ///
    /// # Errors
    ///
    /// Returns [`TransactionError::InvalidDate`] when the input is not in `YYYY-MM-DD` shape.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TransactionError> {
        iso_date_text(value).map(Self)
    }

    /// Returns the transaction date.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TransactionDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A posted date in `YYYY-MM-DD` shape.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PostedDate(String);

impl PostedDate {
    /// Creates a posted date from `YYYY-MM-DD` shaped text.
    ///
    /// # Errors
    ///
    /// Returns [`TransactionError::InvalidDate`] when the input is not in `YYYY-MM-DD` shape.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TransactionError> {
        iso_date_text(value).map(Self)
    }

    /// Returns the posted date.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An effective date in `YYYY-MM-DD` shape.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectiveDate(String);

impl EffectiveDate {
    /// Creates an effective date from `YYYY-MM-DD` shaped text.
    ///
    /// # Errors
    ///
    /// Returns [`TransactionError::InvalidDate`] when the input is not in `YYYY-MM-DD` shape.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TransactionError> {
        iso_date_text(value).map(Self)
    }

    /// Returns the effective date.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Generic transaction status vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TransactionStatus {
    /// Transaction has been recorded but not posted.
    Pending,
    /// Transaction has posted.
    Posted,
    /// Transaction has settled.
    Settled,
    /// Transaction was voided.
    Voided,
    /// Transaction was reversed.
    Reversed,
}

/// Generic transaction direction vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TransactionDirection {
    /// Money or value moving in.
    Inflow,
    /// Money or value moving out.
    Outflow,
}

/// A generic financial transaction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transaction {
    id: TransactionId,
    amount: Amount,
    date: TransactionDate,
    posted_date: Option<PostedDate>,
    effective_date: Option<EffectiveDate>,
    status: TransactionStatus,
    direction: TransactionDirection,
    description: Option<String>,
}

impl Transaction {
    /// Creates a pending transaction from required fields.
    #[must_use]
    pub const fn new(
        id: TransactionId,
        amount: Amount,
        transaction_date: TransactionDate,
        direction: TransactionDirection,
    ) -> Self {
        Self {
            id,
            amount,
            date: transaction_date,
            posted_date: None,
            effective_date: None,
            status: TransactionStatus::Pending,
            direction,
            description: None,
        }
    }

    /// Returns the transaction identifier.
    #[must_use]
    pub const fn id(&self) -> &TransactionId {
        &self.id
    }

    /// Returns the transaction amount.
    #[must_use]
    pub const fn amount(&self) -> Amount {
        self.amount
    }

    /// Returns the transaction date.
    #[must_use]
    pub const fn transaction_date(&self) -> &TransactionDate {
        &self.date
    }

    /// Returns the posted date.
    #[must_use]
    pub const fn posted_date(&self) -> Option<&PostedDate> {
        self.posted_date.as_ref()
    }

    /// Returns the effective date.
    #[must_use]
    pub const fn effective_date(&self) -> Option<&EffectiveDate> {
        self.effective_date.as_ref()
    }

    /// Returns the transaction status.
    #[must_use]
    pub const fn status(&self) -> TransactionStatus {
        self.status
    }

    /// Returns the transaction direction.
    #[must_use]
    pub const fn direction(&self) -> TransactionDirection {
        self.direction
    }

    /// Returns the optional transaction description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Sets the transaction status.
    #[must_use]
    pub const fn with_status(mut self, status: TransactionStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the posted date.
    #[must_use]
    pub fn with_posted_date(mut self, posted_date: PostedDate) -> Self {
        self.posted_date = Some(posted_date);
        self
    }

    /// Sets the effective date.
    #[must_use]
    pub fn with_effective_date(mut self, effective_date: EffectiveDate) -> Self {
        self.effective_date = Some(effective_date);
        self
    }

    /// Sets a non-empty transaction description.
    ///
    /// # Errors
    ///
    /// Returns [`TransactionError::EmptyDescription`] when the trimmed input is empty.
    pub fn with_description(
        mut self,
        description: impl AsRef<str>,
    ) -> Result<Self, TransactionError> {
        self.description = Some(non_empty_text(
            description,
            TransactionError::EmptyDescription,
        )?);
        Ok(self)
    }
}

/// Errors returned by transaction primitives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionError {
    /// The identifier was empty after trimming whitespace.
    EmptyIdentifier,
    /// The date was not in `YYYY-MM-DD` shape.
    InvalidDate,
    /// The description was empty after trimming whitespace.
    EmptyDescription,
}

impl fmt::Display for TransactionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => formatter.write_str("transaction identifier cannot be empty"),
            Self::InvalidDate => formatter.write_str("transaction date must use YYYY-MM-DD shape"),
            Self::EmptyDescription => {
                formatter.write_str("transaction description cannot be empty")
            },
        }
    }
}

impl Error for TransactionError {}

fn non_empty_text(
    value: impl AsRef<str>,
    error: TransactionError,
) -> Result<String, TransactionError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

fn iso_date_text(value: impl AsRef<str>) -> Result<String, TransactionError> {
    let trimmed = value.as_ref().trim();
    let bytes = trimmed.as_bytes();
    if bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[8..].iter().all(u8::is_ascii_digit)
    {
        Ok(trimmed.to_string())
    } else {
        Err(TransactionError::InvalidDate)
    }
}

#[cfg(test)]
mod tests {
    use use_amount::Amount;

    use super::{
        EffectiveDate, PostedDate, Transaction, TransactionDate, TransactionDirection,
        TransactionError, TransactionId, TransactionStatus,
    };

    #[test]
    fn creates_transaction() -> Result<(), Box<dyn std::error::Error>> {
        let transaction = Transaction::new(
            TransactionId::new("txn-1001")?,
            Amount::from_minor_units(12_345, 2)?,
            TransactionDate::new("2026-06-07")?,
            TransactionDirection::Inflow,
        )
        .with_status(TransactionStatus::Posted)
        .with_posted_date(PostedDate::new("2026-06-08")?)
        .with_effective_date(EffectiveDate::new("2026-06-07")?)
        .with_description("customer payment")?;

        assert_eq!(transaction.id().as_str(), "txn-1001");
        assert_eq!(transaction.status(), TransactionStatus::Posted);
        assert_eq!(transaction.description(), Some("customer payment"));
        Ok(())
    }

    #[test]
    fn rejects_empty_identifier_and_bad_date() {
        assert_eq!(
            TransactionId::new(""),
            Err(TransactionError::EmptyIdentifier)
        );
        assert_eq!(
            TransactionDate::new("06/07/2026"),
            Err(TransactionError::InvalidDate)
        );
    }
}
