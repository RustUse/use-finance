#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common IBAN primitives.
pub mod prelude {
    pub use crate::{Iban, IbanError};
}

const MAX_IBAN_LENGTH: usize = 34;

const IBAN_COUNTRY_LENGTHS: &[(&str, usize)] = &[
    ("AD", 24),
    ("AE", 23),
    ("AL", 28),
    ("AT", 20),
    ("AZ", 28),
    ("BA", 20),
    ("BE", 16),
    ("BG", 22),
    ("BH", 22),
    ("BI", 16),
    ("BR", 29),
    ("BY", 28),
    ("CH", 21),
    ("CR", 22),
    ("CY", 28),
    ("CZ", 24),
    ("DE", 22),
    ("DK", 18),
    ("DO", 28),
    ("EE", 20),
    ("EG", 29),
    ("ES", 24),
    ("FI", 18),
    ("FO", 18),
    ("FR", 27),
    ("GB", 22),
    ("GE", 22),
    ("GI", 23),
    ("GL", 18),
    ("GR", 27),
    ("GT", 28),
    ("HR", 21),
    ("HU", 28),
    ("IE", 22),
    ("IL", 23),
    ("IQ", 23),
    ("IS", 26),
    ("IT", 27),
    ("JO", 30),
    ("KW", 30),
    ("KZ", 20),
    ("LB", 28),
    ("LC", 32),
    ("LI", 21),
    ("LT", 20),
    ("LU", 20),
    ("LV", 21),
    ("LY", 25),
    ("MC", 27),
    ("MD", 24),
    ("ME", 22),
    ("MK", 19),
    ("MR", 27),
    ("MT", 31),
    ("MU", 30),
    ("NL", 18),
    ("NO", 15),
    ("PK", 24),
    ("PL", 28),
    ("PS", 29),
    ("PT", 25),
    ("QA", 29),
    ("RO", 24),
    ("RS", 22),
    ("SA", 24),
    ("SC", 31),
    ("SE", 24),
    ("SI", 19),
    ("SK", 24),
    ("SM", 27),
    ("SO", 23),
    ("ST", 25),
    ("SV", 28),
    ("TL", 23),
    ("TN", 24),
    ("TR", 26),
    ("UA", 29),
    ("VA", 22),
    ("VG", 24),
    ("XK", 20),
];

/// A validated International Bank Account Number in compact uppercase form.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Iban(String);

impl Iban {
    /// Creates an IBAN after normalization, length validation, and mod-97 validation.
    ///
    /// # Errors
    ///
    /// Returns an [`IbanError`] when the input is empty, malformed, uses an unsupported country
    /// code, has the wrong country-specific length, or fails the mod-97 checksum.
    pub fn new(value: impl AsRef<str>) -> Result<Self, IbanError> {
        let compact = compact_iban(value.as_ref())?;

        if compact.len() < 4 || compact.len() > MAX_IBAN_LENGTH {
            return Err(IbanError::InvalidLength);
        }

        if !compact.as_bytes()[0].is_ascii_uppercase()
            || !compact.as_bytes()[1].is_ascii_uppercase()
        {
            return Err(IbanError::InvalidCountryCode);
        }

        if !compact.as_bytes()[2].is_ascii_digit() || !compact.as_bytes()[3].is_ascii_digit() {
            return Err(IbanError::InvalidCheckDigits);
        }

        let expected_length =
            country_length(&compact[..2]).ok_or(IbanError::UnsupportedCountryCode)?;
        if compact.len() != expected_length {
            return Err(IbanError::InvalidCountryLength);
        }

        if !has_valid_checksum(&compact) {
            return Err(IbanError::InvalidChecksum);
        }

        Ok(Self(compact))
    }

    /// Returns the compact uppercase IBAN.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the compact uppercase IBAN.
    #[must_use]
    pub fn compact(&self) -> &str {
        self.as_str()
    }

    /// Returns the IBAN grouped with spaces every four characters.
    #[must_use]
    pub fn format_grouped(&self) -> String {
        let space_count = self.0.len().saturating_sub(1) / 4;
        let mut grouped = String::with_capacity(self.0.len() + space_count);

        for (index, byte) in self.0.bytes().enumerate() {
            if index > 0 && index % 4 == 0 {
                grouped.push(' ');
            }
            grouped.push(char::from(byte));
        }

        grouped
    }

    /// Returns the two-letter country code.
    #[must_use]
    pub fn country_code(&self) -> &str {
        &self.0[..2]
    }

    /// Returns the two numeric check digits.
    #[must_use]
    pub fn check_digits(&self) -> &str {
        &self.0[2..4]
    }

    /// Returns the country-specific basic bank account number portion.
    #[must_use]
    pub fn bban(&self) -> &str {
        &self.0[4..]
    }
}

impl AsRef<str> for Iban {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Iban {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Iban {
    type Err = IbanError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Iban {
    type Error = IbanError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Errors returned while constructing IBAN values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IbanError {
    /// The input was empty after trimming whitespace and spaces.
    Empty,
    /// The compact IBAN was shorter than four characters or longer than 34 characters.
    InvalidLength,
    /// The country code was not two uppercase ASCII letters.
    InvalidCountryCode,
    /// The check digits were not two ASCII digits.
    InvalidCheckDigits,
    /// The IBAN contained a character other than an ASCII letter, digit, or space.
    InvalidCharacter,
    /// The country code is not present in the static IBAN length table.
    UnsupportedCountryCode,
    /// The compact IBAN length did not match the country-specific IBAN length.
    InvalidCountryLength,
    /// The standard mod-97 checksum failed.
    InvalidChecksum,
}

impl fmt::Display for IbanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("IBAN cannot be empty"),
            Self::InvalidLength => formatter.write_str("IBAN length is invalid"),
            Self::InvalidCountryCode => {
                formatter.write_str("IBAN country code must be two letters")
            },
            Self::InvalidCheckDigits => formatter.write_str("IBAN check digits must be two digits"),
            Self::InvalidCharacter => {
                formatter.write_str("IBAN must contain only ASCII letters, digits, or spaces")
            },
            Self::UnsupportedCountryCode => formatter.write_str("IBAN country code is unsupported"),
            Self::InvalidCountryLength => {
                formatter.write_str("IBAN length does not match the country-specific length")
            },
            Self::InvalidChecksum => formatter.write_str("IBAN mod-97 checksum is invalid"),
        }
    }
}

impl Error for IbanError {}

fn compact_iban(value: &str) -> Result<String, IbanError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(IbanError::Empty);
    }

    let mut compact = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b' ' => {},
            b'a'..=b'z' => compact.push(char::from(byte.to_ascii_uppercase())),
            b'A'..=b'Z' | b'0'..=b'9' => compact.push(char::from(byte)),
            _ => return Err(IbanError::InvalidCharacter),
        }
    }

    if compact.is_empty() {
        return Err(IbanError::Empty);
    }

    Ok(compact)
}

fn country_length(country_code: &str) -> Option<usize> {
    IBAN_COUNTRY_LENGTHS
        .iter()
        .find_map(|(country, length)| (*country == country_code).then_some(*length))
}

fn has_valid_checksum(value: &str) -> bool {
    let rearranged = value[4..].bytes().chain(value[..4].bytes());
    let mut remainder = 0_u32;

    for byte in rearranged {
        if byte.is_ascii_digit() {
            remainder = ((remainder * 10) + u32::from(byte - b'0')) % 97;
        } else if byte.is_ascii_uppercase() {
            let letter_value = u32::from(byte - b'A') + 10;
            remainder = ((remainder * 10) + (letter_value / 10)) % 97;
            remainder = ((remainder * 10) + (letter_value % 10)) % 97;
        } else {
            return false;
        }
    }

    remainder == 1
}

#[cfg(test)]
mod tests {
    use super::{Iban, IbanError};

    #[test]
    fn accepts_valid_ibans() -> Result<(), IbanError> {
        let cases = [
            ("GB82 WEST 1234 5698 7654 32", "GB82WEST12345698765432"),
            ("DE89 3704 0044 0532 0130 00", "DE89370400440532013000"),
            (
                "FR14 2004 1010 0505 0001 3M02 606",
                "FR1420041010050500013M02606",
            ),
        ];

        for (input, compact) in cases {
            let iban = Iban::new(input)?;
            assert_eq!(iban.as_str(), compact);
            assert_eq!(iban.compact(), compact);
        }

        Ok(())
    }

    #[test]
    fn normalizes_lowercase_and_formats_groups() -> Result<(), IbanError> {
        let iban = Iban::new("gb82 west 1234 5698 7654 32")?;

        assert_eq!(iban.as_str(), "GB82WEST12345698765432");
        assert_eq!(iban.format_grouped(), "GB82 WEST 1234 5698 7654 32");
        assert_eq!(iban.country_code(), "GB");
        assert_eq!(iban.check_digits(), "82");
        assert_eq!(iban.bban(), "WEST12345698765432");
        Ok(())
    }

    #[test]
    fn rejects_mod97_failures() {
        assert_eq!(
            Iban::new("GB82 WEST 1234 5698 7654 33"),
            Err(IbanError::InvalidChecksum)
        );
    }

    #[test]
    fn rejects_invalid_characters_and_country_parts() {
        assert_eq!(Iban::new(""), Err(IbanError::Empty));
        assert_eq!(
            Iban::new("1B82WEST12345698765432"),
            Err(IbanError::InvalidCountryCode)
        );
        assert_eq!(
            Iban::new("GBXXWEST12345698765432"),
            Err(IbanError::InvalidCheckDigits)
        );
        assert_eq!(
            Iban::new("GB82-WEST-1234"),
            Err(IbanError::InvalidCharacter)
        );
    }

    #[test]
    fn rejects_unsupported_or_wrong_country_lengths() {
        assert_eq!(
            Iban::new("US82WEST12345698765432"),
            Err(IbanError::UnsupportedCountryCode)
        );
        assert_eq!(
            Iban::new("DE8937040044053201300"),
            Err(IbanError::InvalidCountryLength)
        );
    }
}
