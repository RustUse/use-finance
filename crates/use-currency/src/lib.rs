#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// United States dollar currency code.
pub const USD: &str = "USD";
/// Euro currency code.
pub const EUR: &str = "EUR";
/// British pound sterling currency code.
pub const GBP: &str = "GBP";
/// Canadian dollar currency code.
pub const CAD: &str = "CAD";
/// Australian dollar currency code.
pub const AUD: &str = "AUD";
/// Japanese yen currency code.
pub const JPY: &str = "JPY";

/// Common currency code primitives.
pub mod prelude {
    pub use crate::{AUD, CAD, CurrencyCode, CurrencyCodeError, EUR, GBP, JPY, USD};
}

/// A validated uppercase 3-letter alphabetic currency code.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CurrencyCode(String);

impl CurrencyCode {
    /// Creates a currency code from an uppercase 3-letter ASCII alphabetic value.
    ///
    /// # Errors
    ///
    /// Returns [`CurrencyCodeError::InvalidLength`] when the input is not exactly three bytes,
    /// [`CurrencyCodeError::NotAlphabetic`] when a byte is not alphabetic, and
    /// [`CurrencyCodeError::NotUppercase`] when an alphabetic byte is lowercase.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CurrencyCodeError> {
        let value = value.as_ref();
        if value.len() != 3 {
            return Err(CurrencyCodeError::InvalidLength);
        }

        if !value.bytes().all(|byte| byte.is_ascii_alphabetic()) {
            return Err(CurrencyCodeError::NotAlphabetic);
        }

        if !value.bytes().all(|byte| byte.is_ascii_uppercase()) {
            return Err(CurrencyCodeError::NotUppercase);
        }

        Ok(Self(value.to_string()))
    }

    /// Returns the validated currency code.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the code and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for CurrencyCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CurrencyCode {
    type Err = CurrencyCodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for CurrencyCode {
    type Error = CurrencyCodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Errors returned while constructing currency codes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CurrencyCodeError {
    /// Currency codes must be exactly three bytes long.
    InvalidLength,
    /// Currency codes must contain only ASCII alphabetic bytes.
    NotAlphabetic,
    /// Currency codes must be uppercase.
    NotUppercase,
}

impl fmt::Display for CurrencyCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => formatter.write_str("currency code must be exactly 3 letters"),
            Self::NotAlphabetic => {
                formatter.write_str("currency code must contain only ASCII letters")
            },
            Self::NotUppercase => formatter.write_str("currency code must be uppercase"),
        }
    }
}

impl Error for CurrencyCodeError {}

#[cfg(test)]
mod tests {
    use super::{AUD, CAD, CurrencyCode, CurrencyCodeError, EUR, GBP, JPY, USD};

    #[test]
    fn accepts_common_uppercase_codes() -> Result<(), CurrencyCodeError> {
        for code in [USD, EUR, GBP, CAD, AUD, JPY] {
            let currency = CurrencyCode::new(code)?;
            assert_eq!(currency.as_str(), code);
            assert_eq!(currency.to_string(), code);
        }
        Ok(())
    }

    #[test]
    fn rejects_lowercase_codes() {
        assert_eq!(
            CurrencyCode::new("usd"),
            Err(CurrencyCodeError::NotUppercase)
        );
    }

    #[test]
    fn rejects_invalid_shapes() {
        assert_eq!(
            CurrencyCode::new("US"),
            Err(CurrencyCodeError::InvalidLength)
        );
        assert_eq!(
            CurrencyCode::new("USDA"),
            Err(CurrencyCodeError::InvalidLength)
        );
        assert_eq!(
            CurrencyCode::new("U1D"),
            Err(CurrencyCodeError::NotAlphabetic)
        );
    }
}
