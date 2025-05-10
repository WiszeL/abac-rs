use reflect_rs::ReflValue;

use crate::operator::Op;

/// An operand in a rule clause: either subject field, object field, or literal constant.
pub enum Operand<'a> {
    Subject(&'a str),     // Attr on subject
    Object(&'a str),      // Attr on object
    Const(ReflValue<'a>), // literal/hardcoded value
}

/// A single policy clause: `<left operand> <operator> <right operand>`
pub struct Clause<'a> {
    pub left: Operand<'a>,
    pub op: Op,
    pub right: Operand<'a>,
}

/// A disjunction (OR-group) of clauses. Evaluates to true if any clause inside is true.
pub struct AnyOf<'a>(pub Vec<Clause<'a>>); // true if **one** Clause is true

/// The complete policy: a conjunction (AND-chain) of OR-groups.
pub struct Rules<'a>(pub Vec<AnyOf<'a>>);

// ==================== Parser ==================== //

impl<'a> TryFrom<&'a str> for Operand<'a> {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let s = s.trim();
        if let Some(stripped) = s.strip_prefix("subject.") {
            Ok(Operand::Subject(stripped))
        } else if let Some(stripped) = s.strip_prefix("object.") {
            Ok(Operand::Object(stripped))
        } else if s.starts_with('\'') && s.ends_with('\'') {
            let val = &s[1..s.len() - 1];
            Ok(Operand::Const(ReflValue::Str(val)))
        } else if let Ok(int) = s.parse::<i32>() {
            Ok(Operand::Const(ReflValue::Int(int)))
        } else {
            Err(format!("Invalid operand: {s}"))
        }
    }
}

impl<'a> TryFrom<&'a str> for Clause<'a> {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        // naive split — improve later with proper tokenizer
        let ops = ["==", "!=", ">=", "<=", ">", "<"];
        for op in ops {
            if let Some(i) = s.find(op) {
                let (left, right) = s.split_at(i);
                let right = &right[op.len()..];

                return Ok(Clause {
                    left: Operand::try_from(left)?,
                    op: op.parse()?,
                    right: Operand::try_from(right)?,
                });
            }
        }
        Err("No valid operator found".into())
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
        let groups = s
            .split("],") // crude but works for your format
            .map(AnyOf::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Rules(groups))
    }
}
