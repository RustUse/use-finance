# use-money

Money primitives pairing amounts and currencies for `RustUse` finance crates.

`use-money` combines a scaled integer `Amount` with a validated `CurrencyCode` and provides checked arithmetic only when currencies match.

## Example

```rust
use use_amount::Amount;
use use_currency::CurrencyCode;
use use_money::Money;

let usd = CurrencyCode::new("USD")?;
let left = Money::new(Amount::from_minor_units(10_000, 2)?, usd.clone());
let right = Money::new(Amount::from_minor_units(2_500, 2)?, usd);

assert_eq!(left.checked_sub(&right)?.amount().minor_units(), 7_500);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for currency-safe money values and checked same-currency addition/subtraction. It does not provide exchange rates, rounding policy engines, tax calculations, live financial data, or formatting by locale.

## License

Licensed under either MIT or Apache-2.0.
