#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common BIC primitives.
pub mod prelude {
    pub use crate::{Bic, BicError};
}

/// A validated SWIFT/BIC-style bank identifier code.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bic(String);

impl Bic {
    /// Creates a BIC after uppercase normalization and position-specific validation.
    ///
    /// # Errors
    ///
    /// Returns [`BicError::InvalidLength`] when the trimmed input is not 8 or 11 bytes,
    /// [`BicError::InvalidBankCode`] when the first four characters are not letters,
    /// [`BicError::InvalidCountryCode`] when the country code is not two letters,
    /// [`BicError::InvalidLocationCode`] when the location code is not alphanumeric, and
    /// [`BicError::InvalidBranchCode`] when the optional branch code is not alphanumeric.
    pub fn new(value: impl AsRef<str>) -> Result<Self, BicError> {
        let value = value.as_ref().trim();
        if value.len() != 8 && value.len() != 11 {
            return Err(BicError::InvalidLength);
        }

        let mut normalized = String::with_capacity(value.len());
        for (index, byte) in value.bytes().enumerate() {
            let uppercase = byte.to_ascii_uppercase();
            match index {
                0..=3 if !uppercase.is_ascii_uppercase() => {
                    return Err(BicError::InvalidBankCode);
                },
                4..=5 if !uppercase.is_ascii_uppercase() => {
                    return Err(BicError::InvalidCountryCode);
                },
                6..=7 if !uppercase.is_ascii_alphanumeric() => {
                    return Err(BicError::InvalidLocationCode);
                },
                8..=10 if !uppercase.is_ascii_alphanumeric() => {
                    return Err(BicError::InvalidBranchCode);
                },
                _ => normalized.push(char::from(uppercase)),
            }
        }

        Ok(Self(normalized))
    }

    /// Returns the normalized BIC.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the four-letter bank code.
    #[must_use]
    pub fn bank_code(&self) -> &str {
        &self.0[..4]
    }

    /// Returns the two-letter country code.
    #[must_use]
    pub fn country_code(&self) -> &str {
        &self.0[4..6]
    }

    /// Returns the two-character location code.
    #[must_use]
    pub fn location_code(&self) -> &str {
        &self.0[6..8]
    }

    /// Returns the optional three-character branch code.
    #[must_use]
    pub fn branch_code(&self) -> Option<&str> {
        (self.0.len() == 11).then(|| &self.0[8..11])
    }

    /// Returns whether the BIC identifies a primary office.
    #[must_use]
    pub fn is_primary_office(&self) -> bool {
        matches!(self.branch_code(), None | Some("XXX"))
    }
}

impl AsRef<str> for Bic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Bic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Bic {
    type Err = BicError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Bic {
    type Error = BicError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Errors returned while constructing BIC values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BicError {
    /// BIC values must be exactly 8 or 11 characters.
    InvalidLength,
    /// The bank code must be four letters.
    InvalidBankCode,
    /// The country code must be two letters.
    InvalidCountryCode,
    /// The location code must be two alphanumeric characters.
    InvalidLocationCode,
    /// The branch code must be three alphanumeric characters when present.
    InvalidBranchCode,
}

impl fmt::Display for BicError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => formatter.write_str("BIC must be exactly 8 or 11 characters"),
            Self::InvalidBankCode => formatter.write_str("BIC bank code must be four letters"),
            Self::InvalidCountryCode => formatter.write_str("BIC country code must be two letters"),
            Self::InvalidLocationCode => {
                formatter.write_str("BIC location code must be two alphanumeric characters")
            },
            Self::InvalidBranchCode => {
                formatter.write_str("BIC branch code must be three alphanumeric characters")
            },
        }
    }
}

impl Error for BicError {}

#[cfg(test)]
mod tests {
    use super::{Bic, BicError};

    #[test]
    fn accepts_valid_8_character_bic() -> Result<(), BicError> {
        let bic = Bic::new("DEUTDEFF")?;

        assert_eq!(bic.as_str(), "DEUTDEFF");
        assert_eq!(bic.bank_code(), "DEUT");
        assert_eq!(bic.country_code(), "DE");
        assert_eq!(bic.location_code(), "FF");
        assert_eq!(bic.branch_code(), None);
        assert!(bic.is_primary_office());
        Ok(())
    }

    #[test]
    fn accepts_valid_11_character_bic() -> Result<(), BicError> {
        let bic = Bic::new("deutdeff500")?;

        assert_eq!(bic.as_str(), "DEUTDEFF500");
        assert_eq!(bic.branch_code(), Some("500"));
        assert!(!bic.is_primary_office());
        Ok(())
    }

    #[test]
    fn treats_xxx_branch_as_primary_office() -> Result<(), BicError> {
        let bic = Bic::new("NEDSZAJJXXX")?;

        assert_eq!(bic.branch_code(), Some("XXX"));
        assert!(bic.is_primary_office());
        Ok(())
    }

    #[test]
    fn rejects_invalid_lengths() {
        assert_eq!(Bic::new("DEUTDEF"), Err(BicError::InvalidLength));
        assert_eq!(Bic::new("DEUTDEFF5000"), Err(BicError::InvalidLength));
    }

    #[test]
    fn rejects_invalid_character_positions() {
        assert_eq!(Bic::new("D3UTDEFF"), Err(BicError::InvalidBankCode));
        assert_eq!(Bic::new("DEUTD3FF"), Err(BicError::InvalidCountryCode));
        assert_eq!(Bic::new("DEUTDE@F"), Err(BicError::InvalidLocationCode));
        assert_eq!(Bic::new("DEUTDEFF50@"), Err(BicError::InvalidBranchCode));
    }
}
