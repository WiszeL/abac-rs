use serde::Serialize;
use serde_value::Value;

use crate::construct_entity;

#[derive(Serialize)]
struct Entity {
    name: String,
    age: i32,
}

#[test]
fn construct_entity_test() {
    // ##### Arrange ##### //
    let entity = Entity {
        name: "WiszeL".into(),
        age: 21,
    };

    // ##### Act ##### //
    let result = construct_entity(&entity);

    // ##### Assert ##### //
    // 1. Shouldn't even be failed
    assert!(result.is_ok(), "Shouldn't even be failed");

    let entity_result = result.unwrap();

    // 2. Should give Some if exists
    assert!(
        matches!(entity_result.get("name"), Some(Value::String(_))),
        "Should give Some with String if exists"
    );
    assert!(
        matches!(entity_result.get("age"), Some(Value::I32(_))),
        "Should give Some with String if exists"
    );

    // 3. Should give NONE if not exists
    assert!(
        matches!(entity_result.get("Not existed"), None),
        "Should give NONE if not exists"
    );
}
