use serde::Deserialize;
use serde_value::Value;

#[derive(Deserialize)]
pub(crate) enum SideRule {
    Subject(/* Field Name */ String),
    Object(/* Field Name */ String),
    Literal(/* Literal Value */ Value),
}

#[derive(Deserialize)]
pub(crate) enum Operator {
    Equal,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Deserialize)]
pub(crate) struct Rule {
    pub(crate) left_rule: SideRule,
    pub(crate) operator: Operator,
    pub(crate) right_rule: SideRule,
}

#[derive(Deserialize)]
pub struct Rules(pub(crate) Vec<Rule>);
