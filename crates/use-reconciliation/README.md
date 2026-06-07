# use-reconciliation

Deterministic reconciliation primitives for `RustUse` finance crates.

`use-reconciliation` provides bounded match scores, confidence vocabulary, candidates, results, and exception reasons. It intentionally contains no machine learning, LLM, or probabilistic matching logic.

## Example

```rust
use use_amount::Amount;
use use_reconciliation::{MatchScore, ReconciliationCandidate};

let candidate = ReconciliationCandidate::new(
    "bank-line-1",
    "invoice-1001",
    Amount::zero(2)?,
    MatchScore::exact(),
)?;

assert!(candidate.is_exact_amount_match());
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for deterministic reconciliation vocabulary and helper types. It does not infer matches, train models, call LLMs, connect to banks, query databases, or perform workflow automation.

## License

Licensed under either MIT or Apache-2.0.
