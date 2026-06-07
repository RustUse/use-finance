# use-routing-number

ABA routing number validation primitives for `RustUse` finance crates.

`use-routing-number` validates 9-digit ABA routing numbers with the standard checksum. It does not connect to banks or verify account ownership.

## Example

```rust
use use_routing_number::RoutingNumber;

let routing = RoutingNumber::new("021000021")?;

assert_eq!(routing.as_str(), "021000021");
assert!(RoutingNumber::new("021000022").is_err());
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for ABA routing number shape and checksum validation. It does not provide bank-directory lookups, ACH processing, wire processing, or account validation.

## License

Licensed under either MIT or Apache-2.0.
