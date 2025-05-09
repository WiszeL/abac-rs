mod operator;

pub mod rules;

use operator::cmp;
use reflect_rs::{ReflValue, Reflection};
use rules::{Clause, Operand, Rules};

#[inline]
pub fn evaluate_rules(rules: &Rules, sub: &impl Reflection, obj: &impl Reflection) -> bool {
    rules.0.iter().all(|any| {
        // AND over groups
        any.0.iter().any(|c| eval_clause(c, sub, obj)) // OR  inside a group
    })
}

fn resolve_operand<'a>(
    sub: &'a impl Reflection,
    obj: &'a impl Reflection,
    operand: &'a Operand,
) -> Option<ReflValue> {
    match operand {
        Operand::Subject(attr) => sub.get_field(&attr),
        Operand::Object(attr) => obj.get_field(&attr),
        Operand::Const(v) => Some(v.clone()),
    }
}

fn eval_clause(c: &Clause, sub: &impl Reflection, obj: &impl Reflection) -> bool {
    let lhs = resolve_operand(sub, obj, &c.left);
    let rhs = resolve_operand(sub, obj, &c.right);

    match (lhs, rhs) {
        (Some(ReflValue::Str(l)), Some(ReflValue::Str(r))) => cmp(l, r, &c.op),
        (Some(ReflValue::Int(l)), Some(ReflValue::Int(r))) => cmp(l, r, &c.op),
        (Some(ReflValue::Float(l)), Some(ReflValue::Float(r))) => cmp(l, r, &c.op),
        (Some(ReflValue::Bool(l)), Some(ReflValue::Bool(r))) => cmp(l, r, &c.op),
        _ => false, // type mismatch or missing field
    }
}
