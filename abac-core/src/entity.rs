use std::collections::HashMap;

use serde_value::Value;

use crate::Error;

pub(crate) type EntityValue = HashMap<String, Value>;

pub trait Entity {
    /// Converting into Serde Value, No need "Serialize" anymore
    fn into_value(&self) -> Result<EntityValue, Error>;

    /// Getting the field name
    fn field_names(&self) -> &'static [&'static str];
}
