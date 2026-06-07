# use-ach

Conservative ACH and NACHA-oriented primitives for `RustUse` finance crates.

`use-ach` provides small enums, identifiers, and lightweight entry metadata for local ACH vocabulary. It intentionally avoids full NACHA file parsing or payment-network workflow rules.

## Example

```rust
use use_ach::{
    AchEntry, AchStandardEntryClass, AchTraceNumber, AchTransactionCode, AchCompanyId,
    AchIndividualId,
};

let entry = AchEntry::new(
    AchStandardEntryClass::Ppd,
    AchTransactionCode::CheckingCredit,
    AchTraceNumber::new("123456780000001")?,
    AchCompanyId::new("1234567890")?,
    AchIndividualId::new("EMPLOYEE001")?,
);

assert_eq!(entry.standard_entry_class().as_str(), "PPD");
assert_eq!(entry.transaction_code().code(), 22);
assert_eq!(entry.trace_number().odfi_identification(), "12345678");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for ACH/NACHA vocabulary, identifiers, and entry metadata. It does not parse or generate full NACHA files, process payments, model returns or reversals, choose settlement windows, enforce same-day ACH operational rules, or implement ODFI/RDFI workflows.

## License

Licensed under either MIT or Apache-2.0.
