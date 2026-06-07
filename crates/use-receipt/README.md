# use-receipt

Receipt primitives for `RustUse` finance crates.

`use-receipt` provides receipt numbers, statuses, received timestamps, applied amounts, unapplied amounts, and a receipt total helper for cash-application workflows.

## Example

```rust
use use_amount::Amount;
use use_currency::CurrencyCode;
use use_money::Money;
use use_receipt::{AppliedAmount, Receipt, ReceiptNumber, ReceivedAt, UnappliedAmount};

let usd = CurrencyCode::new("USD")?;
let receipt = Receipt::new(
    ReceiptNumber::new("rcpt-1001")?,
    ReceivedAt::new("2026-06-07T10:00:00Z")?,
    AppliedAmount::new(Money::new(Amount::from_minor_units(10_000, 2)?, usd.clone())),
    UnappliedAmount::new(Money::new(Amount::zero(2)?, usd)),
)?;

assert_eq!(receipt.total_received()?.amount().minor_units(), 10_000);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for receipt vocabulary and simple applied/unapplied amount handling. It does not connect to payment processors, allocate cash automatically, or implement an accounting platform.

## License

Licensed under either MIT or Apache-2.0.
