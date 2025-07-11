use std::path::PathBuf;

use macros::Entity;
use serde_value::Value;
use uuid::Uuid;

use crate::{Engine, EntityAdapter, EvaluateEntity, LoadResult, Operator, Rule, Rules, SideRule};

#[derive(Entity, Default)]
struct Task {
    owner: String,
}

impl EntityAdapter for Task {
    type Provider = PathBuf;

    // Assume it loads data from database
    fn load_data(_: Uuid, _: &Self::Provider) -> LoadResult<Self> {
        Box::pin(async move {
            Ok(Self {
                owner: "WiszeL".into(),
            })
        })
    }
}

#[derive(Entity, Default)]
struct User {
    name: String,
}

impl EntityAdapter for User {
    type Provider = PathBuf;

    // Assume it loads data from database
    fn load_data(_: Uuid, _: &Self::Provider) -> LoadResult<Self> {
        Box::pin(async move {
            Ok(Self {
                name: "WiszeL".into(),
            })
        })
    }
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
        left: SideRule::Subject("name".into()),
        operator: Operator::Equal,
        right: SideRule::Object("owner".into()),
    }];
    let wo_rsc_rule = vec![Rule {
        left: SideRule::Subject("name".into()),
        operator: Operator::Equal,
        right: SideRule::Literal(Value::String("WiszeL".into())),
    }];

    let w_rsc_rules = Rules(vec![w_rsc_rule]);
    let wo_rsc_rules = Rules(vec![wo_rsc_rule]);

    let engine = Engine::new()
        .with_provider(path_buf)
        .register_adapter::<User>("user")
        .register_adapter::<Task>("task");

    // ##### Act ##### //
    let evaluate_subject = EvaluateEntity::new("user", Uuid::nil().into());

    // 1. With Resource
    let evaluate_resource = EvaluateEntity::new("task", Uuid::nil().into());
    let w_result = engine
        .evaluate(evaluate_subject.clone(), evaluate_resource, &w_rsc_rules)
        .await;

    // 2. Without Resource
    let evaluate_resource = EvaluateEntity::new("task", Uuid::nil().into());
    let wo_result = engine
        .evaluate(evaluate_subject, evaluate_resource, &wo_rsc_rules)
        .await;

    // ##### Assert ##### //
    // 1. With Resource
    assert!(w_result.is_ok(), "Evaluate shouldn't throw any error!");
    assert!(w_result.unwrap(), "Evaluate should be true!");

    // 2. Without Resource
    assert!(wo_result.is_ok(), "Evalute shouldn't throw any error!");
    assert!(wo_result.unwrap(), "Evaluate should be true!");
}
