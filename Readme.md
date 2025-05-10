# abac-rs 🛡️ — Attribute-Based Access Control in Pure Rust

**abac-rs** is a lightweight, zero-dependency attribute-based access control engine designed to be practical, embeddable, and dead simple to use.

Define your policies in string form, implement a small reflection trait, and let the engine do the evaluation for you. Inspired by real-world ABAC usage like AWS IAM, Rego (OPA), and Keycloak — but kept minimal and ergonomic for Rust.

---

## ✨ Features

- ✅ Human-readable rule strings (`subject.role == 'admin'`)
- ✅ Supports `AND` of `OR` groups: `[A, B], [C]` → `(A OR B) AND C`
- ✅ Dynamic field access via `AbacEntity` trait
- ✅ Supports comparison on `String`, `i32`, `f32`, `bool`
- ✅ Custom error types for integration (`Parse`, `UnknownField`, `TypeMismatch`)
- 🌐 Easily extensible (e.g. time, IP, env conditions)

---

## 🔧 Example

```rust
use abac_rs::{evaluate_rules, AbacEntity};

#[derive(Reflection)]
struct User {
    role: String,
    department: String,
    id: i32,
}

#[derive(Reflection)]
struct File {
    owner_id: i32,
    tag: String,
}

let rules = r#"
    [ subject.role == 'admin', object.owner_id == subject.id ],
    [ subject.department == 'informatics' ]
"#;

let user = User { role: "admin".into(), department: "informatics".into(), id: 42 };
let file = File { owner_id: 42, tag: "draft".into() };

let allowed = evaluate_rules(rules, &user, &file)?;
assert!(allowed);
```

---

## 📊 Rule Syntax

- **Operands**: `subject.<field>`, `object.<field>`, or literal values (`'string'`, `42`, `true`)
- **Operators**: `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Groups**: Use brackets `[]` for OR-groups. Comma-separated groups are ANDed.

```txt
[ subject.role == 'admin', object.owner_id == subject.id ],
[ subject.department == 'informatics' ]
```

---

## 🔌 Integration

- Implement `AbacEntity` on your types.
- Use `#[derive(AbacEntity)]` (re-exported from `reflect-rs`).
- Evaluate your rules with `evaluate_rules()`.

You only need:
```toml
abac-rs = "*"
```
No need to add `reflect-rs` manually.

---

## 🛠️ Extending This Crate

The engine is designed to be minimal but hackable:

| Idea | How |
|------|-----|
| Support `env.time >= 8` | Add `Operand::Env`, pass a context map to `resolve_operand()` |
| Deny-override logic | Add `Effect` enum to wrap `Rules` |
| Rule caching / compiled trees | Pre-parse to `Rules` and reuse them |
| Refl customization | Extend `reflect-rs` for extra type support |

---

## 🤝 Contributing

This crate is built for the community. Feel free to open issues, PRs, or just share use cases!

Whether you're using `abac-rs` for internal authorization, WASM apps, games, or cloud control — **contributions are welcome** 💙

---

## 🙋 Why Use abac-rs?

Not every project needs a heavyweight solution like AWS IAM, OPA (Rego), or Keycloak. If you're building:

- 🚀 Lightweight Rust backends or microservices
- 🧰 Internal tools or admin dashboards
- 🧩 Plugins or embedded ABAC inside games, CLI tools, or engines
- 🔐 Role-based access inside isolated apps

…then `abac-rs` gives you exactly what you need: readable policies, clean trait-based design, and no runtime overhead.

It's built for clarity, control, and hackability — ideal for devs who want a native, flexible ABAC engine without an entire ecosystem strapped on top.

## License

MIT or Apache-2.0 — pick what works best for you.
