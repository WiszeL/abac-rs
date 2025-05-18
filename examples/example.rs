use abac_rs::{Entity, evaluate_rules};
use uuid::Uuid;

// ─── mock entities ──────────────────────────────────────────────────
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
// ────────────────────────────────────────────────────────────────────

fn main() -> Result<(), String> {
    // (role == "admin" OR object.owner_id == subject.id)  AND  department == "informatics"
    let rules = r#"
        [ subject.role == 'admin', object.owner_id == subject.id ],
        [ subject.department == 'informatics' ]
    "#;

    // ── create two users ────────────────────────────────────────────
    let han_id = Uuid::new_v4();
    let leia_id = Uuid::new_v4();

    let han = User {
        name: "Han".into(),
        role: "guest".into(),             // NOT admin
        department: "informatics".into(), // passes 2nd AND-group
        id: han_id,
    };

    let leia = User {
        name: "Leia".into(),
        role: "guest".into(),             // NOT admin
        department: "informatics".into(), // passes 2nd AND-group
        id: leia_id,
    };

    let joko = User {
        name: "Joko".into(),
        role: "admin".into(),             // NOT admin
        department: "informatics".into(), // passes 2nd AND-group
        id: han_id,
    };

    // ── one file owned by Han ───────────────────────────────────────
    let doc = File {
        owner_id: han_id,
        tag: "draft".into(),
    };

    // ── evaluate ────────────────────────────────────────────────────
    println!(
        "Han allowed?  {}",
        evaluate_rules(rules, &han, Some(&doc)).unwrap() // Some(&dyn Entity)
    ); // true  (owner_id matches Han)

    println!(
        "Leia allowed? {}",
        evaluate_rules(rules, &leia, Some(&doc)).unwrap()
    ); // false (not admin, not owner)

    // ── type-level call (no object) just as reference ───────────────
    println!(
        "Joko allowed to LIST?  {}",
        evaluate_rules(rules, &joko, None).unwrap()
    ); // true

    Ok(())
}
