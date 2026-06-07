#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_money::{Money, MoneyError};

/// Common receipt primitives.
pub mod prelude {
    pub use crate::{
        AppliedAmount, Receipt, ReceiptError, ReceiptNumber, ReceiptStatus, ReceivedAt,
        UnappliedAmount,
    };
}

/// A non-empty receipt number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReceiptNumber(String);

impl ReceiptNumber {
    /// Creates a receipt number from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ReceiptError::EmptyReceiptNumber`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ReceiptError> {
        non_empty(value, ReceiptError::EmptyReceiptNumber).map(Self)
    }

    /// Returns the receipt number.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ReceiptNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ReceiptNumber {
    type Err = ReceiptError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A received timestamp stored as non-empty caller-provided text.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReceivedAt(String);

impl ReceivedAt {
    /// Creates a received timestamp from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ReceiptError::EmptyReceivedAt`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ReceiptError> {
        non_empty(value, ReceiptError::EmptyReceivedAt).map(Self)
    }

    /// Returns the received timestamp text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Receipt lifecycle status.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReceiptStatus {
    /// Receipt has been recorded.
    Received,
    /// Receipt has been partially applied.
    PartiallyApplied,
    /// Receipt has been fully applied.
    Applied,
    /// Receipt was voided.
    Voided,
}

/// A receipt amount applied to open items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppliedAmount(Money);

impl AppliedAmount {
    /// Creates an applied amount.
    #[must_use]
    pub const fn new(amount: Money) -> Self {
        Self(amount)
    }

    /// Returns the applied money amount.
    #[must_use]
    pub const fn amount(&self) -> &Money {
        &self.0
    }
}

/// A receipt amount not yet applied to open items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnappliedAmount(Money);

impl UnappliedAmount {
    /// Creates an unapplied amount.
    #[must_use]
    pub const fn new(amount: Money) -> Self {
        Self(amount)
    }

    /// Returns the unapplied money amount.
    #[must_use]
    pub const fn amount(&self) -> &Money {
        &self.0
    }
}

/// A receipt with applied and unapplied amounts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Receipt {
    number: ReceiptNumber,
    received_at: ReceivedAt,
    applied_amount: AppliedAmount,
    unapplied_amount: UnappliedAmount,
    status: ReceiptStatus,
}

impl Receipt {
    /// Creates a receipt and validates that applied and unapplied amounts use the same currency.
    ///
    /// # Errors
    ///
    /// Returns [`ReceiptError::Money`] when applied and unapplied amounts cannot be added.
    pub fn new(
        number: ReceiptNumber,
        received_at: ReceivedAt,
        applied_amount: AppliedAmount,
        unapplied_amount: UnappliedAmount,
    ) -> Result<Self, ReceiptError> {
        applied_amount
            .amount()
            .checked_add(unapplied_amount.amount())
            .map_err(ReceiptError::Money)?;

        let status = if unapplied_amount.amount().is_zero() {
            ReceiptStatus::Applied
        } else if applied_amount.amount().is_zero() {
            ReceiptStatus::Received
        } else {
            ReceiptStatus::PartiallyApplied
        };

        Ok(Self {
            number,
            received_at,
            applied_amount,
            unapplied_amount,
            status,
        })
    }

    /// Returns the receipt number.
    #[must_use]
    pub const fn number(&self) -> &ReceiptNumber {
        &self.number
    }

    /// Returns the received timestamp.
    #[must_use]
    pub const fn received_at(&self) -> &ReceivedAt {
        &self.received_at
    }

    /// Returns the applied amount.
    #[must_use]
    pub const fn applied_amount(&self) -> &AppliedAmount {
        &self.applied_amount
    }

    /// Returns the unapplied amount.
    #[must_use]
    pub const fn unapplied_amount(&self) -> &UnappliedAmount {
        &self.unapplied_amount
    }

    /// Returns the receipt status.
    #[must_use]
    pub const fn status(&self) -> ReceiptStatus {
        self.status
    }

    /// Returns the total received amount.
    ///
    /// # Errors
    ///
    /// Returns [`ReceiptError::Money`] when applied and unapplied amounts cannot be added.
    pub fn total_received(&self) -> Result<Money, ReceiptError> {
        self.applied_amount
            .amount()
            .checked_add(self.unapplied_amount.amount())
            .map_err(ReceiptError::Money)
    }
}

/// Errors returned by receipt primitives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReceiptError {
    /// Receipt number must not be empty.
    EmptyReceiptNumber,
    /// Received timestamp must not be empty.
    EmptyReceivedAt,
    /// Money arithmetic failed.
    Money(MoneyError),
}

impl fmt::Display for ReceiptError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyReceiptNumber => formatter.write_str("receipt number cannot be empty"),
            Self::EmptyReceivedAt => formatter.write_str("received timestamp cannot be empty"),
            Self::Money(error) => error.fmt(formatter),
        }
    }
}

impl Error for ReceiptError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Money(error) => Some(error),
            Self::EmptyReceiptNumber | Self::EmptyReceivedAt => None,
        }
    }
}

fn non_empty(value: impl AsRef<str>, error: ReceiptError) -> Result<String, ReceiptError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use use_amount::Amount;
    use use_currency::CurrencyCode;
    use use_money::Money;

    use super::{
        AppliedAmount, Receipt, ReceiptError, ReceiptNumber, ReceiptStatus, ReceivedAt,
        UnappliedAmount,
    };

    fn money(code: &str, minor_units: i128) -> Result<Money, Box<dyn std::error::Error>> {
        Ok(Money::new(
            Amount::from_minor_units(minor_units, 2)?,
            CurrencyCode::new(code)?,
        ))
    }

    #[test]
    fn creates_applied_receipt() -> Result<(), Box<dyn std::error::Error>> {
        let receipt = Receipt::new(
            ReceiptNumber::new("rcpt-1001")?,
            ReceivedAt::new("2026-06-07T10:00:00Z")?,
            AppliedAmount::new(money("USD", 10_000)?),
            UnappliedAmount::new(money("USD", 0)?),
        )?;

        assert_eq!(receipt.status(), ReceiptStatus::Applied);
        assert_eq!(receipt.total_received()?.amount().minor_units(), 10_000);
        Ok(())
    }

    #[test]
    fn rejects_currency_mismatch() -> Result<(), Box<dyn std::error::Error>> {
        let receipt = Receipt::new(
            ReceiptNumber::new("rcpt-1002")?,
            ReceivedAt::new("2026-06-07T10:00:00Z")?,
            AppliedAmount::new(money("USD", 10_000)?),
            UnappliedAmount::new(money("EUR", 100)?),
        );

        assert!(matches!(receipt, Err(ReceiptError::Money(_))));
        Ok(())
    }
}
