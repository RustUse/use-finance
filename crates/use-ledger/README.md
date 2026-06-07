# use-ledger

Basic ledger and accounting primitives for `RustUse` finance crates.

`use-ledger` models debit/credit postings, balanced journal entries, ledger entries, and account balances without tying the types to any accounting platform.

## Example

```rust
use use_amount::Amount;
use use_currency::CurrencyCode;
use use_ledger::{DebitCredit, JournalEntry, Posting};
use use_money::Money;

let usd = CurrencyCode::new("USD")?;
let amount = Money::new(Amount::from_minor_units(5_000, 2)?, usd);
let entry = JournalEntry::new(
    "je-1001",
    vec![
        Posting::new("cash", amount.clone(), DebitCredit::Debit)?,
        Posting::new("revenue", amount, DebitCredit::Credit)?,
    ],
)?;

assert!(entry.is_balanced());
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for balanced-entry validation and small accounting vocabulary. It does not implement a general ledger database, accounting rules engine, chart-of-accounts framework, tax engine, or reporting platform.

## License

Licensed under either MIT or Apache-2.0.
