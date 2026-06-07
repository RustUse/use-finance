# use-transaction

Generic financial transaction primitives for `RustUse` finance crates.

`use-transaction` provides small identifiers, date wrappers, statuses, directions, and a generic amount-bearing transaction type. It does not model market trades.

## Example

```rust
use use_amount::Amount;
use use_transaction::{Transaction, TransactionDate, TransactionDirection, TransactionId};

let transaction = Transaction::new(
    TransactionId::new("txn-1001")?,
    Amount::from_minor_units(12_345, 2)?,
    TransactionDate::new("2026-06-07")?,
    TransactionDirection::Inflow,
);

assert_eq!(transaction.id().as_str(), "txn-1001");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for general financial transaction vocabulary. It does not represent market trades, orders, fills, exchange events, broker APIs, or market-data activity.

## License

Licensed under either MIT or Apache-2.0.
