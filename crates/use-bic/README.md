# use-bic

Bank Identifier Code primitives for `RustUse` finance crates.

`use-bic` validates SWIFT/BIC-style bank identifier codes with 8-character and 11-character forms. It normalizes input by trimming whitespace and uppercasing letters.

## Example

```rust
use use_bic::Bic;

let bic = Bic::new("deutdeff500")?;

assert_eq!(bic.as_str(), "DEUTDEFF500");
assert_eq!(bic.bank_code(), "DEUT");
assert_eq!(bic.country_code(), "DE");
assert_eq!(bic.location_code(), "FF");
assert_eq!(bic.branch_code(), Some("500"));
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for offline BIC shape validation and local code handling. It does not implement SWIFT network messaging, MT/MX parsing, ISO 20022 messages, live bank-directory lookup, payment transmission, or bank connectivity.

## License

Licensed under either MIT or Apache-2.0.
