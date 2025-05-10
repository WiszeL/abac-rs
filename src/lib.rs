//! Attribute-Based Access Control (ABAC) Engine in Pure Rust
//! ----------------------------------------------------------
//!
//! This crate provides a lightweight and expressive ABAC evaluation engine
//! built entirely in Rust. Policies are defined as human-readable strings
//! and evaluated against subjects and objects implementing the `Reflection` trait.
//!
//! - Policy: `[ subject.role == 'admin', object.owner_id == subject.id ], [ subject.department == 'informatics' ]`
//! - Semantics: (A OR B) AND (C)
//! - Integration: Just implement `Reflection` on your types!

mod operator;

pub mod error;
pub mod rules;

pub use reflect_rs::Reflection as AbacEntity;

use error::Error;
use operator::cmp;
use reflect_rs::ReflValue;
use rules::{Clause, Operand, Rules};

/// Evaluates a full ABAC policy string against the given subject and object.
///
/// # Arguments
/// - `rules`: policy string
/// - `sub`: reference to subject implementing `AbacEntity`
/// - `obj`: reference to object implementing `AbacEntity`
///
/// # Returns
/// - `Ok(true)` if policy is satisfied
/// - `Ok(false)` if denied
/// - `Err(Error)` if parsing or evaluation fails
pub fn evaluate_rules(
    rules: &str,
    sub: &impl AbacEntity,
    obj: &impl AbacEntity,
) -> Result<bool, Error> {
    let rules = Rules::try_from(rules).map_err(Error::Parse)?;

    for any in &rules.0 {
        let mut group_ok = false;
        for clause in &any.0 {
            if eval_clause(clause, sub, obj)? {
                group_ok = true; // one clause true ⇒ OR‑group true
                break;
            }
        }
        if !group_ok {
            return Ok(false); // AND‑chain fails fast
        }
    }
    Ok(true)
}

/// Resolves a field or literal operand into its value using subject/object reflection.
fn resolve_operand<'a>(
    sub: &'a impl AbacEntity,
    obj: &'a impl AbacEntity,
    operand: &'a Operand,
) -> Result<ReflValue<'a>, Error> {
    match *operand {
        Operand::Subject(attr) => sub
            .get_field(attr)
            .ok_or_else(|| Error::UnknownField(format!("Subject's field: {attr}"))),
        Operand::Object(attr) => obj
            .get_field(attr)
            .ok_or_else(|| Error::UnknownField(format!("Object's field: {attr}"))),
        Operand::Const(ref v) => Ok(v.clone()),
    }
}

/// Evaluates a single clause with resolved operands and operator.
fn eval_clause(c: &Clause, sub: &impl AbacEntity, obj: &impl AbacEntity) -> Result<bool, Error> {
    let lhs = resolve_operand(sub, obj, &c.left)?;
    let rhs = resolve_operand(sub, obj, &c.right)?;

    match (lhs, rhs) {
        (ReflValue::Str(l), ReflValue::Str(r)) => Ok(cmp(l, r, &c.op)),
        (ReflValue::Int(l), ReflValue::Int(r)) => Ok(cmp(l, r, &c.op)),
        (ReflValue::Float(l), ReflValue::Float(r)) => Ok(cmp(l, r, &c.op)),
        (ReflValue::Bool(l), ReflValue::Bool(r)) => Ok(cmp(l, r, &c.op)),
        (l, r) => Err(Error::TypeMismatch {
            lhs: l.kind(),
            rhs: r.kind(),
            op: c.op,
        }), // type mismatch or missing field
    }
}
