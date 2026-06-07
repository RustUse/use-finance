# use-finance

Facade crate for `RustUse` practical finance primitives.

`use-finance` re-exports focused crates for money, currencies, amounts, ledgers, transactions, payments, receipts, invoices, bank accounts, routing numbers, reconciliation, and BAI2 banking files.

## Example

```rust
use use_finance::{amount, currency, money, routing_number};

let usd = currency::CurrencyCode::new("USD")?;
let total = money::Money::new(amount::Amount::from_minor_units(12_345, 2)?, usd);
let routing = routing_number::RoutingNumber::new("021000021")?;

assert_eq!(total.amount().minor_units(), 12_345);
assert_eq!(routing.as_str(), "021000021");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this facade for practical finance primitives. Use `use-quant` for bars, ticks, market prices, price series, returns, drawdowns, volatility, risk, factors, portfolio weights, and signals.

## License

Licensed under either MIT or Apache-2.0.
