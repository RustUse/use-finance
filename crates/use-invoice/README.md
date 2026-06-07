# use-invoice

Invoice primitives for `RustUse` finance crates.

`use-invoice` provides invoice numbers, statuses, lines, due dates, subtotal, total, and balance-due values without implementing a tax engine or billing platform.

## Example

```rust
use use_amount::Amount;
use use_currency::CurrencyCode;
use use_invoice::{Invoice, InvoiceLine, InvoiceNumber};
use use_money::Money;

let usd = CurrencyCode::new("USD")?;
let invoice = Invoice::from_lines(
    InvoiceNumber::new("inv-1001")?,
    vec![InvoiceLine::new(
        "consulting",
        Money::new(Amount::from_minor_units(25_000, 2)?, usd),
    )?],
)?;

assert_eq!(invoice.total().amount().amount().minor_units(), 25_000);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for general invoice vocabulary and simple same-currency totals. It does not calculate taxes, connect to billing systems, send invoices, process payments, or implement country-specific rules.

## License

Licensed under either MIT or Apache-2.0.
