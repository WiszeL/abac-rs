use serde_value::Value;

use crate::{Operator, Rule, SideRule};

#[test]
fn rule_01_equal_subject_vs_object() {
    // ##### Arrange ##### //
    let json_rule = r#"
        {
            "left":  { "Subject": "name" },
            "operator":   "Equal",
            "right": { "Object":  "owner" }
        }
        "#;

    // ##### Act ##### //
    let rule: Rule = serde_json::from_str(json_rule).expect("Should deserialize Rule");

    // ##### Assert ##### //
    assert!(matches!(rule.left,
                         SideRule::Subject(ref s) if s == "name"));
    assert!(matches!(rule.right,
                         SideRule::Object(ref s) if s == "owner"));
    assert!(matches!(rule.operator, Operator::Equal));
}

#[test]
fn rule_02_greater_subject_vs_literal() {
    // ##### Arrange ##### //
    let json_rule = r#"
        {
            "left":  { "Subject": "age" },
            "operator":   "Greater",
            "right": { "Literal": 18 }
        }
        "#;

    // ##### Act ##### //
    let rule: Rule = serde_json::from_str(json_rule).expect("Should deserialize Rule");

    // ##### Assert ##### //
    assert!(matches!(rule.left,
                         SideRule::Subject(ref s) if s == "age"));
    assert!(matches!(rule.right,
                         SideRule::Literal(ref v) if *v == Value::U64(18)));
    assert!(matches!(rule.operator, Operator::Greater));
}

#[test]
fn rule_03_less_subject_vs_literal() {
    // ##### Arrange ##### //
    let json_rule = r#"
        {
            "left":  { "Subject": "priority" },
            "operator":   "Less",
            "right": { "Literal": 5 }
        }
        "#;

    // ##### Act ##### //
    let rule: Rule = serde_json::from_str(json_rule).expect("Should deserialize Rule");

    // ##### Assert ##### //
    assert!(matches!(rule.left,
                         SideRule::Subject(ref s) if s == "priority"));
    assert!(matches!(rule.right,
                         SideRule::Literal(ref v) if *v == Value::U64(5)));
    assert!(matches!(rule.operator, Operator::Less));
}

#[test]
fn rule_04_greater_equal_subject_vs_literal() {
    // ##### Arrange ##### //
    let json_rule = r#"
        {
            "left":  { "Subject": "score" },
            "operator":   "GreaterEqual",
            "right": { "Literal": 90 }
        }
        "#;

    // ##### Act ##### //
    let rule: Rule = serde_json::from_str(json_rule).expect("Should deserialize Rule");

    // ##### Assert ##### //
    assert!(matches!(rule.left,
                         SideRule::Subject(ref s) if s == "score"));
    assert!(matches!(rule.right,
                         SideRule::Literal(ref v) if *v == Value::U64(90)));
    assert!(matches!(rule.operator, Operator::GreaterEqual));
}

#[test]
fn rule_05_less_equal_subject_vs_literal() {
    // ##### Arrange ##### //
    let json_rule = r#"
        {
            "left":  { "Subject": "cost" },
            "operator":   "LessEqual",
            "right": { "Literal": 1000 }
        }
        "#;

    // ##### Act ##### //
    let rule: Rule = serde_json::from_str(json_rule).expect("Should deserialize Rule");

    // ##### Assert ##### //
    assert!(matches!(rule.left,
                         SideRule::Subject(ref s) if s == "cost"));
    assert!(matches!(rule.right,
                         SideRule::Literal(ref v) if *v == Value::U64(1000)));
    assert!(matches!(rule.operator, Operator::LessEqual));
}
