#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_money::Money;

/// Common payment primitives.
pub mod prelude {
    pub use crate::{
        Payment, PaymentDirection, PaymentError, PaymentMethod, PaymentReference, PaymentStatus,
    };
}

/// A non-empty payment reference.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PaymentReference(String);

impl PaymentReference {
    /// Creates a payment reference from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`PaymentError::EmptyReference`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PaymentError> {
        let value = value.as_ref().trim();
        if value.is_empty() {
            return Err(PaymentError::EmptyReference);
        }

        Ok(Self(value.to_string()))
    }

    /// Returns the payment reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PaymentReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PaymentReference {
    type Err = PaymentError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Conservative payment method vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PaymentMethod {
    /// Automated Clearing House payment.
    Ach,
    /// Wire transfer.
    Wire,
    /// Card payment.
    Card,
    /// Check payment.
    Check,
    /// Cash payment.
    Cash,
    /// Bank transfer.
    BankTransfer,
    /// Other payment method.
    Other,
}

/// Payment lifecycle status.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PaymentStatus {
    /// Payment has been created but not completed.
    Pending,
    /// Payment is processing.
    Processing,
    /// Payment has completed.
    Completed,
    /// Payment failed.
    Failed,
    /// Payment was canceled.
    Canceled,
    /// Payment was returned.
    Returned,
}

/// Payment direction.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PaymentDirection {
    /// Incoming payment.
    Inbound,
    /// Outgoing payment.
    Outbound,
}

/// A payment value with reference, amount, method, direction, and status.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Payment {
    reference: PaymentReference,
    amount: Money,
    method: PaymentMethod,
    direction: PaymentDirection,
    status: PaymentStatus,
}

impl Payment {
    /// Creates a pending payment.
    #[must_use]
    pub const fn new(
        reference: PaymentReference,
        amount: Money,
        method: PaymentMethod,
        direction: PaymentDirection,
    ) -> Self {
        Self {
            reference,
            amount,
            method,
            direction,
            status: PaymentStatus::Pending,
        }
    }

    /// Returns the payment reference.
    #[must_use]
    pub const fn reference(&self) -> &PaymentReference {
        &self.reference
    }

    /// Returns the payment amount.
    #[must_use]
    pub const fn amount(&self) -> &Money {
        &self.amount
    }

    /// Returns the payment method.
    #[must_use]
    pub const fn method(&self) -> PaymentMethod {
        self.method
    }

    /// Returns the payment direction.
    #[must_use]
    pub const fn direction(&self) -> PaymentDirection {
        self.direction
    }

    /// Returns the payment status.
    #[must_use]
    pub const fn status(&self) -> PaymentStatus {
        self.status
    }

    /// Sets the payment status.
    #[must_use]
    pub const fn with_status(mut self, status: PaymentStatus) -> Self {
        self.status = status;
        self
    }
}

/// Errors returned by payment primitives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaymentError {
    /// The reference was empty after trimming whitespace.
    EmptyReference,
}

impl fmt::Display for PaymentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyReference => formatter.write_str("payment reference cannot be empty"),
        }
    }
}

impl Error for PaymentError {}

#[cfg(test)]
mod tests {
    use use_amount::Amount;
    use use_currency::CurrencyCode;
    use use_money::Money;

    use super::{
        Payment, PaymentDirection, PaymentError, PaymentMethod, PaymentReference, PaymentStatus,
    };

    #[test]
    fn creates_payment() -> Result<(), Box<dyn std::error::Error>> {
        let payment = Payment::new(
            PaymentReference::new("pay-1001")?,
            Money::new(
                Amount::from_minor_units(12_345, 2)?,
                CurrencyCode::new("USD")?,
            ),
            PaymentMethod::Ach,
            PaymentDirection::Inbound,
        )
        .with_status(PaymentStatus::Completed);

        assert_eq!(payment.reference().as_str(), "pay-1001");
        assert_eq!(payment.status(), PaymentStatus::Completed);
        assert_eq!(payment.method(), PaymentMethod::Ach);
        Ok(())
    }

    #[test]
    fn rejects_empty_reference() {
        assert_eq!(PaymentReference::new(""), Err(PaymentError::EmptyReference));
    }
}
