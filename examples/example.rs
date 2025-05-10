use abac_rs::{AbacEntity, evaluate_rules};

// ===== mock types that derive AbacEntity ===============================
#[derive(AbacEntity)]
struct User {
    name: String,
    role: String,
    department: String,
    id: i32,
}

#[derive(AbacEntity)]
struct File {
    owner_id: i32,
    tag: String,
}
// =======================================================================

fn main() -> Result<(), String> {
    // (role == "admin" OR owner_id == subject.id)  AND  (department == "informatics")
    let rules = r#"
        [ subject.role == 'admin', object.owner_id == subject.id ],
        [ subject.department == 'informatics' ]
    "#;

    // -------- two users -----------------------------------------------
    let admin = User {
        name: "Han".into(),
        role: "admin".into(),             // passes first OR‑group
        department: "informatics".into(), // passes second group
        id: 7,
    };

    let guest = User {
        name: "Leia".into(),
        role: "guest".into(),             // fails first OR‑group
        department: "informatics".into(), // fails second group
        id: 42,
    };

    // -------- one file -------------------------------------------------
    let doc = File {
        owner_id: 42,
        tag: "draft".into(),
    };

    // -------- evaluate -------------------------------------------------
    println!(
        "Han allowed? {}",
        evaluate_rules(rules, &admin, &doc).unwrap()
    ); // true
    println!(
        "Leia allowed? {}",
        evaluate_rules(rules, &guest, &doc).unwrap()
    ); // false

    Ok(())
}
