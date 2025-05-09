use std::str::FromStr;

use reflect_rs::ReflValue;

use crate::operator::Op;

pub enum Operand {
    Subject(String),  // Attr on subject
    Object(String),   // Attr on object
    Const(ReflValue), // literal/hardcoded value
}

impl FromStr for Operand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.starts_with("subject.") {
            Ok(Operand::Subject(s[8..].to_string()))
        } else if s.starts_with("object.") {
            Ok(Operand::Object(s[7..].to_string()))
        } else if s.starts_with('\'') && s.ends_with('\'') {
            let val = &s[1..s.len() - 1];
            Ok(Operand::Const(ReflValue::Str(val.into())))
        } else if let Ok(int) = s.parse::<i32>() {
            Ok(Operand::Const(ReflValue::Int(int)))
        } else {
            Err(format!("Invalid operand: {s}"))
        }
    }
}
pub struct Clause {
    pub left: Operand,
    pub op: Op,
    pub right: Operand,
}

impl FromStr for Clause {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // naive split — improve later with proper tokenizer
        let ops = ["==", "!=", ">=", "<=", ">", "<"];
        for op in ops {
            if let Some(i) = s.find(op) {
                let (left, right) = s.split_at(i);
                let right = &right[op.len()..];
                return Ok(Clause {
                    left: left.parse()?,
                    op: op.parse()?,
                    right: right.parse()?,
                });
            }
        }
        Err("No valid operator found".into())
    }
}
/// One disjunction (OR‑group)
pub struct AnyOf(pub Vec<Clause>); // true if **one** Clause is true

impl FromStr for AnyOf {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s.trim().trim_start_matches('[').trim_end_matches(']');
        let clauses = inner
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|c| c.parse::<Clause>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(AnyOf(clauses))
    }
}

/// A whole policy = conjunction (AND) of OR‑groups
pub struct Rules(pub Vec<AnyOf>);

impl FromStr for Rules {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = s
            .split("],") // crude but works for your format
            .map(|g| g.parse::<AnyOf>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Rules(groups))
    }
}
