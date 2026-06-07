#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::{error::Error, slice};

use use_money::{Money, MoneyError};

/// Common invoice primitives.
pub mod prelude {
    pub use crate::{
        BalanceDue, DueDate, Invoice, InvoiceError, InvoiceLine, InvoiceNumber, InvoiceStatus,
        Subtotal, Total,
    };
}

/// A non-empty invoice number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InvoiceNumber(String);

impl InvoiceNumber {
    /// Creates an invoice number from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`InvoiceError::EmptyInvoiceNumber`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, InvoiceError> {
        non_empty(value, InvoiceError::EmptyInvoiceNumber).map(Self)
    }

    /// Returns the invoice number.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for InvoiceNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for InvoiceNumber {
    type Err = InvoiceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A due date in `YYYY-MM-DD` shape.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DueDate(String);

impl DueDate {
    /// Creates a due date from `YYYY-MM-DD` shaped text.
    ///
    /// # Errors
    ///
    /// Returns [`InvoiceError::InvalidDueDate`] when the input is not in `YYYY-MM-DD` shape.
    pub fn new(value: impl AsRef<str>) -> Result<Self, InvoiceError> {
        let value = value.as_ref().trim();
        let bytes = value.as_bytes();
        if bytes.len() == 10
            && bytes[4] == b'-'
            && bytes[7] == b'-'
            && bytes[..4].iter().all(u8::is_ascii_digit)
            && bytes[5..7].iter().all(u8::is_ascii_digit)
            && bytes[8..].iter().all(u8::is_ascii_digit)
        {
            Ok(Self(value.to_string()))
        } else {
            Err(InvoiceError::InvalidDueDate)
        }
    }

    /// Returns the due date.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Invoice lifecycle status.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InvoiceStatus {
    /// Draft invoice.
    Draft,
    /// Open invoice.
    Open,
    /// Partially paid invoice.
    PartiallyPaid,
    /// Paid invoice.
    Paid,
    /// Void invoice.
    Void,
}

/// A single invoice line.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvoiceLine {
    description: String,
    amount: Money,
}

impl InvoiceLine {
    /// Creates an invoice line with a non-empty description.
    ///
    /// # Errors
    ///
    /// Returns [`InvoiceError::EmptyLineDescription`] when the trimmed description is empty.
    pub fn new(description: impl AsRef<str>, amount: Money) -> Result<Self, InvoiceError> {
        Ok(Self {
            description: non_empty(description, InvoiceError::EmptyLineDescription)?,
            amount,
        })
    }

    /// Returns the line description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the line amount.
    #[must_use]
    pub const fn amount(&self) -> &Money {
        &self.amount
    }
}

/// Invoice subtotal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subtotal(Money);

impl Subtotal {
    /// Creates a subtotal.
    #[must_use]
    pub const fn new(amount: Money) -> Self {
        Self(amount)
    }

    /// Returns the subtotal amount.
    #[must_use]
    pub const fn amount(&self) -> &Money {
        &self.0
    }
}

/// Invoice total.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Total(Money);

impl Total {
    /// Creates a total.
    #[must_use]
    pub const fn new(amount: Money) -> Self {
        Self(amount)
    }

    /// Returns the total amount.
    #[must_use]
    pub const fn amount(&self) -> &Money {
        &self.0
    }
}

/// Invoice balance due.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BalanceDue(Money);

impl BalanceDue {
    /// Creates a balance due.
    #[must_use]
    pub const fn new(amount: Money) -> Self {
        Self(amount)
    }

    /// Returns the balance-due amount.
    #[must_use]
    pub const fn amount(&self) -> &Money {
        &self.0
    }
}

/// A general invoice with same-currency line totals.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Invoice {
    number: InvoiceNumber,
    status: InvoiceStatus,
    due_date: Option<DueDate>,
    lines: Vec<InvoiceLine>,
    subtotal: Subtotal,
    total: Total,
    balance_due: BalanceDue,
}

impl Invoice {
    /// Creates an open invoice from same-currency lines.
    ///
    /// # Errors
    ///
    /// Returns [`InvoiceError::NoLines`] when no lines are supplied and [`InvoiceError::Money`]
    /// when line totals cannot be added.
    pub fn from_lines(
        number: InvoiceNumber,
        lines: Vec<InvoiceLine>,
    ) -> Result<Self, InvoiceError> {
        Self::new(number, InvoiceStatus::Open, None, lines)
    }

    /// Creates an invoice from same-currency lines.
    ///
    /// # Errors
    ///
    /// Returns [`InvoiceError::NoLines`] when no lines are supplied and [`InvoiceError::Money`]
    /// when line totals cannot be added.
    pub fn new(
        number: InvoiceNumber,
        status: InvoiceStatus,
        due_date: Option<DueDate>,
        lines: Vec<InvoiceLine>,
    ) -> Result<Self, InvoiceError> {
        let subtotal = sum_lines(&lines)?;
        Ok(Self {
            number,
            status,
            due_date,
            lines,
            subtotal: Subtotal::new(subtotal.clone()),
            total: Total::new(subtotal.clone()),
            balance_due: BalanceDue::new(subtotal),
        })
    }

    /// Returns a copy of this invoice with a due date.
    #[must_use]
    pub fn with_due_date(mut self, due_date: DueDate) -> Self {
        self.due_date = Some(due_date);
        self
    }

    /// Returns a copy of this invoice with an amount paid applied to the balance due.
    ///
    /// # Errors
    ///
    /// Returns [`InvoiceError::Money`] when the payment currency or amount scale is incompatible.
    pub fn with_amount_paid(mut self, amount_paid: &Money) -> Result<Self, InvoiceError> {
        self.balance_due = BalanceDue::new(
            self.total
                .amount()
                .checked_sub(amount_paid)
                .map_err(InvoiceError::Money)?,
        );
        self.status = if self.balance_due.amount().is_zero() {
            InvoiceStatus::Paid
        } else {
            InvoiceStatus::PartiallyPaid
        };
        Ok(self)
    }

    /// Returns the invoice number.
    #[must_use]
    pub const fn number(&self) -> &InvoiceNumber {
        &self.number
    }

    /// Returns the invoice status.
    #[must_use]
    pub const fn status(&self) -> InvoiceStatus {
        self.status
    }

    /// Returns the optional due date.
    #[must_use]
    pub const fn due_date(&self) -> Option<&DueDate> {
        self.due_date.as_ref()
    }

    /// Returns the invoice lines.
    #[must_use]
    pub fn lines(&self) -> &[InvoiceLine] {
        &self.lines
    }

    /// Iterates over invoice lines.
    pub fn iter(&self) -> slice::Iter<'_, InvoiceLine> {
        self.lines.iter()
    }

    /// Returns the subtotal.
    #[must_use]
    pub const fn subtotal(&self) -> &Subtotal {
        &self.subtotal
    }

    /// Returns the total.
    #[must_use]
    pub const fn total(&self) -> &Total {
        &self.total
    }

    /// Returns the balance due.
    #[must_use]
    pub const fn balance_due(&self) -> &BalanceDue {
        &self.balance_due
    }
}

impl<'a> IntoIterator for &'a Invoice {
    type Item = &'a InvoiceLine;
    type IntoIter = slice::Iter<'a, InvoiceLine>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Errors returned by invoice primitives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvoiceError {
    /// Invoice number must not be empty.
    EmptyInvoiceNumber,
    /// Line description must not be empty.
    EmptyLineDescription,
    /// Due date must use `YYYY-MM-DD` shape.
    InvalidDueDate,
    /// Invoices require at least one line.
    NoLines,
    /// Money arithmetic failed.
    Money(MoneyError),
}

impl fmt::Display for InvoiceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInvoiceNumber => formatter.write_str("invoice number cannot be empty"),
            Self::EmptyLineDescription => {
                formatter.write_str("invoice line description cannot be empty")
            },
            Self::InvalidDueDate => formatter.write_str("due date must use YYYY-MM-DD shape"),
            Self::NoLines => formatter.write_str("invoice requires at least one line"),
            Self::Money(error) => error.fmt(formatter),
        }
    }
}

impl Error for InvoiceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Money(error) => Some(error),
            Self::EmptyInvoiceNumber
            | Self::EmptyLineDescription
            | Self::InvalidDueDate
            | Self::NoLines => None,
        }
    }
}

fn non_empty(value: impl AsRef<str>, error: InvoiceError) -> Result<String, InvoiceError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

fn sum_lines(lines: &[InvoiceLine]) -> Result<Money, InvoiceError> {
    let Some(first) = lines.first() else {
        return Err(InvoiceError::NoLines);
    };

    let mut total = first.amount().clone();
    for line in &lines[1..] {
        total = total
            .checked_add(line.amount())
            .map_err(InvoiceError::Money)?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use use_amount::Amount;
    use use_currency::CurrencyCode;
    use use_money::Money;

    use super::{DueDate, Invoice, InvoiceError, InvoiceLine, InvoiceNumber, InvoiceStatus};

    fn money(code: &str, minor_units: i128) -> Result<Money, Box<dyn std::error::Error>> {
        Ok(Money::new(
            Amount::from_minor_units(minor_units, 2)?,
            CurrencyCode::new(code)?,
        ))
    }

    #[test]
    fn totals_invoice_lines() -> Result<(), Box<dyn std::error::Error>> {
        let invoice = Invoice::from_lines(
            InvoiceNumber::new("inv-1001")?,
            vec![
                InvoiceLine::new("consulting", money("USD", 20_000)?)?,
                InvoiceLine::new("support", money("USD", 5_000)?)?,
            ],
        )?
        .with_due_date(DueDate::new("2026-07-01")?)
        .with_amount_paid(&money("USD", 10_000)?)?;

        assert_eq!(invoice.status(), InvoiceStatus::PartiallyPaid);
        assert_eq!(invoice.total().amount().amount().minor_units(), 25_000);
        assert_eq!(
            invoice.balance_due().amount().amount().minor_units(),
            15_000
        );
        assert_eq!(invoice.due_date().map(DueDate::as_str), Some("2026-07-01"));
        Ok(())
    }

    #[test]
    fn rejects_empty_lines_and_mixed_currencies() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            Invoice::from_lines(InvoiceNumber::new("inv-empty")?, Vec::new()),
            Err(InvoiceError::NoLines)
        );

        let invoice = Invoice::from_lines(
            InvoiceNumber::new("inv-mixed")?,
            vec![
                InvoiceLine::new("usd", money("USD", 100)?)?,
                InvoiceLine::new("eur", money("EUR", 100)?)?,
            ],
        );
        assert!(matches!(invoice, Err(InvoiceError::Money(_))));
        Ok(())
    }
}
