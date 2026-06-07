#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;

const MAX_SCALE: u8 = 18;

/// Common scaled amount primitives.
pub mod prelude {
    pub use crate::{Amount, AmountError};
}

/// A decimal-safe amount represented as integer minor units and a decimal scale.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Amount {
    minor_units: i128,
    scale: u8,
}

impl Amount {
    /// Creates an amount from integer minor units and a decimal scale.
    ///
    /// # Errors
    ///
    /// Returns [`AmountError::ScaleTooLarge`] when `scale` is greater than 18.
    pub const fn from_minor_units(minor_units: i128, scale: u8) -> Result<Self, AmountError> {
        if scale > MAX_SCALE {
            return Err(AmountError::ScaleTooLarge);
        }

        Ok(Self { minor_units, scale })
    }

    /// Creates a zero amount at the requested scale.
    ///
    /// # Errors
    ///
    /// Returns [`AmountError::ScaleTooLarge`] when `scale` is greater than 18.
    pub const fn zero(scale: u8) -> Result<Self, AmountError> {
        Self::from_minor_units(0, scale)
    }

    /// Returns the integer minor-unit value.
    #[must_use]
    pub const fn minor_units(self) -> i128 {
        self.minor_units
    }

    /// Returns the decimal scale.
    #[must_use]
    pub const fn scale(self) -> u8 {
        self.scale
    }

    /// Returns whether this amount is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.minor_units == 0
    }

    /// Returns whether this amount is greater than zero.
    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.minor_units > 0
    }

    /// Returns whether this amount is less than zero.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.minor_units < 0
    }

    /// Returns the absolute value of this amount.
    ///
    /// # Errors
    ///
    /// Returns [`AmountError::Overflow`] when the absolute value cannot be represented.
    pub const fn checked_abs(self) -> Result<Self, AmountError> {
        match self.minor_units.checked_abs() {
            Some(minor_units) => Self::from_minor_units(minor_units, self.scale),
            None => Err(AmountError::Overflow),
        }
    }

    /// Adds two same-scale amounts.
    ///
    /// # Errors
    ///
    /// Returns [`AmountError::ScaleMismatch`] when scales differ and [`AmountError::Overflow`]
    /// when the integer addition overflows.
    pub fn checked_add(self, other: Self) -> Result<Self, AmountError> {
        self.ensure_same_scale(other)?;
        let minor_units = self
            .minor_units
            .checked_add(other.minor_units)
            .ok_or(AmountError::Overflow)?;
        Self::from_minor_units(minor_units, self.scale)
    }

    /// Subtracts two same-scale amounts.
    ///
    /// # Errors
    ///
    /// Returns [`AmountError::ScaleMismatch`] when scales differ and [`AmountError::Overflow`]
    /// when the integer subtraction overflows.
    pub fn checked_sub(self, other: Self) -> Result<Self, AmountError> {
        self.ensure_same_scale(other)?;
        let minor_units = self
            .minor_units
            .checked_sub(other.minor_units)
            .ok_or(AmountError::Overflow)?;
        Self::from_minor_units(minor_units, self.scale)
    }

    /// Rescales an amount without losing precision.
    ///
    /// # Errors
    ///
    /// Returns [`AmountError::ScaleTooLarge`] for unsupported scales,
    /// [`AmountError::Overflow`] when scaling up overflows, and
    /// [`AmountError::PrecisionLoss`] when scaling down would discard non-zero digits.
    pub fn checked_rescale(self, new_scale: u8) -> Result<Self, AmountError> {
        if new_scale > MAX_SCALE {
            return Err(AmountError::ScaleTooLarge);
        }

        if new_scale == self.scale {
            return Ok(self);
        }

        if new_scale > self.scale {
            let multiplier = pow10(new_scale - self.scale)?;
            let minor_units = self
                .minor_units
                .checked_mul(multiplier)
                .ok_or(AmountError::Overflow)?;
            return Self::from_minor_units(minor_units, new_scale);
        }

        let divisor = pow10(self.scale - new_scale)?;
        if self.minor_units % divisor != 0 {
            return Err(AmountError::PrecisionLoss);
        }

        Self::from_minor_units(self.minor_units / divisor, new_scale)
    }

    /// Removes trailing decimal zeroes from the minor-unit representation.
    #[must_use]
    pub const fn normalize(self) -> Self {
        let mut minor_units = self.minor_units;
        let mut scale = self.scale;

        while scale > 0 && minor_units % 10 == 0 {
            minor_units /= 10;
            scale -= 1;
        }

        Self { minor_units, scale }
    }

    const fn ensure_same_scale(self, other: Self) -> Result<(), AmountError> {
        if self.scale == other.scale {
            Ok(())
        } else {
            Err(AmountError::ScaleMismatch {
                left: self.scale,
                right: other.scale,
            })
        }
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.scale == 0 {
            return write!(formatter, "{}", self.minor_units);
        }

        let negative = self.minor_units < 0;
        let absolute = self.minor_units.unsigned_abs();
        let divisor = 10_u128.pow(u32::from(self.scale));
        let whole = absolute / divisor;
        let fraction = absolute % divisor;

        if negative {
            write!(formatter, "-")?;
        }

        write!(
            formatter,
            "{}.{:0width$}",
            whole,
            fraction,
            width = usize::from(self.scale)
        )
    }
}

/// Errors returned by amount helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AmountError {
    /// Decimal scales above 18 are intentionally unsupported.
    ScaleTooLarge,
    /// Arithmetic requires matching scales.
    ScaleMismatch {
        /// Left-hand amount scale.
        left: u8,
        /// Right-hand amount scale.
        right: u8,
    },
    /// Integer arithmetic overflowed.
    Overflow,
    /// Rescaling would discard non-zero minor units.
    PrecisionLoss,
}

impl fmt::Display for AmountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScaleTooLarge => formatter.write_str("amount scale cannot exceed 18"),
            Self::ScaleMismatch { left, right } => write!(
                formatter,
                "amount scales must match, got {left} and {right}"
            ),
            Self::Overflow => formatter.write_str("amount arithmetic overflowed"),
            Self::PrecisionLoss => formatter.write_str("amount rescale would lose precision"),
        }
    }
}

impl Error for AmountError {}

fn pow10(exponent: u8) -> Result<i128, AmountError> {
    10_i128
        .checked_pow(u32::from(exponent))
        .ok_or(AmountError::Overflow)
}

#[cfg(test)]
mod tests {
    use super::{Amount, AmountError};

    #[test]
    fn formats_scaled_amounts() -> Result<(), AmountError> {
        assert_eq!(Amount::from_minor_units(12_345, 2)?.to_string(), "123.45");
        assert_eq!(Amount::from_minor_units(-5, 2)?.to_string(), "-0.05");
        assert_eq!(Amount::from_minor_units(42, 0)?.to_string(), "42");
        Ok(())
    }

    #[test]
    fn adds_and_subtracts_same_scale_amounts() -> Result<(), AmountError> {
        let left = Amount::from_minor_units(10_000, 2)?;
        let right = Amount::from_minor_units(2_500, 2)?;

        assert_eq!(left.checked_add(right)?.minor_units(), 12_500);
        assert_eq!(left.checked_sub(right)?.minor_units(), 7_500);
        Ok(())
    }

    #[test]
    fn rejects_mismatched_scales() -> Result<(), AmountError> {
        let left = Amount::from_minor_units(100, 2)?;
        let right = Amount::from_minor_units(100, 3)?;

        assert_eq!(
            left.checked_add(right),
            Err(AmountError::ScaleMismatch { left: 2, right: 3 })
        );
        Ok(())
    }

    #[test]
    fn rescales_without_precision_loss() -> Result<(), AmountError> {
        let amount = Amount::from_minor_units(123, 2)?;
        assert_eq!(amount.checked_rescale(4)?.minor_units(), 12_300);
        assert_eq!(
            Amount::from_minor_units(12_300, 4)?.checked_rescale(2)?,
            amount
        );
        assert_eq!(amount.checked_rescale(1), Err(AmountError::PrecisionLoss));
        Ok(())
    }

    #[test]
    fn normalizes_trailing_zeroes() -> Result<(), AmountError> {
        assert_eq!(
            Amount::from_minor_units(12_300, 4)?.normalize(),
            Amount::from_minor_units(123, 2)?
        );
        assert_eq!(
            Amount::from_minor_units(0, 4)?.normalize(),
            Amount::from_minor_units(0, 0)?
        );
        Ok(())
    }
}
