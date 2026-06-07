# use-finance

RustUse is "Composable sets of primitive Rust utility crates for fellow crustaceans."

`use-finance` is a practical financial primitive set. It provides small, composable Rust primitives for money, currencies, amounts, ledgers, transactions, payments, receipts, invoices, bank accounts, routing numbers, IBANs, BICs, ACH metadata, reconciliation, and BAI2 banking file records.

`use-finance` is not a bank integration, payment processor, tax engine, accounting platform, financial advisor, exchange-rate service, market data provider, trading system, or database layer.

## Boundary

Use `use-finance` for practical finance and accounting primitives: money, invoices, payments, receipts, ledgers, bank accounts, routing numbers, IBANs, BICs, conservative ACH metadata, reconciliation, and BAI2 banking files.

Use `use-quant` for quantitative finance and market-analysis primitives: bars, ticks, market prices, price series, returns, drawdowns, volatility, risk, factors, portfolio weights, and signals.

This workspace is intentionally offline and deterministic. It does not fetch exchange rates, connect to banks, call external APIs, run async runtimes, query databases, or model market trades.

## Crates

| Crate                | Purpose                                                                                      |
| -------------------- | -------------------------------------------------------------------------------------------- |
| `use-finance`        | Thin facade over the focused practical finance crates.                                       |
| `use-currency`       | Uppercase 3-letter currency code primitives.                                                 |
| `use-amount`         | Decimal-safe scaled integer amounts without floating point.                                  |
| `use-money`          | Amounts paired with currencies and same-currency arithmetic.                                 |
| `use-ledger`         | Debit, credit, posting, journal entry, ledger entry, and balance primitives.                 |
| `use-transaction`    | Generic financial transaction identifiers, dates, statuses, and directions.                  |
| `use-payment`        | Payment references, methods, statuses, directions, and amounts.                              |
| `use-receipt`        | Receipt numbers, receipt statuses, received timestamps, and applied amounts.                 |
| `use-invoice`        | Invoice numbers, lines, statuses, due dates, totals, and balances due.                       |
| `use-bank-account`   | Bank account numbers, masks, account types, and holder names.                                |
| `use-routing-number` | ABA routing number validation and formatting.                                                |
| `use-iban`           | IBAN validation, compact normalization, and grouped formatting.                              |
| `use-bic`            | SWIFT/BIC-style bank identifier code validation.                                             |
| `use-ach`            | Conservative ACH/NACHA-oriented enums, identifiers, and entry metadata.                      |
| `use-reconciliation` | Deterministic reconciliation candidates, scores, results, and exceptions.                    |
| `use-bai2`           | Initial BAI2 record parsing, validation, continuation handling, and normalized transactions. |

## Example

```rust
use use_finance::{ach, amount, bai2, bic, currency, iban, money, reconciliation, routing_number};

let usd = currency::CurrencyCode::new("USD")?;
let cents = amount::Amount::from_minor_units(12_345, 2)?;
let invoice_total = money::Money::new(cents, usd.clone());

let routing = routing_number::RoutingNumber::new("021000021")?;
assert_eq!(routing.as_str(), "021000021");

let iban = iban::Iban::new("GB82 WEST 1234 5698 7654 32")?;
let bic = bic::Bic::new("DEUTDEFF500")?;
let ach_entry = ach::AchEntry::new(
    ach::AchStandardEntryClass::Ppd,
    ach::AchTransactionCode::CheckingCredit,
    ach::AchTraceNumber::new("123456780000001")?,
    ach::AchCompanyId::new("1234567890")?,
    ach::AchIndividualId::new("EMPLOYEE001")?,
);

let records = bai2::parse_logical_records("16,475,12345,Z,bank-ref,customer-ref,invoice payment/\n")?;
let detail = bai2::TransactionDetailRecord::try_from(&records[0])?;
let normalized = bai2::NormalizedTransaction::from_detail(&detail)?;

let candidate = reconciliation::ReconciliationCandidate::new(
    "bank-ref",
    "invoice-1001",
    amount::Amount::zero(2)?,
    reconciliation::MatchScore::exact(),
)?;

assert_eq!(invoice_total.currency().as_str(), "USD");
assert_eq!(iban.country_code(), "GB");
assert_eq!(bic.country_code(), "DE");
assert_eq!(ach_entry.transaction_code().direction(), ach::AchEntryDirection::Credit);
assert_eq!(normalized.amount().minor_units(), 12_345);
assert_eq!(candidate.score(), reconciliation::MatchScore::exact());
# Ok::<(), Box<dyn std::error::Error>>(())
```

The example composes primitives that downstream crates can store, compare, serialize, or transform. It does not perform live reconciliation, fetch bank data, call payment APIs, or infer matches with machine learning.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
