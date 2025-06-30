use serde::Deserialize;
use serde_value::Value;

#[derive(Clone, Deserialize)]
pub(crate) enum SideRule {
    Subject(/* Field Name */ String),
    Object(/* Field Name */ String),
    Literal(/* Literal Value */ Value),
}

#[derive(Clone, Deserialize)]
pub(crate) enum Operator {
    Equal,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Clone, Deserialize)]
pub(crate) struct Rule {
    pub(crate) left_rule: SideRule,
    pub(crate) operator: Operator,
    pub(crate) right_rule: SideRule,
}

#[derive(Clone, Deserialize)]
pub struct Rules(pub(crate) Vec<Vec<Rule>>);
