//! AST + parser for `[ subject.role == 'admin', object.id == 1 ], [ … ]`

use crate::{entity::Value, operator::Op};

/// Operand in a clause.
pub enum Operand<'a> {
    Subject(&'a str),
    Object(&'a str),
    Const(Value<'a>),
}

/// `<left> <op> <right>`
pub struct Clause<'a> {
    pub left: Operand<'a>,
    pub op: Op,
    pub right: Operand<'a>,
}

/// OR-group: true if *any* clause is true.
pub struct AnyOf<'a>(pub Vec<Clause<'a>>);

/// Policy root: AND of OR-groups.
pub struct Rules<'a>(pub Vec<AnyOf<'a>>);

// ---------- helper used by lib.rs -----------------------------------

#[inline]
pub fn clause_uses_object(c: &Clause) -> bool {
    use Operand::*;
    matches!(c.left, Object(_)) || matches!(c.right, Object(_))
}

// ---------- naïve parser (keep simple & fast) -----------------------

impl<'a> TryFrom<&'a str> for Operand<'a> {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let s = s.trim();
        if let Some(rest) = s.strip_prefix("subject.") {
            Ok(Operand::Subject(rest))
        } else if let Some(rest) = s.strip_prefix("object.") {
            Ok(Operand::Object(rest))
        } else if s.starts_with('\'') && s.ends_with('\'') {
            Ok(Operand::Const(Value::Str(&s[1..s.len() - 1])))
        } else if let Ok(i) = s.parse::<i32>() {
            Ok(Operand::Const(Value::Int(i)))
        } else {
            Err(format!("invalid operand `{s}`"))
        }
    }
}

impl<'a> TryFrom<&'a str> for Clause<'a> {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let ops = ["==", "!=", ">=", "<=", ">", "<"];
        for op in ops {
            if let Some(idx) = s.find(op) {
                let (l, r) = s.split_at(idx);
                return Ok(Clause {
                    left: Operand::try_from(l)?,
                    op: op.parse()?,
                    right: Operand::try_from(&r[op.len()..])?,
                });
            }
        }
        Err("operator not found".into())
    }
}

impl<'a> TryFrom<&'a str> for AnyOf<'a> {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let inner = s.trim().trim_start_matches('[').trim_end_matches(']');
        let clauses = inner
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(Clause::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(AnyOf(clauses))
    }
}

impl<'a> TryFrom<&'a str> for Rules<'a> {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.split("],")
            .map(AnyOf::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map(Rules)
    }
}
