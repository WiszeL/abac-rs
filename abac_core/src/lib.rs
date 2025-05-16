//! abac-rs — Attribute-Based Access Control (ABAC) Engine in Pure Rust
//!
//! This crate provides a lightweight and expressive ABAC evaluation engine built entirely in Rust.
//! Policies are defined as human-readable strings and evaluated against subjects and objects
//! implementing the `Entity` trait.
//!
//! ## ✨ Features
//!
//! - ✅ Human-readable rule strings (`subject.role == 'admin'`)
//! - ✅ Supports `AND` of `OR` groups: `[A, B], [C]` → `(A OR B) AND C`
//! - ✅ Dynamic field access via `Entity` trait
//! - ✅ Supports comparison on `String`, `i32`, `f32`, `bool`
//! - ✅ Custom error types for integration (`Parse`, `UnknownField`, `TypeMismatch`)
//! - 🌐 Easily extensible (e.g. time, IP, env conditions)
//!
//! ## 🔧 Example
//!
//! ```rust,ignore
//! use abac_rs::{evaluate_rules, Entity};
//!
//! #[derive(Entity)]
//! struct User {
//!     role: String,
//!     department: String,
//!     id: i32,
//! }
//!
//! #[derive(Entity)]
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

mod entity;
mod operator;
mod rules;

pub mod error;

use error::Error;
use operator::cmp;
use rules::{Clause, Operand, Rules};

pub use entity::{Entity, Value};

/// Evaluates a full ABAC policy string against the given subject and object.
///
/// # Arguments
/// - `rules`: policy string
/// - `sub`: reference to subject implementing `Entity`
/// - `obj`: reference to object implementing `Entity`
///
/// # Returns
/// - `Ok(true)` if policy is satisfied
/// - `Ok(false)` if denied
/// - `Err(Error)` if parsing or evaluation fails
pub fn evaluate_rules(
    rules: &str,
    sub: &(impl Entity + ?Sized),
    obj: &(impl Entity + ?Sized),
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
    sub: &'a (impl Entity + ?Sized),
    obj: &'a (impl Entity + ?Sized),
    operand: &'a Operand,
) -> Result<Value<'a>, Error> {
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
    sub: &(impl Entity + ?Sized),
    obj: &(impl Entity + ?Sized),
) -> Result<bool, Error> {
    let lhs = resolve_operand(sub, obj, &c.left)?;
    let rhs = resolve_operand(sub, obj, &c.right)?;

    match (lhs, rhs) {
        (Value::Str(l), Value::Str(r)) => Ok(cmp(l, r, &c.op)),
        (Value::Int(l), Value::Int(r)) => Ok(cmp(l, r, &c.op)),
        (Value::Float(l), Value::Float(r)) => Ok(cmp(l, r, &c.op)),
        (Value::Bool(l), Value::Bool(r)) => Ok(cmp(l, r, &c.op)),
        (l, r) => Err(Error::TypeMismatch {
            lhs: l.kind(),
            rhs: r.kind(),
            op: c.op,
        }), // type mismatch or missing field
    }
}
