use abac_rs::{evaluate_rules, rules::Rules};
use reflect_rs::Reflection;

// ===== mock types that derive Reflection ===============================
#[derive(Reflection)]
struct User {
    name: String,
    role: String,
    department: String,
    id: i32,
}

#[derive(Reflection)]
struct File {
    owner_id: i32,
    tag: String,
}
// =======================================================================

fn main() -> Result<(), String> {
    // (role == "admin" OR owner_id == subject.id)  AND  (department == "informatics")
    let raw = r#"
        [ subject.role == 'admin', object.owner_id == subject.id ],
        [ subject.department == 'informatics' ]
    "#;
    let rules: Rules = raw.parse()?; // FromStr chain

    // -------- two users -----------------------------------------------
    let admin = User {
        name: "Han".into(),
        role: "guest".into(),             // passes first OR‑group
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
    println!("Han allowed? {}", evaluate_rules(&rules, &admin, &doc)); // true
    println!("Leia allowed? {}", evaluate_rules(&rules, &guest, &doc)); // false

    Ok(())
}
