#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common ABA routing number primitives.
pub mod prelude {
    pub use crate::{RoutingNumber, RoutingNumberError};
}

/// A validated 9-digit ABA routing number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RoutingNumber(String);

impl RoutingNumber {
    /// Creates a routing number after shape and checksum validation.
    ///
    /// # Errors
    ///
    /// Returns [`RoutingNumberError::InvalidLength`] when the trimmed input is not nine bytes,
    /// [`RoutingNumberError::NotDigits`] when any byte is not a digit, and
    /// [`RoutingNumberError::InvalidChecksum`] when the ABA checksum fails.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RoutingNumberError> {
        let value = value.as_ref().trim();
        if value.len() != 9 {
            return Err(RoutingNumberError::InvalidLength);
        }

        if !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(RoutingNumberError::NotDigits);
        }

        if !has_valid_checksum(value) {
            return Err(RoutingNumberError::InvalidChecksum);
        }

        Ok(Self(value.to_string()))
    }

    /// Returns the validated routing number.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the routing number and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for RoutingNumber {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RoutingNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RoutingNumber {
    type Err = RoutingNumberError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for RoutingNumber {
    type Error = RoutingNumberError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Errors returned while constructing routing numbers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoutingNumberError {
    /// ABA routing numbers must be exactly nine digits.
    InvalidLength,
    /// ABA routing numbers must contain only digits.
    NotDigits,
    /// The ABA routing checksum failed.
    InvalidChecksum,
}

impl fmt::Display for RoutingNumberError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => formatter.write_str("routing number must be exactly 9 digits"),
            Self::NotDigits => formatter.write_str("routing number must contain only digits"),
            Self::InvalidChecksum => formatter.write_str("routing number checksum is invalid"),
        }
    }
}

impl Error for RoutingNumberError {}

fn has_valid_checksum(value: &str) -> bool {
    let mut digits = [0_u32; 9];
    for (index, byte) in value.bytes().enumerate() {
        digits[index] = u32::from(byte - b'0');
    }

    let checksum = 3 * (digits[0] + digits[3] + digits[6])
        + 7 * (digits[1] + digits[4] + digits[7])
        + digits[2]
        + digits[5]
        + digits[8];

    checksum % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::{RoutingNumber, RoutingNumberError};

    #[test]
    fn accepts_valid_routing_numbers() -> Result<(), RoutingNumberError> {
        for value in ["021000021", "011000015", "121000248"] {
            let routing = RoutingNumber::new(value)?;
            assert_eq!(routing.as_str(), value);
        }
        Ok(())
    }

    #[test]
    fn rejects_bad_checksum() {
        assert_eq!(
            RoutingNumber::new("021000022"),
            Err(RoutingNumberError::InvalidChecksum)
        );
    }

    #[test]
    fn rejects_non_digits_and_bad_lengths() {
        assert_eq!(
            RoutingNumber::new("02100002"),
            Err(RoutingNumberError::InvalidLength)
        );
        assert_eq!(
            RoutingNumber::new("02100002A"),
            Err(RoutingNumberError::NotDigits)
        );
    }
}
