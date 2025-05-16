//! Error definitions for ABAC evaluation.
//!
//! This module contains the core `Error` type used throughout the ABAC engine,
//! covering parse errors, unknown field lookups, and type mismatches.

use thiserror::Error;

use crate::operator::Op;

/// Top-level error type for ABAC evaluation.
#[derive(Debug, Error)]
pub enum Error {
    /// Returned when the input policy string is invalid.
    #[error("Parse Error: {0}")]
    Parse(String),

    /// A referenced attribute was not found on the subject or object.
    #[error("Unknown Field Error: {0}")]
    UnknownField(String),

    /// A clause failed because operands are of mismatched types.
    #[error("Type mismatch: left `{lhs}` vs right `{rhs}` for operation `{op}`")]
    TypeMismatch {
        lhs: &'static str,
        rhs: &'static str,
        op: Op,
    },
}
