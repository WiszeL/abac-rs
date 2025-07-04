use serde::{Deserialize, Serialize};
use serde_value::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum SideRule {
    Subject(/* Field Name */ String),
    Object(/* Field Name */ String),
    Literal(/* Literal Value */ Value),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum Operator {
    Equal,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Rule {
    pub(crate) left: SideRule,
    pub(crate) operator: Operator,
    pub(crate) right: SideRule,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rules(pub(crate) Vec<Vec<Rule>>);
