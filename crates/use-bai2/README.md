# use-bai2

Initial BAI2 banking file primitives for `RustUse` finance crates.

`use-bai2` provides conservative BAI2 record-code parsing, typed record wrappers, continuation handling, basic validation, and normalized transaction detail output. It intentionally preserves raw codes where the BAI2 standard can expand later.

## Example

```rust
use use_bai2::{parse_logical_records, NormalizedTransaction, TransactionDetailRecord};

let records = parse_logical_records("16,475,12345,Z,bank-ref,customer-ref,payment/\n")?;
let detail = TransactionDetailRecord::try_from(&records[0])?;
let normalized = NormalizedTransaction::from_detail(&detail)?;

assert_eq!(normalized.amount().minor_units(), 12_345);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for first-pass BAI2 parsing and validation. It does not fully enumerate every BAI2 type code, connect to banks, fetch statements, process ACH or wires, or perform reconciliation.

## License

Licensed under either MIT or Apache-2.0.
