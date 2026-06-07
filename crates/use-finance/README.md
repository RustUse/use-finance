# use-finance

Facade crate for `RustUse` practical finance primitives.

`use-finance` re-exports focused crates for money, currencies, amounts, ledgers, transactions, payments, receipts, invoices, bank accounts, routing numbers, IBANs, BICs, ACH metadata, reconciliation, and BAI2 banking files.

## Example

```rust
use use_finance::{ach, amount, bic, currency, iban, money, routing_number};

let usd = currency::CurrencyCode::new("USD")?;
let total = money::Money::new(amount::Amount::from_minor_units(12_345, 2)?, usd);
let routing = routing_number::RoutingNumber::new("021000021")?;
let iban = iban::Iban::new("GB82 WEST 1234 5698 7654 32")?;
let bic = bic::Bic::new("DEUTDEFF500")?;
let ach_entry = ach::AchEntry::new(
	ach::AchStandardEntryClass::Ppd,
	ach::AchTransactionCode::CheckingCredit,
	ach::AchTraceNumber::new("123456780000001")?,
	ach::AchCompanyId::new("1234567890")?,
	ach::AchIndividualId::new("EMPLOYEE001")?,
);

assert_eq!(total.amount().minor_units(), 12_345);
assert_eq!(routing.as_str(), "021000021");
assert_eq!(iban.country_code(), "GB");
assert_eq!(bic.country_code(), "DE");
assert_eq!(ach_entry.transaction_code().code(), 22);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this facade for practical finance primitives. Use `use-quant` for bars, ticks, market prices, price series, returns, drawdowns, volatility, risk, factors, portfolio weights, and signals.

This facade stays offline and deterministic. It does not perform live bank lookups, payment transmission, API calls, database access, or market-data analysis.

## License

Licensed under either MIT or Apache-2.0.
