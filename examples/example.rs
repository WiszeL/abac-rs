use abac_rs::{Entity, evaluate_rules};
use uuid::Uuid;

// ===== mock types that derive Entity ===============================
#[derive(Entity)]
struct User {
    name: String,
    role: String,
    department: String,
    id: Uuid,
}

#[derive(Entity)]
struct File {
    owner_id: Uuid,
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
    let han_id = Uuid::new_v4();
    let leia_id = Uuid::new_v4();
    let han = User {
        name: "Han".into(),
        role: "guest".into(),             // passes first OR‑group
        department: "informatics".into(), // passes second group
        id: han_id,
    };

    let leia = User {
        name: "Leia".into(),
        role: "guest".into(),             // fails first OR‑group
        department: "informatics".into(), // fails second group
        id: leia_id,
    };

    // -------- one file -------------------------------------------------
    let doc = File {
        owner_id: leia_id,
        tag: "draft".into(),
    };

    // -------- evaluate -------------------------------------------------
    println!(
        "Han allowed? {}",
        evaluate_rules(rules, &han, &doc).unwrap()
    ); // true
    println!(
        "Leia allowed? {}",
        evaluate_rules(rules, &leia, &doc).unwrap()
    ); // false

    Ok(())
}
