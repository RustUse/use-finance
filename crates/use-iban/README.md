# use-iban

International Bank Account Number primitives for `RustUse` finance crates.

`use-iban` validates IBAN shape, country-specific length, and the standard mod-97 checksum. It normalizes compact text for local storage and can format grouped display text.

## Example

```rust
use use_iban::Iban;

let iban = Iban::new("gb82 west 1234 5698 7654 32")?;

assert_eq!(iban.as_str(), "GB82WEST12345698765432");
assert_eq!(iban.format_grouped(), "GB82 WEST 1234 5698 7654 32");
assert_eq!(iban.country_code(), "GB");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for local IBAN validation, compact normalization, and display grouping. It does not perform bank lookup, account ownership validation, registry fetching, payment initiation, or bank integration.

## License

Licensed under either MIT or Apache-2.0.
