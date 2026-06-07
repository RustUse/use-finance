# use-bank-account

Conservative bank account primitives for `RustUse` finance crates.

`use-bank-account` provides small account-number, masked-account-number, account-type, account-holder, and bank-account values. Validation is intentionally modest because account number rules vary by institution and country.

## Example

```rust
use use_bank_account::{AccountHolderName, AccountNumber, AccountType, BankAccount};

let account = BankAccount::new(
    AccountNumber::new("1234567890")?,
    AccountType::Checking,
    AccountHolderName::new("Example LLC")?,
);

assert_eq!(account.masked_number().as_str(), "******7890");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for local bank account vocabulary and masking. It does not validate account ownership, call bank APIs, process ACH or wires, or model institution-specific numbering rules.

## License

Licensed under either MIT or Apache-2.0.
