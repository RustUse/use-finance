#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::{error::Error, slice};

use use_amount::Amount;

/// Common reconciliation primitives.
pub mod prelude {
    pub use crate::{
        ExceptionReason, MatchConfidence, MatchScore, MatchStatus, ReconciliationCandidate,
        ReconciliationError, ReconciliationResult,
    };
}

/// Match lifecycle status.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchStatus {
    /// No match has been selected.
    Unmatched,
    /// A candidate match exists.
    Candidate,
    /// The match has been confirmed.
    Matched,
    /// The item was excluded from matching.
    Ignored,
}

/// Human-readable confidence band for a deterministic match score.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchConfidence {
    /// No confidence.
    None,
    /// Low confidence.
    Low,
    /// Medium confidence.
    Medium,
    /// High confidence.
    High,
    /// Exact confidence.
    Exact,
}

/// A bounded deterministic match score from 0 to 10,000 basis points.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MatchScore {
    basis_points: u16,
}

impl MatchScore {
    /// Creates a match score from basis points in the inclusive range 0..=10,000.
    ///
    /// # Errors
    ///
    /// Returns [`ReconciliationError::ScoreOutOfRange`] when the value is greater than 10,000.
    pub const fn from_basis_points(basis_points: u16) -> Result<Self, ReconciliationError> {
        if basis_points > 10_000 {
            Err(ReconciliationError::ScoreOutOfRange)
        } else {
            Ok(Self { basis_points })
        }
    }

    /// Returns an exact match score.
    #[must_use]
    pub const fn exact() -> Self {
        Self {
            basis_points: 10_000,
        }
    }

    /// Returns the score in basis points.
    #[must_use]
    pub const fn basis_points(self) -> u16 {
        self.basis_points
    }

    /// Returns the confidence band for this score.
    #[must_use]
    pub const fn confidence(self) -> MatchConfidence {
        match self.basis_points {
            10_000 => MatchConfidence::Exact,
            8_000..=u16::MAX => MatchConfidence::High,
            5_000..=7_999 => MatchConfidence::Medium,
            1..=4_999 => MatchConfidence::Low,
            0 => MatchConfidence::None,
        }
    }
}

/// A deterministic reconciliation candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationCandidate {
    source_id: String,
    target_id: String,
    amount_delta: Amount,
    score: MatchScore,
    status: MatchStatus,
}

impl ReconciliationCandidate {
    /// Creates a reconciliation candidate.
    ///
    /// # Errors
    ///
    /// Returns [`ReconciliationError::EmptySourceId`] or [`ReconciliationError::EmptyTargetId`]
    /// when identifiers are empty after trimming whitespace.
    pub fn new(
        source_id: impl AsRef<str>,
        target_id: impl AsRef<str>,
        amount_delta: Amount,
        score: MatchScore,
    ) -> Result<Self, ReconciliationError> {
        Ok(Self {
            source_id: non_empty(source_id, ReconciliationError::EmptySourceId)?,
            target_id: non_empty(target_id, ReconciliationError::EmptyTargetId)?,
            amount_delta,
            score,
            status: MatchStatus::Candidate,
        })
    }

    /// Returns the source identifier.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the target identifier.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Returns the amount delta between source and target.
    #[must_use]
    pub const fn amount_delta(&self) -> Amount {
        self.amount_delta
    }

    /// Returns the match score.
    #[must_use]
    pub const fn score(&self) -> MatchScore {
        self.score
    }

    /// Returns the match status.
    #[must_use]
    pub const fn status(&self) -> MatchStatus {
        self.status
    }

    /// Returns whether the amount delta is zero.
    #[must_use]
    pub const fn is_exact_amount_match(&self) -> bool {
        self.amount_delta.is_zero()
    }

    /// Sets the candidate status.
    #[must_use]
    pub const fn with_status(mut self, status: MatchStatus) -> Self {
        self.status = status;
        self
    }
}

/// A deterministic reconciliation result.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReconciliationResult {
    candidates: Vec<ReconciliationCandidate>,
    exceptions: Vec<ExceptionReason>,
}

impl ReconciliationResult {
    /// Creates an empty reconciliation result.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            candidates: Vec::new(),
            exceptions: Vec::new(),
        }
    }

    /// Adds a candidate.
    pub fn push_candidate(&mut self, candidate: ReconciliationCandidate) {
        self.candidates.push(candidate);
    }

    /// Adds an exception reason.
    pub fn push_exception(&mut self, exception: ExceptionReason) {
        self.exceptions.push(exception);
    }

    /// Returns reconciliation candidates.
    #[must_use]
    pub fn candidates(&self) -> &[ReconciliationCandidate] {
        &self.candidates
    }

    /// Returns exception reasons.
    #[must_use]
    pub fn exceptions(&self) -> &[ExceptionReason] {
        &self.exceptions
    }

    /// Iterates over candidates.
    pub fn iter(&self) -> slice::Iter<'_, ReconciliationCandidate> {
        self.candidates.iter()
    }
}

/// Reconciliation exception reason vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExceptionReason {
    /// Amounts differ beyond the caller's tolerance.
    AmountMismatch,
    /// Dates differ beyond the caller's tolerance.
    DateMismatch,
    /// Duplicate candidate was found.
    DuplicateCandidate,
    /// Required reference was missing.
    MissingReference,
    /// Currency differed between items.
    CurrencyMismatch,
    /// Caller-defined exception reason.
    Other(String),
}

/// Errors returned by reconciliation primitives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReconciliationError {
    /// The source identifier was empty.
    EmptySourceId,
    /// The target identifier was empty.
    EmptyTargetId,
    /// The score was outside the inclusive 0..=10,000 range.
    ScoreOutOfRange,
}

impl fmt::Display for ReconciliationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySourceId => formatter.write_str("source identifier cannot be empty"),
            Self::EmptyTargetId => formatter.write_str("target identifier cannot be empty"),
            Self::ScoreOutOfRange => {
                formatter.write_str("match score must be between 0 and 10000 basis points")
            },
        }
    }
}

impl Error for ReconciliationError {}

impl<'a> IntoIterator for &'a ReconciliationResult {
    type Item = &'a ReconciliationCandidate;
    type IntoIter = slice::Iter<'a, ReconciliationCandidate>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn non_empty(
    value: impl AsRef<str>,
    error: ReconciliationError,
) -> Result<String, ReconciliationError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use use_amount::Amount;

    use super::{
        ExceptionReason, MatchConfidence, MatchScore, MatchStatus, ReconciliationCandidate,
        ReconciliationError, ReconciliationResult,
    };

    #[test]
    fn creates_candidate_with_exact_score() -> Result<(), Box<dyn std::error::Error>> {
        let candidate = ReconciliationCandidate::new(
            "bank-line-1",
            "invoice-1001",
            Amount::zero(2)?,
            MatchScore::exact(),
        )?;

        assert!(candidate.is_exact_amount_match());
        assert_eq!(candidate.score().confidence(), MatchConfidence::Exact);
        assert_eq!(candidate.status(), MatchStatus::Candidate);
        Ok(())
    }

    #[test]
    fn rejects_invalid_score_and_empty_ids() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            MatchScore::from_basis_points(10_001),
            Err(ReconciliationError::ScoreOutOfRange)
        );
        assert_eq!(
            ReconciliationCandidate::new("", "target", Amount::zero(2)?, MatchScore::exact()),
            Err(ReconciliationError::EmptySourceId)
        );
        Ok(())
    }

    #[test]
    fn collects_candidates_and_exceptions() -> Result<(), Box<dyn std::error::Error>> {
        let mut result = ReconciliationResult::new();
        result.push_candidate(ReconciliationCandidate::new(
            "a",
            "b",
            Amount::zero(2)?,
            MatchScore::from_basis_points(8_000)?,
        )?);
        result.push_exception(ExceptionReason::MissingReference);

        assert_eq!(result.candidates().len(), 1);
        assert_eq!(result.exceptions(), &[ExceptionReason::MissingReference]);
        Ok(())
    }
}
