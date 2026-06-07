#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin facade for `RustUse` practical finance primitive crates.
//!
//! `use-finance` describes money, currency, amount, ledger, transaction, payment, receipt,
//! invoice, bank-account, routing-number, reconciliation, and BAI2 vocabulary. It is not a
//! trading system, exchange-rate service, bank integration, payment processor, tax engine,
//! accounting platform, or market-data provider.

pub use use_amount as amount;
pub use use_bai2 as bai2;
pub use use_bank_account as bank_account;
pub use use_currency as currency;
pub use use_invoice as invoice;
pub use use_ledger as ledger;
pub use use_money as money;
pub use use_payment as payment;
pub use use_receipt as receipt;
pub use use_reconciliation as reconciliation;
pub use use_routing_number as routing_number;
pub use use_transaction as transaction;

/// Common practical finance primitive types from the focused crates.
pub mod prelude {
    pub use crate::amount::{Amount, AmountError};
    pub use crate::bai2::{
        AccountIdentifierRecord, Bai2Error, FileHeaderRecord, FileTrailerRecord, FundsTypeCode,
        NormalizedTransaction, RawRecord, RecordCode, TransactionDetailRecord, TransactionTypeCode,
    };
    pub use crate::bank_account::{
        AccountHolderName, AccountNumber, AccountType, BankAccount, MaskedAccountNumber,
    };
    pub use crate::currency::{AUD, CAD, CurrencyCode, CurrencyCodeError, EUR, GBP, JPY, USD};
    pub use crate::invoice::{
        BalanceDue, DueDate, Invoice, InvoiceLine, InvoiceNumber, InvoiceStatus, Subtotal, Total,
    };
    pub use crate::ledger::{Balance, DebitCredit, JournalEntry, LedgerEntry, Posting};
    pub use crate::money::{CurrencyMismatch, Money, MoneyError};
    pub use crate::payment::{
        Payment, PaymentDirection, PaymentMethod, PaymentReference, PaymentStatus,
    };
    pub use crate::receipt::{
        AppliedAmount, Receipt, ReceiptNumber, ReceiptStatus, ReceivedAt, UnappliedAmount,
    };
    pub use crate::reconciliation::{
        ExceptionReason, MatchConfidence, MatchScore, MatchStatus, ReconciliationCandidate,
        ReconciliationResult,
    };
    pub use crate::routing_number::{RoutingNumber, RoutingNumberError};
    pub use crate::transaction::{
        EffectiveDate, PostedDate, Transaction, TransactionDate, TransactionDirection,
        TransactionId, TransactionStatus,
    };
}

#[cfg(test)]
mod tests {
    use super::{amount, bai2, currency, money, reconciliation, routing_number};

    #[test]
    fn facade_exposes_composable_finance_primitives() -> Result<(), Box<dyn std::error::Error>> {
        let usd = currency::CurrencyCode::new("USD")?;
        let cents = amount::Amount::from_minor_units(12_345, 2)?;
        let invoice_total = money::Money::new(cents, usd);

        let routing = routing_number::RoutingNumber::new("021000021")?;
        assert_eq!(routing.as_str(), "021000021");

        let records =
            bai2::parse_logical_records("16,475,12345,Z,bank-ref,customer-ref,invoice payment/\n")?;
        let detail = bai2::TransactionDetailRecord::try_from(&records[0])?;
        let normalized = bai2::NormalizedTransaction::from_detail(&detail)?;

        let candidate = reconciliation::ReconciliationCandidate::new(
            "bank-ref",
            "invoice-1001",
            amount::Amount::zero(2)?,
            reconciliation::MatchScore::exact(),
        )?;

        assert_eq!(invoice_total.currency().as_str(), "USD");
        assert_eq!(normalized.amount().minor_units(), 12_345);
        assert_eq!(candidate.score(), reconciliation::MatchScore::exact());
        Ok(())
    }
}
