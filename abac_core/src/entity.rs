use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Str(&'a str),
    Int(i32),
    Float(f32),
    Bool(bool),
    Uuid(Uuid),
}

/// A helper to get what type is it
impl Value<'_> {
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

pub trait Entity {
    fn get_field(&self, field_name: &str) -> Option<Value>;
}
