//! Error definitions for ABAC evaluation.
//!
//! This module contains the core `Error` type used throughout the ABAC engine,
//! covering parse errors, unknown field lookups, and type mismatches.

use crate::operator::Op;

/// Top-level error type for ABAC evaluation.
#[derive(Debug)]
pub enum Error {
    /// Returned when the input policy string is invalid.
    Parse(String),
    /// A referenced attribute was not found on the subject or object.
    UnknownField(String),
    /// A clause failed because operands are of mismatched types.
    TypeMismatch {
        lhs: &'static str,
        rhs: &'static str,
        op: Op,
    },
}
