use uuid::Uuid;

/// Dynamically-typed attribute value.
#[derive(Debug, Clone)]
pub enum Value<'a> {
    Str(&'a str),
    Int(i32),
    Float(f32),
    Bool(bool),
    Uuid(Uuid),
}

impl Value<'_> {
    #[inline]
    pub fn kind(&self) -> &'static str {
        match self {
            Value::Str(_) => "str",
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::Uuid(_) => "uuid",
        }
    }
}

/// Minimal reflection trait exposed to the engine.
///
/// Implemented automatically by `#[derive(Entity)]` in **reflect-rs**.
pub trait Entity {
    /// Return the field’s value by *name*, or `None` if absent.
    fn get_field(&self, field_name: &str) -> Option<Value>;
}

/// Dummy entity used when no object is supplied.
pub struct NullEntity;
impl Entity for NullEntity {
    #[inline]
    fn get_field(&self, _: &str) -> Option<Value> {
        None
    }
}
