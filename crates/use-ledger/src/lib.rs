#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::{collections::BTreeMap, error::Error, slice};

use use_money::{Money, MoneyError};

/// Common ledger primitives.
pub mod prelude {
    pub use crate::{Balance, DebitCredit, JournalEntry, LedgerEntry, LedgerError, Posting};
}

/// Debit or credit side of a posting.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DebitCredit {
    /// Debit side.
    Debit,
    /// Credit side.
    Credit,
}

/// A single account posting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Posting {
    account_id: String,
    amount: Money,
    side: DebitCredit,
}

impl Posting {
    /// Creates a posting with a non-empty account identifier.
    ///
    /// # Errors
    ///
    /// Returns [`LedgerError::EmptyAccountId`] when the trimmed account identifier is empty.
    pub fn new(
        account_id: impl AsRef<str>,
        amount: Money,
        side: DebitCredit,
    ) -> Result<Self, LedgerError> {
        let account_id = non_empty(account_id, LedgerError::EmptyAccountId)?;
        Ok(Self {
            account_id,
            amount,
            side,
        })
    }

    /// Returns the account identifier.
    #[must_use]
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    /// Returns the posted amount.
    #[must_use]
    pub const fn amount(&self) -> &Money {
        &self.amount
    }

    /// Returns the debit or credit side.
    #[must_use]
    pub const fn side(&self) -> DebitCredit {
        self.side
    }
}

/// A balanced journal entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JournalEntry {
    entry_id: String,
    postings: Vec<Posting>,
}

impl JournalEntry {
    /// Creates a journal entry and validates that debits and credits balance.
    ///
    /// # Errors
    ///
    /// Returns [`LedgerError::EmptyEntryId`] for an empty entry identifier,
    /// [`LedgerError::NoPostings`] when no postings are supplied, and
    /// [`LedgerError::NotBalanced`] when debits and credits do not balance by currency and scale.
    pub fn new(entry_id: impl AsRef<str>, postings: Vec<Posting>) -> Result<Self, LedgerError> {
        let entry_id = non_empty(entry_id, LedgerError::EmptyEntryId)?;
        if postings.is_empty() {
            return Err(LedgerError::NoPostings);
        }

        validate_balanced(&postings)?;
        Ok(Self { entry_id, postings })
    }

    /// Returns the journal entry identifier.
    #[must_use]
    pub fn entry_id(&self) -> &str {
        &self.entry_id
    }

    /// Returns the postings.
    #[must_use]
    pub fn postings(&self) -> &[Posting] {
        &self.postings
    }

    /// Iterates over postings.
    pub fn iter(&self) -> slice::Iter<'_, Posting> {
        self.postings.iter()
    }

    /// Returns whether debits and credits balance by currency and amount scale.
    #[must_use]
    pub fn is_balanced(&self) -> bool {
        validate_balanced(&self.postings).is_ok()
    }
}

/// A journal entry placed into a ledger sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerEntry {
    sequence: u64,
    journal_entry: JournalEntry,
}

impl LedgerEntry {
    /// Creates a ledger entry from a sequence number and balanced journal entry.
    #[must_use]
    pub const fn new(sequence: u64, journal_entry: JournalEntry) -> Self {
        Self {
            sequence,
            journal_entry,
        }
    }

    /// Returns the ledger sequence number.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the journal entry.
    #[must_use]
    pub const fn journal_entry(&self) -> &JournalEntry {
        &self.journal_entry
    }
}

/// An account balance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Balance {
    account_id: String,
    amount: Money,
}

impl Balance {
    /// Creates a balance with a non-empty account identifier.
    ///
    /// # Errors
    ///
    /// Returns [`LedgerError::EmptyAccountId`] when the trimmed account identifier is empty.
    pub fn new(account_id: impl AsRef<str>, amount: Money) -> Result<Self, LedgerError> {
        Ok(Self {
            account_id: non_empty(account_id, LedgerError::EmptyAccountId)?,
            amount,
        })
    }

    /// Returns the account identifier.
    #[must_use]
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    /// Returns the balance amount.
    #[must_use]
    pub const fn amount(&self) -> &Money {
        &self.amount
    }

    /// Applies another same-currency balance amount.
    ///
    /// # Errors
    ///
    /// Returns [`LedgerError::Money`] when money addition fails.
    pub fn checked_add(&self, amount: &Money) -> Result<Self, LedgerError> {
        Ok(Self {
            account_id: self.account_id.clone(),
            amount: self
                .amount
                .checked_add(amount)
                .map_err(LedgerError::Money)?,
        })
    }
}

impl<'a> IntoIterator for &'a JournalEntry {
    type Item = &'a Posting;
    type IntoIter = slice::Iter<'a, Posting>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Errors returned by ledger primitives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerError {
    /// Account identifiers must not be empty.
    EmptyAccountId,
    /// Entry identifiers must not be empty.
    EmptyEntryId,
    /// Journal entries require at least one posting.
    NoPostings,
    /// Debits and credits did not balance.
    NotBalanced,
    /// Money arithmetic failed.
    Money(MoneyError),
}

impl fmt::Display for LedgerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAccountId => formatter.write_str("account identifier cannot be empty"),
            Self::EmptyEntryId => formatter.write_str("entry identifier cannot be empty"),
            Self::NoPostings => formatter.write_str("journal entry requires at least one posting"),
            Self::NotBalanced => {
                formatter.write_str("journal entry debits and credits must balance")
            },
            Self::Money(error) => error.fmt(formatter),
        }
    }
}

impl Error for LedgerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Money(error) => Some(error),
            Self::EmptyAccountId | Self::EmptyEntryId | Self::NoPostings | Self::NotBalanced => {
                None
            },
        }
    }
}

fn non_empty(value: impl AsRef<str>, error: LedgerError) -> Result<String, LedgerError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

fn validate_balanced(postings: &[Posting]) -> Result<(), LedgerError> {
    let mut totals: BTreeMap<(String, u8), (i128, i128)> = BTreeMap::new();

    for posting in postings {
        let key = (
            posting.amount.currency().as_str().to_string(),
            posting.amount.amount().scale(),
        );
        let entry = totals.entry(key).or_insert((0, 0));
        match posting.side {
            DebitCredit::Debit => {
                entry.0 = entry
                    .0
                    .checked_add(posting.amount.amount().minor_units())
                    .ok_or(LedgerError::NotBalanced)?;
            },
            DebitCredit::Credit => {
                entry.1 = entry
                    .1
                    .checked_add(posting.amount.amount().minor_units())
                    .ok_or(LedgerError::NotBalanced)?;
            },
        }
    }

    if totals.values().all(|(debits, credits)| debits == credits) {
        Ok(())
    } else {
        Err(LedgerError::NotBalanced)
    }
}

#[cfg(test)]
mod tests {
    use use_amount::Amount;
    use use_currency::CurrencyCode;
    use use_money::Money;

    use super::{DebitCredit, JournalEntry, LedgerError, Posting};

    fn usd_amount(minor_units: i128) -> Result<Money, Box<dyn std::error::Error>> {
        Ok(Money::new(
            Amount::from_minor_units(minor_units, 2)?,
            CurrencyCode::new("USD")?,
        ))
    }

    #[test]
    fn accepts_balanced_entries() -> Result<(), Box<dyn std::error::Error>> {
        let amount = usd_amount(5_000)?;
        let entry = JournalEntry::new(
            "je-1001",
            vec![
                Posting::new("cash", amount.clone(), DebitCredit::Debit)?,
                Posting::new("revenue", amount, DebitCredit::Credit)?,
            ],
        )?;

        assert!(entry.is_balanced());
        assert_eq!(entry.postings().len(), 2);
        Ok(())
    }

    #[test]
    fn rejects_unbalanced_entries() -> Result<(), Box<dyn std::error::Error>> {
        let entry = JournalEntry::new(
            "je-1002",
            vec![
                Posting::new("cash", usd_amount(5_000)?, DebitCredit::Debit)?,
                Posting::new("revenue", usd_amount(4_999)?, DebitCredit::Credit)?,
            ],
        );

        assert_eq!(entry, Err(LedgerError::NotBalanced));
        Ok(())
    }

    #[test]
    fn rejects_empty_postings_and_accounts() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            JournalEntry::new("je-empty", Vec::new()),
            Err(LedgerError::NoPostings)
        );
        assert_eq!(
            Posting::new("", usd_amount(100)?, DebitCredit::Debit),
            Err(LedgerError::EmptyAccountId)
        );
        Ok(())
    }
}
