use crate::{Rule, SideRule};

#[test]
fn rules_test() {
    // ##### Arrange ##### //
    let json_rule = r#"
    {
        "left_rule": { "Subject": "name" },
        "operator": "Equal",
        "right_rule": { "Object": "owner" }
    }
    "#;

    // ##### Act ##### //
    let result = serde_json::from_str::<Rule>(json_rule);

    // ##### Assert ##### //
    assert!(result.is_ok(), "Shouldn't throw error");

    let parsed_result = result.unwrap();

    assert!(
        matches!(parsed_result.left_rule, SideRule::Subject(_)),
        "Should have proper deserialization"
    );
}
