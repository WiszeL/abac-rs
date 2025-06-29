use std::collections::HashMap;

use macros::Entity;
use serde_value::Value;

use crate::{Operator, Rule, Rules, SideRule, evaluate, which_to_evaluate};

#[test]
fn which_to_evaluate_test() {
    // ##### Arrange ##### //
    let mut subject = HashMap::new();
    subject.insert("age".to_string(), Value::I32(21));
    subject.insert("name".to_string(), Value::String("WiszeL".into()));

    let mut object = HashMap::new();
    object.insert("owner".to_string(), Value::String("WiszeL".into()));

    let literal = Value::Bool(true);

    // ##### Act & Assert ##### //

    /* -----------------------------------------------
     * Case 01 – Subject field exists
     * ----------------------------------------------- */
    let binding = SideRule::Subject("age".into());
    let result = which_to_evaluate(&subject, &object, &binding);
    assert!(
        matches!(result, Ok(Value::I32(21))),
        "Case 01: should return subject field 'age'"
    );

    /* -----------------------------------------------
     * Case 02 – Object field exists
     * ----------------------------------------------- */
    let binding = SideRule::Object("owner".into());
    let result = which_to_evaluate(&subject, &object, &binding);
    assert!(
        matches!(result, Ok(Value::String(s)) if s == "WiszeL"),
        "Case 02: should return object field 'owner'"
    );

    /* -----------------------------------------------
     * Case 03 – Literal value returned directly
     * ----------------------------------------------- */
    let binding = SideRule::Literal(literal.clone());
    let result = which_to_evaluate(&subject, &object, &binding);
    assert!(
        matches!(result, Ok(val) if *val == literal),
        "Case 03: should return literal directly"
    );

    /* -----------------------------------------------
     * Case 04 – Missing field returns error
     * ----------------------------------------------- */
    let binding = SideRule::Subject("not_found".into());
    let result = which_to_evaluate(&subject, &object, &binding);
    assert!(
        result.is_err(),
        "Case 04: should return error for missing subject field"
    );
}

#[derive(Entity)]
struct User {
    name: String,
    age: u64,
}

#[derive(Entity)]
struct Task {
    owner: String,
}

#[tokio::test]
async fn evaluate_test() {
    // ##### Arrange ##### //
    let user = User {
        name: "WiszeL".into(),
        age: 21,
    };
    let task = Task {
        owner: "WiszeL".into(),
    };

    // ##### Act & Assert ##### //

    /* -----------------------------------------------
     * Case 01 – Subject.age >= 18 → true
     * ----------------------------------------------- */
    let rules = Rules(vec![vec![Rule {
        left_rule: SideRule::Subject("age".into()),
        operator: Operator::GreaterEqual,
        right_rule: SideRule::Literal(Value::U64(18)),
    }]]);

    let result = evaluate(&user, &task, &rules);
    assert_eq!(result.unwrap(), true, "Case 01: age >= 18 should pass");

    /* -----------------------------------------------
     * Case 02 – Subject.name == Object.owner → true
     * ----------------------------------------------- */
    let rules = Rules(vec![vec![Rule {
        left_rule: SideRule::Subject("name".into()),
        operator: Operator::Equal,
        right_rule: SideRule::Object("owner".into()),
    }]]);

    let result = evaluate(&user, &task, &rules);
    assert_eq!(result.unwrap(), true, "Case 02: name == owner should pass");

    /* -----------------------------------------------
     * Case 03 – Subject.age > 30 → false
     * ----------------------------------------------- */
    let rules = Rules(vec![vec![Rule {
        left_rule: SideRule::Subject("age".into()),
        operator: Operator::Greater,
        right_rule: SideRule::Literal(Value::U64(30)),
    }]]);

    let result = evaluate(&user, &task, &rules);
    assert_eq!(result.unwrap(), false, "Case 03: age > 30 should fail");

    /* -----------------------------------------------
     * Case 04 – OR group: (age > 30 OR name == owner) → true
     * ----------------------------------------------- */
    let rules = Rules(vec![vec![
        Rule {
            left_rule: SideRule::Subject("age".into()),
            operator: Operator::Greater,
            right_rule: SideRule::Literal(Value::U64(30)),
        },
        Rule {
            left_rule: SideRule::Subject("name".into()),
            operator: Operator::Equal,
            right_rule: SideRule::Object("owner".into()),
        },
    ]]);

    let result = evaluate(&user, &task, &rules);
    assert_eq!(result.unwrap(), true, "Case 04: OR group should pass");

    /* -----------------------------------------------
     * Case 05 – AND group fail: (age >= 18) AND (name == 'SomeoneElse') → false
     * ----------------------------------------------- */
    let rules = Rules(vec![
        vec![Rule {
            left_rule: SideRule::Subject("age".into()),
            operator: Operator::GreaterEqual,
            right_rule: SideRule::Literal(Value::U64(18)),
        }],
        vec![Rule {
            left_rule: SideRule::Subject("name".into()),
            operator: Operator::Equal,
            right_rule: SideRule::Literal(Value::String("SomeoneElse".into())),
        }],
    ]);

    let result = evaluate(&user, &task, &rules);
    assert_eq!(result.unwrap(), false, "Case 05: AND block should fail");
}
