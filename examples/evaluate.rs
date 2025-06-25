use abac_rs::{Rules, evaluate};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
    age: u64,
    address: String,
}

#[derive(Serialize)]
struct Task {
    name: String,
    deadline: String,
    owner: String,
}

fn main() {
    // Assume this JSON is from PGSQL DB JSONB
    let json_rule = r#"
    [
        {
            "left_rule": { "Subject": "age" },
            "operator": "Less",
            "right_rule": { "Literal": 23 }
        },
        {
            "left_rule": { "Subject": "name" },
            "operator": "Equal",
            "right_rule": { "Object": "owner" }
        }
    ]
    "#;
    let rule = serde_json::from_str::<Rules>(json_rule).unwrap();

    // User as Subject Entity
    let user = User {
        name: "WiszeL".into(),
        age: 17,
        address: "Indonesia".into(),
    };

    // Task as Object Entity
    let task = Task {
        name: "kerjaan".into(),
        deadline: "besok".into(),
        owner: "WiszL".into(),
    };

    // The actual evaluate
    let allowed = evaluate(&user, &task, rule).unwrap();

    println!("User is allowed to do the task: {allowed}")
}
