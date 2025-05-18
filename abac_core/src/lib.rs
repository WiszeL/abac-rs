//! abac-rs — Attribute-Based Access Control Engine
//!
//! ### Features
//! - Human-readable rule strings (`subject.role == 'admin'`).
//! - AND-of-OR evaluation (`[A,B], [C]` ⇒ `(A OR B) AND C`).
//! - Optional *object* support: omit `object` for type-level actions
//!   (e.g. `list users`) and all `object.*` clauses are ignored.
//! - Extensible: plug extra `Value` variants or custom operators.
//!
//! ## Quick start
//! ```no_run
//! use abac_rs::{Entity, evaluate_rules};
//!
//! # #[derive(abac_rs::Entity)]
//! # struct User { role: &'static str, department: &'static str }
//! # let user = User { role: "admin", department: "it" };
//! let policy = "[ subject.role == 'admin' ]";
//! assert!(evaluate_rules(policy, &user, None).unwrap());
//! ```

mod entity;
mod error;
mod operator;
mod rules;

pub use entity::{Entity, NullEntity, Value};
pub use error::Error;
pub use rules::{AnyOf, Clause, Operand, Rules};

use operator::cmp;
use rules::clause_uses_object;

/// Public evaluation entry point.
///
/// * `sub`  – the **subject** performing the action.
/// * `obj`  – the **object** being accessed, or `None` for type-level checks.
/// * `rules` – policy string.
///
/// If `obj` is `None`, every clause containing `object.*` is silently skipped.
///
/// Returns `Ok(true)` if all AND-groups succeed, `Ok(false)` if denied,
/// or `Err` for policy / type errors.
pub fn evaluate_rules(
    rules: &str,
    sub: &dyn Entity,
    obj: Option<&dyn Entity>,
) -> Result<bool, Error> {
    let rules = Rules::try_from(rules).map_err(Error::Parse)?;

    // use a dummy object when `None`
    let null_obj = NullEntity;
    let obj_ref = obj.unwrap_or(&null_obj);

    'groups: for any in &rules.0 {
        for clause in &any.0 {
            if obj.is_none() && clause_uses_object(clause) {
                continue; // skip clause that needs object.*
            }
            if eval_clause(clause, sub, obj_ref)? {
                continue 'groups; // OR-group satisfied
            }
        }
        return Ok(false); // AND-group failed
    }
    Ok(true)
}

// ---------- internal helpers ----------------------------------------

fn eval_clause(c: &Clause, sub: &dyn Entity, obj: &dyn Entity) -> Result<bool, Error> {
    // Resolve operand -----------------------------------------------
    fn resolve<'a>(
        op: &'a Operand<'a>,
        sub: &'a dyn Entity,
        obj: &'a dyn Entity,
    ) -> Result<Value<'a>, Error> {
        match *op {
            Operand::Subject(field) => sub
                .get_field(field)
                .ok_or_else(|| Error::UnknownField(format!("subject.{field}"))),
            Operand::Object(field) => obj
                .get_field(field)
                .ok_or_else(|| Error::UnknownField(format!("object.{field}"))),
            Operand::Const(ref v) => Ok(v.clone()),
        }
    }

    let l = resolve(&c.left, sub, obj)?;
    let r = resolve(&c.right, sub, obj)?;

    // Type-safe comparison ------------------------------------------
    macro_rules! cmp_match {
        ($lhs:ident, $rhs:ident, $ty:path) => {
            cmp($lhs, $rhs, &c.op)
        };
    }
    use Value::*;
    Ok(match (l, r) {
        (Str(l), Str(r)) => cmp_match!(l, r, Str),
        (Int(l), Int(r)) => cmp_match!(l, r, Int),
        (Float(l), Float(r)) => cmp_match!(l, r, Float),
        (Bool(l), Bool(r)) => cmp_match!(l, r, Bool),
        (Uuid(l), Uuid(r)) => cmp_match!(l, r, Uuid),
        (lhs, rhs) => {
            return Err(Error::TypeMismatch {
                lhs: lhs.kind(),
                rhs: rhs.kind(),
                op: c.op,
            });
        }
    })
}
