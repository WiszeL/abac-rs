use std::collections::HashMap;

use serde_value::Value;

use crate::Error;

pub(crate) type EntityValue = HashMap<String, Value>;

/// The actual Entity that is gonna be evaluated by ABAC
pub trait Entity {
    /// Converting into Serde Value, No need "Serialize" anymore
    fn to_value(&self) -> Result<EntityValue, Error>;

    /// Getting the field name
    fn field_names(&self) -> &'static [&'static str];
}

/// Empty Entity, this is used for something when resource is not needed to be evaluated (eg. List all Resources, Create Resource)
pub(crate) struct EmptyEntity;

impl Entity for EmptyEntity {
    fn to_value(&self) -> Result<EntityValue, Error> {
        Ok(HashMap::new())
    }

    fn field_names(&self) -> &'static [&'static str] {
        &[]
    }
}
