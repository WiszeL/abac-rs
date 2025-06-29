use serde_value::Value;

use crate::{Entity, EntityValue, Error, Operator, Rules, SideRule};

/// Which to evaluate based on the left/right rule
pub(crate) fn which_to_evaluate<'a>(
    subject: &'a EntityValue,
    object: &'a EntityValue,
    side_rule: &'a SideRule,
) -> Result<&'a Value, Error> {
    match side_rule {
        SideRule::Subject(field_name) => subject.get(field_name).ok_or(Error::FieldNotFound),
        SideRule::Object(field_name) => object.get(field_name).ok_or(Error::FieldNotFound),
        SideRule::Literal(value) => Ok(value),
    }
}

/// The actual
pub fn evaluate(subject: &dyn Entity, object: &dyn Entity, rules: &Rules) -> Result<bool, Error> {
    // Construct Entity
    let subject = subject.into_value()?;
    let object = object.into_value()?;

    rules.0.iter().try_fold(true, |acc, r_and| {
        if !acc {
            return Ok(false); // short-circuit outer AND
        }

        let or_result = r_and.iter().try_fold(false, |acc, rule| {
            if acc {
                return Ok::<_, Error>(true); // short-circuit inner OR
            }

            let left = which_to_evaluate(&subject, &object, &rule.left_rule)?;
            let right = which_to_evaluate(&subject, &object, &rule.right_rule)?;

            let pass = match rule.operator {
                Operator::Equal => left == right,
                Operator::Greater => left > right,
                Operator::Less => left < right,
                Operator::GreaterEqual => left >= right,
                Operator::LessEqual => left <= right,
            };

            Ok(pass)
        })?;

        Ok(or_result)
    })
}
