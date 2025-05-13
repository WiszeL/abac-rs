//! abac-rs — Attribute-Based Access Control (ABAC) Engine in Pure Rust
//!
//! This crate provides a lightweight and expressive ABAC evaluation engine built entirely in Rust.
//! Policies are defined as human-readable strings and evaluated against subjects and objects
//! implementing the `AbacEntity` trait.
//!
//! ## ✨ Features
//!
//! - ✅ Human-readable rule strings (`subject.role == 'admin'`)
//! - ✅ Supports `AND` of `OR` groups: `[A, B], [C]` → `(A OR B) AND C`
//! - ✅ Dynamic field access via `AbacEntity` trait
//! - ✅ Supports comparison on `String`, `i32`, `f32`, `bool`
//! - ✅ Custom error types for integration (`Parse`, `UnknownField`, `TypeMismatch`)
//! - 🌐 Easily extensible (e.g. time, IP, env conditions)
//!
//! ## 🔧 Example
//!
//! ```rust,ignore
//! use abac_rs::{evaluate_rules, AbacEntity};
//!
//! #[derive(AbacEntity)]
//! struct User {
//!     role: String,
//!     department: String,
//!     id: i32,
//! }
//!
//! #[derive(AbacEntity)]
//! struct File {
//!     owner_id: i32,
//!     tag: String,
//! }
//!
//! let rules = r#"
//!     [ subject.role == 'admin', object.owner_id == subject.id ],
//!     [ subject.department == 'informatics' ]
//! "#;
//!
//! let user = User { role: "admin".into(), department: "informatics".into(), id: 42 };
//! let file = File { owner_id: 42, tag: "draft".into() };
//!
//! let allowed = evaluate_rules(rules, &user, &file)?;
//! assert!(allowed);
//! ```

mod operator;

pub mod error;
pub mod rules;

use error::Error;
use operator::cmp;
use reflect_rs::ReflValue;
use rules::{Clause, Operand, Rules};

/// A trait representing any ABAC entity (subject or object) with field-level reflection.
/// Re-exported from [`reflect_rs::Reflection`](https://github.com/WiszeL/reflect-rs).
pub use reflect_rs::Reflection as AbacEntity;

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
    sub: &(impl AbacEntity + ?Sized),
    obj: &(impl AbacEntity + ?Sized),
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
    sub: &'a (impl AbacEntity + ?Sized),
    obj: &'a (impl AbacEntity + ?Sized),
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
fn eval_clause(
    c: &Clause,
    sub: &(impl AbacEntity + ?Sized),
    obj: &(impl AbacEntity + ?Sized),
) -> Result<bool, Error> {
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
