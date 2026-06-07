# use-currency

Lightweight currency code primitives for `RustUse` finance crates.

`use-currency` validates uppercase 3-letter alphabetic currency code identifiers without fetching exchange rates or currency metadata.

## Example

```rust
use use_currency::{CurrencyCode, USD};

let currency = CurrencyCode::new(USD)?;

assert_eq!(currency.as_str(), "USD");
assert!(CurrencyCode::new("usd").is_err());
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for small validated currency identifiers. It does not provide exchange rates, currency metadata downloads, locale formatting, decimal arithmetic, or money values.

## License

Licensed under either MIT or Apache-2.0.
