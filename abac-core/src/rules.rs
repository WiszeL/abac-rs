use serde::{Deserialize, Serialize};
use serde_value::Value;

#[derive(Serialize, Deserialize)]
pub(crate) enum SideRule {
    Subject(/* Field Name */ String),
    Object(/* Field Name */ String),
    Literal(/* Literal Value */ Value),
}

#[derive(Serialize, Deserialize)]
pub(crate) enum Operator {
    Equal,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Rule {
    pub(crate) left_rule: SideRule,
    pub(crate) operator: Operator,
    pub(crate) right_rule: SideRule,
}

#[derive(Serialize, Deserialize)]
pub struct Rules(pub(crate) Vec<Vec<Rule>>);
