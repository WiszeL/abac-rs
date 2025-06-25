use abac_rs::{Rule, evaluate};
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
    {
        "left_rule": { "Subject": "age" },
        "operator": "Less",
        "right_rule": { "Literal": 23 }
    }
    "#;
    let rule = serde_json::from_str::<Rule>(json_rule).unwrap();

    // User as Subject Entity
    let user = User {
        name: "WiszeL".into(),
        age: 35,
        address: "Indonesia".into(),
    };

    // Task as Object Entity
    let task = Task {
        name: "kerjaan".into(),
        deadline: "besok".into(),
        owner: "WiszeL".into(),
    };

    // The actual evaluate
    let allowed = evaluate(&user, &task, rule).unwrap();

    println!("User is allowed to do the task: {allowed}")
}
