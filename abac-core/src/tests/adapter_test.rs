use std::{any::Any, path::PathBuf};

use macros::Entity;
use serde_value::Value;
use uuid::Uuid;

use crate::{DynAdapter, EntityAdapter, FutPin};

#[derive(Entity, Default)]
struct Task {
    owner: String,
}

impl EntityAdapter for Task {
    type Provider = PathBuf;

    // Assume it loads data from database
    fn load_data(_: uuid::Uuid, _: &Self::Provider) -> FutPin<Self> {
        Box::pin(async move {
            Self {
                owner: "WiszeL".into(),
            }
        })
    }
}

#[tokio::test]
async fn dyn_adapter_test() {
    // ##### Arrange ##### //
    let dyn_adapter = Task::default();
    let path_buf = PathBuf::new();

    // ##### Act ##### //
    let adapter_provider = dyn_adapter.provider_type();
    let adapter_load = dyn_adapter.load(Uuid::nil(), &path_buf).await;

    // ##### Arrange ##### //
    assert_eq!(
        adapter_provider,
        path_buf.type_id(),
        "Should havea the same provider!"
    );
    assert_eq!(
        adapter_load.into_value().unwrap().get("owner"),
        Some(Value::String("WiszeL".into())).as_ref(),
        "Should really load"
    );
}
