# use-amount

Decimal-safe scaled integer amount primitives for `RustUse` finance crates.

`use-amount` stores financial amounts as integer minor units plus a decimal scale. It avoids `f32` and `f64` entirely.

## Example

```rust
use use_amount::Amount;

let invoice = Amount::from_minor_units(12_345, 2)?;
let payment = Amount::from_minor_units(2_345, 2)?;
let balance = invoice.checked_sub(payment)?;

assert_eq!(invoice.to_string(), "123.45");
assert_eq!(balance.minor_units(), 10_000);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for scaled integer amount values, checked same-scale arithmetic, sign checks, rescaling, normalization, and formatting. It does not model currencies, money, exchange rates, taxes, or rounding policy engines.

## License

Licensed under either MIT or Apache-2.0.
