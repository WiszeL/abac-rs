use std::{any::Any, collections::HashMap};

use serde_value::Value;
use uuid::Uuid;

use crate::Error;

pub(crate) type EntityValue = HashMap<String, Value>;

/// The actual Entity that is gonna be evaluated by ABAC
pub trait Entity {
    /// Converting into Serde Value, No need "Serialize" anymore
    fn into_value(&self) -> Result<EntityValue, Error>;

    /// Getting the field name
    fn field_names(&self) -> &'static [&'static str];
}

/// Loader for the entity before getting evaluated
pub trait EntityLoader: Sized + 'static {
    type Provider: Any + Send + Sync + 'static;

    fn load_data(id: Uuid, provider: &Self::Provider) -> impl Future<Output = Self> + Send;
}
