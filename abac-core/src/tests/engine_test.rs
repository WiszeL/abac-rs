use std::path::PathBuf;

use macros::Entity;

use crate::{Engine, EntityAdapter, FutPin};

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

#[test]
fn register_adapter_test() {
    // ##### Arrange ##### //
    let mut engine = Engine::new();
    let path_buf = PathBuf::new();

    // ##### Act ##### //
    engine.with_provider(path_buf);
}
