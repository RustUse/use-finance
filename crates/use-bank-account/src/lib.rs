#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common bank account primitives.
pub mod prelude {
    pub use crate::{
        AccountHolderName, AccountNumber, AccountType, BankAccount, BankAccountError,
        MaskedAccountNumber,
    };
}

/// A conservatively validated bank account number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AccountNumber(String);

impl AccountNumber {
    /// Creates an account number from 1 to 34 ASCII alphanumeric characters.
    ///
    /// # Errors
    ///
    /// Returns [`BankAccountError::EmptyAccountNumber`] for empty input,
    /// [`BankAccountError::AccountNumberTooLong`] for values longer than 34 characters, and
    /// [`BankAccountError::InvalidAccountNumberCharacter`] for non-alphanumeric characters.
    pub fn new(value: impl AsRef<str>) -> Result<Self, BankAccountError> {
        let value = value.as_ref().trim();
        if value.is_empty() {
            return Err(BankAccountError::EmptyAccountNumber);
        }

        if value.len() > 34 {
            return Err(BankAccountError::AccountNumberTooLong);
        }

        if !value.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
            return Err(BankAccountError::InvalidAccountNumberCharacter);
        }

        Ok(Self(value.to_string()))
    }

    /// Returns the account number.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns a masked account number with the last four characters visible.
    #[must_use]
    pub fn masked(&self) -> MaskedAccountNumber {
        MaskedAccountNumber::from_account_number(self, 4)
    }
}

impl AsRef<str> for AccountNumber {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AccountNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AccountNumber {
    type Err = BankAccountError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A masked bank account number intended for display.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MaskedAccountNumber(String);

impl MaskedAccountNumber {
    /// Masks an account number, keeping at most `visible_suffix` trailing characters visible.
    #[must_use]
    pub fn from_account_number(account_number: &AccountNumber, visible_suffix: usize) -> Self {
        let value = account_number.as_str();
        let visible = visible_suffix.min(value.len());
        let hidden = value.len() - visible;
        let suffix_start = value.len() - visible;
        let mut masked = "*".repeat(hidden);
        masked.push_str(&value[suffix_start..]);
        Self(masked)
    }

    /// Returns the masked account number.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MaskedAccountNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Broad account type vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AccountType {
    /// Checking or current account.
    Checking,
    /// Savings account.
    Savings,
    /// Money market deposit account.
    MoneyMarket,
    /// Loan account.
    Loan,
    /// Credit account.
    Credit,
    /// Other account type.
    Other,
}

impl fmt::Display for AccountType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
            Self::MoneyMarket => "money-market",
            Self::Loan => "loan",
            Self::Credit => "credit",
            Self::Other => "other",
        })
    }
}

/// A non-empty account holder name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AccountHolderName(String);

impl AccountHolderName {
    /// Creates an account holder name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`BankAccountError::EmptyAccountHolderName`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, BankAccountError> {
        let value = value.as_ref().trim();
        if value.is_empty() {
            return Err(BankAccountError::EmptyAccountHolderName);
        }

        Ok(Self(value.to_string()))
    }

    /// Returns the account holder name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AccountHolderName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A bank account with a number, type, and holder name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankAccount {
    number: AccountNumber,
    account_type: AccountType,
    holder_name: AccountHolderName,
}

impl BankAccount {
    /// Creates a bank account from validated parts.
    #[must_use]
    pub const fn new(
        number: AccountNumber,
        account_type: AccountType,
        holder_name: AccountHolderName,
    ) -> Self {
        Self {
            number,
            account_type,
            holder_name,
        }
    }

    /// Returns the account number.
    #[must_use]
    pub const fn number(&self) -> &AccountNumber {
        &self.number
    }

    /// Returns the masked account number.
    #[must_use]
    pub fn masked_number(&self) -> MaskedAccountNumber {
        self.number.masked()
    }

    /// Returns the account type.
    #[must_use]
    pub const fn account_type(&self) -> AccountType {
        self.account_type
    }

    /// Returns the account holder name.
    #[must_use]
    pub const fn holder_name(&self) -> &AccountHolderName {
        &self.holder_name
    }
}

/// Errors returned by bank account primitives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankAccountError {
    /// The account number was empty after trimming whitespace.
    EmptyAccountNumber,
    /// The account number was longer than 34 characters.
    AccountNumberTooLong,
    /// The account number contained a non-alphanumeric character.
    InvalidAccountNumberCharacter,
    /// The account holder name was empty after trimming whitespace.
    EmptyAccountHolderName,
}

impl fmt::Display for BankAccountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAccountNumber => formatter.write_str("account number cannot be empty"),
            Self::AccountNumberTooLong => {
                formatter.write_str("account number cannot exceed 34 characters")
            },
            Self::InvalidAccountNumberCharacter => {
                formatter.write_str("account number must be ASCII alphanumeric")
            },
            Self::EmptyAccountHolderName => {
                formatter.write_str("account holder name cannot be empty")
            },
        }
    }
}

impl Error for BankAccountError {}

#[cfg(test)]
mod tests {
    use super::{
        AccountHolderName, AccountNumber, AccountType, BankAccount, BankAccountError,
        MaskedAccountNumber,
    };

    #[test]
    fn creates_bank_account_and_mask() -> Result<(), BankAccountError> {
        let account = BankAccount::new(
            AccountNumber::new("1234567890")?,
            AccountType::Checking,
            AccountHolderName::new("Example LLC")?,
        );

        assert_eq!(account.number().as_str(), "1234567890");
        assert_eq!(account.masked_number().as_str(), "******7890");
        assert_eq!(account.account_type(), AccountType::Checking);
        assert_eq!(account.holder_name().as_str(), "Example LLC");
        Ok(())
    }

    #[test]
    fn rejects_empty_or_symbolic_account_numbers() {
        assert_eq!(
            AccountNumber::new(""),
            Err(BankAccountError::EmptyAccountNumber)
        );
        assert_eq!(
            AccountNumber::new("123-456"),
            Err(BankAccountError::InvalidAccountNumberCharacter)
        );
    }

    #[test]
    fn supports_custom_mask_width() -> Result<(), BankAccountError> {
        let number = AccountNumber::new("ABCD1234")?;
        assert_eq!(
            MaskedAccountNumber::from_account_number(&number, 2).as_str(),
            "******34"
        );
        Ok(())
    }
}
