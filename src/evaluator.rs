use std::collections::HashMap;

use serde::Serialize;
use serde_value::Value;

use crate::{Error, Operator, Rules, SideRule};

type Entity = HashMap<String, Value>;

/// Serializing any struct that derives Serialize into ABAC Entity
pub(crate) fn construct_entity<T: Serialize>(entity: &T) -> Result<Entity, Error> {
    let value = serde_value::to_value(entity)?;
    let mut ett = HashMap::new();

    if let Value::Map(map) = value {
        ett = map
            .into_iter()
            .filter_map(|(k, v)| {
                if let Value::String(name) = k {
                    Some((name, v))
                } else {
                    None
                }
            })
            .collect();
    }

    Ok(ett)
}

/// Which to evaluate based on the left/right rule
fn which_to_evaluate<'a>(
    subject: &'a Entity,
    object: &'a Entity,
    side_rule: &'a SideRule,
) -> Result<&'a Value, Error> {
    match side_rule {
        SideRule::Subject(field_name) => subject.get(field_name).ok_or(Error::FieldNotFound),
        SideRule::Object(field_name) => object.get(field_name).ok_or(Error::FieldNotFound),
        SideRule::Literal(value) => Ok(value),
    }
}

/// The actual
pub fn evaluate<S, O>(subject: &S, object: &O, rules: Rules) -> Result<bool, Error>
where
    S: Serialize,
    O: Serialize,
{
    // Construct Entity
    let subject = construct_entity(subject)?;
    let object = construct_entity(object)?;

    for r in &rules.0 {
        let left = which_to_evaluate(&subject, &object, &r.left_rule)?;
        let right = which_to_evaluate(&subject, &object, &r.right_rule)?;

        let pass = match r.operator {
            Operator::Equal => left == right,
            Operator::Greater => left > right,
            Operator::Less => left < right,
            Operator::GreaterEqual => left >= right,
            Operator::LessEqual => left <= right,
        };

        if !pass {
            return Ok(false);
        }
    }

    Ok(true)
}
