# use-payment

Payment primitives for `RustUse` finance crates.

`use-payment` provides payment references, methods, directions, statuses, and money amounts without implementing payment network protocols.

## Example

```rust
use use_amount::Amount;
use use_currency::CurrencyCode;
use use_money::Money;
use use_payment::{Payment, PaymentDirection, PaymentMethod, PaymentReference};

let payment = Payment::new(
    PaymentReference::new("pay-1001")?,
    Money::new(Amount::from_minor_units(12_345, 2)?, CurrencyCode::new("USD")?),
    PaymentMethod::Ach,
    PaymentDirection::Inbound,
);

assert_eq!(payment.reference().as_str(), "pay-1001");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for payment vocabulary and local state. It does not implement ACH files, wire formats, card processing, payment gateways, bank APIs, async runtimes, or settlement systems.

## License

Licensed under either MIT or Apache-2.0.
