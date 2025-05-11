use abac_rs::{AbacEntity, error::Error, evaluate_rules};

// --- helper structs implementing AbacEntity ------------------------
#[derive(AbacEntity)]
struct User {
    id: i32,
    role: String,
    department: String,
}

#[derive(AbacEntity)]
struct File {
    owner_id: i32,
    tag: String,
}

// Success: (admin OR owner) AND department matches
#[test]
fn policy_allows_admin_in_department() {
    let rules = r#"
        [ subject.role == 'admin', object.owner_id == subject.id ],
        [ subject.department == 'informatics' ]
    "#;

    let user = User {
        id: 1,
        role: "admin".into(),
        department: "informatics".into(),
    };
    let file = File {
        owner_id: 42,
        tag: "draft".into(),
    };
    assert!(evaluate_rules(rules, &user, &file).unwrap());
}

// Denied: none of OR‑group clauses pass
#[test]
fn policy_denies_guest_wrong_department() {
    let rules = r#"
        [ subject.role == 'admin', object.owner_id == subject.id ],
        [ subject.department == 'informatics' ]
    "#;

    let user = User {
        id: 2,
        role: "guest".into(),
        department: "design".into(),
    };
    let file = File {
        owner_id: 99,
        tag: "draft".into(),
    };
    assert!(!evaluate_rules(rules, &user, &file).unwrap());
}

// Error: unknown field referenced in policy
#[test]
fn unknown_field_error() {
    let rules = "[ subject.nonexistent == 'x' ]";

    let user = User {
        id: 3,
        role: "guest".into(),
        department: "informatics".into(),
    };
    let file = File {
        owner_id: 3,
        tag: "draft".into(),
    };

    match evaluate_rules(rules, &user, &file) {
        Err(Error::UnknownField(_)) => {} // expected
        other => panic!("expected UnknownField error, got {other:?}"),
    }
}

// Error: type mismatch (string literal vs int field)
#[test]
fn type_mismatch_error() {
    let rules = "[ subject.id == '42' ]"; // id is i32, literal is str

    let user = User {
        id: 42,
        role: "guest".into(),
        department: "informatics".into(),
    };
    let file = File {
        owner_id: 0,
        tag: "draft".into(),
    };

    match evaluate_rules(rules, &user, &file) {
        Err(Error::TypeMismatch { .. }) => {} // expected
        other => panic!("expected TypeMismatch error, got {other:?}"),
    }
}

// Error: parse failure due to invalid operator
#[test]
fn parse_error_invalid_operator() {
    let rules = "[ subject.role === 'admin' ]"; // triple equals not supported

    let user = User {
        id: 0,
        role: "admin".into(),
        department: "informatics".into(),
    };
    let file = File {
        owner_id: 0,
        tag: "draft".into(),
    };

    match evaluate_rules(rules, &user, &file) {
        Err(Error::Parse(_)) => {} // expected
        other => panic!("expected Parse error, got {other:?}"),
    }
}
