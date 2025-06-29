use std::path::PathBuf;

use macros::Entity;
use serde_value::Value;
use uuid::Uuid;

use crate::{Engine, EntityAdapter, LoadResult, Operator, Rule, Rules, SideRule};

#[derive(Entity, Default)]
struct Task {
    owner: String,
}

impl EntityAdapter for Task {
    type Provider = PathBuf;

    // Assume it loads data from database
    fn load_data(_: uuid::Uuid, _: &Self::Provider) -> LoadResult<Self> {
        Box::pin(async move {
            Ok(Self {
                owner: "WiszeL".into(),
            })
        })
    }
}

#[derive(Entity)]
struct User {
    name: String,
}

#[test]
fn register_adapter_test() {
    // ##### Arrange ##### //
    let path_buf = PathBuf::new();

    // ##### Act ##### //
    let engine = Engine::new().with_provider(path_buf);

    // ##### Assert ##### //
    assert_eq!(engine.providers.len(), 1, "Should really add provider");
}

#[test]
fn with_provider_test() {
    // ##### Arrange ##### //
    let path_buf = PathBuf::new();

    // ##### Act ##### //
    let engine = Engine::new()
        .with_provider(path_buf)
        .register_adapter::<Task>("task");

    // ##### Assert ##### //
    assert_eq!(engine.providers.len(), 1, "Should really add provider");
    assert_eq!(engine.adapters.len(), 1, "Should really add adapter");
}

#[tokio::test]
async fn evaluate_with_subject_test() {
    // ##### Arrange ##### //
    let path_buf = PathBuf::new();

    let w_rsc_rule = vec![Rule {
        left_rule: SideRule::Subject("name".into()),
        operator: Operator::Equal,
        right_rule: SideRule::Object("owner".into()),
    }];
    let wo_rsc_rule = vec![Rule {
        left_rule: SideRule::Subject("name".into()),
        operator: Operator::Equal,
        right_rule: SideRule::Literal(Value::String("WiszeL".into())),
    }];

    let w_rsc_rules = Rules(vec![w_rsc_rule]);
    let wo_rsc_rules = Rules(vec![wo_rsc_rule]);

    let engine = Engine::new()
        .with_provider(path_buf)
        .register_adapter::<Task>("task");

    // ##### Act ##### //
    let subject = User {
        name: "WiszeL".into(),
    };

    // 1. With Resource
    let w_result = engine
        .evaluate_with_subject(&subject, "task", Some(Uuid::nil()), &w_rsc_rules)
        .await;

    // 2. Without Resource
    let wo_result = engine
        .evaluate_with_subject(&subject, "task", None, &wo_rsc_rules)
        .await;

    // ##### Assert ##### //
    // 1. With Resource
    assert!(w_result.is_ok(), "Evaluate shouldn't throw any error!");
    assert!(w_result.unwrap(), "Evaluate should be true!");

    // 2. Without Resource
    assert!(wo_result.is_ok(), "Evalute shouldn't throw any error!");
    assert!(wo_result.unwrap(), "Evaluate should be true!");
}
