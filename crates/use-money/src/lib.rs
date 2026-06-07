#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;

use use_amount::{Amount, AmountError};
use use_currency::CurrencyCode;

/// Common money primitives.
pub mod prelude {
    pub use crate::{CurrencyMismatch, Money, MoneyError};
}

/// A currency-safe money value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Money {
    amount: Amount,
    currency: CurrencyCode,
}

impl Money {
    /// Creates money from an amount and currency.
    #[must_use]
    pub const fn new(amount: Amount, currency: CurrencyCode) -> Self {
        Self { amount, currency }
    }

    /// Returns the amount.
    #[must_use]
    pub const fn amount(&self) -> Amount {
        self.amount
    }

    /// Returns the currency.
    #[must_use]
    pub const fn currency(&self) -> &CurrencyCode {
        &self.currency
    }

    /// Returns whether the amount is zero.
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }

    /// Adds money values when their currencies match.
    ///
    /// # Errors
    ///
    /// Returns [`MoneyError::CurrencyMismatch`] when currencies differ and
    /// [`MoneyError::Amount`] when amount addition fails.
    pub fn checked_add(&self, other: &Self) -> Result<Self, MoneyError> {
        self.ensure_same_currency(other)?;
        Ok(Self::new(
            self.amount
                .checked_add(other.amount)
                .map_err(MoneyError::Amount)?,
            self.currency.clone(),
        ))
    }

    /// Subtracts money values when their currencies match.
    ///
    /// # Errors
    ///
    /// Returns [`MoneyError::CurrencyMismatch`] when currencies differ and
    /// [`MoneyError::Amount`] when amount subtraction fails.
    pub fn checked_sub(&self, other: &Self) -> Result<Self, MoneyError> {
        self.ensure_same_currency(other)?;
        Ok(Self::new(
            self.amount
                .checked_sub(other.amount)
                .map_err(MoneyError::Amount)?,
            self.currency.clone(),
        ))
    }

    fn ensure_same_currency(&self, other: &Self) -> Result<(), MoneyError> {
        if self.currency == other.currency {
            Ok(())
        } else {
            Err(MoneyError::CurrencyMismatch(CurrencyMismatch {
                expected: self.currency.clone(),
                actual: other.currency.clone(),
            }))
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.amount, self.currency)
    }
}

/// A same-currency operation received different currencies.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyMismatch {
    expected: CurrencyCode,
    actual: CurrencyCode,
}

impl CurrencyMismatch {
    /// Returns the expected currency.
    #[must_use]
    pub const fn expected(&self) -> &CurrencyCode {
        &self.expected
    }

    /// Returns the actual currency.
    #[must_use]
    pub const fn actual(&self) -> &CurrencyCode {
        &self.actual
    }
}

impl fmt::Display for CurrencyMismatch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "currency mismatch: expected {}, got {}",
            self.expected, self.actual
        )
    }
}

impl Error for CurrencyMismatch {}

/// Errors returned by money helpers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MoneyError {
    /// A same-currency operation received different currencies.
    CurrencyMismatch(CurrencyMismatch),
    /// Amount arithmetic failed.
    Amount(AmountError),
}

impl fmt::Display for MoneyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrencyMismatch(error) => error.fmt(formatter),
            Self::Amount(error) => error.fmt(formatter),
        }
    }
}

impl Error for MoneyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CurrencyMismatch(error) => Some(error),
            Self::Amount(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use use_amount::Amount;
    use use_currency::CurrencyCode;

    use super::{Money, MoneyError};

    #[test]
    fn adds_and_subtracts_same_currency_money() -> Result<(), Box<dyn std::error::Error>> {
        let usd = CurrencyCode::new("USD")?;
        let left = Money::new(Amount::from_minor_units(10_000, 2)?, usd.clone());
        let right = Money::new(Amount::from_minor_units(2_500, 2)?, usd);

        assert_eq!(left.checked_add(&right)?.amount().minor_units(), 12_500);
        assert_eq!(left.checked_sub(&right)?.amount().minor_units(), 7_500);
        assert_eq!(left.to_string(), "100.00 USD");
        Ok(())
    }

    #[test]
    fn rejects_currency_mismatch() -> Result<(), Box<dyn std::error::Error>> {
        let usd = Money::new(Amount::from_minor_units(100, 2)?, CurrencyCode::new("USD")?);
        let eur = Money::new(Amount::from_minor_units(100, 2)?, CurrencyCode::new("EUR")?);

        let error = usd.checked_add(&eur).expect_err("currencies should differ");
        assert!(matches!(error, MoneyError::CurrencyMismatch(_)));
        Ok(())
    }
}
