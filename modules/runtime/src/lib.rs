use rpds::{List, Vector};

mod symbol;
use symbol::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Symbol(Symbol),
    Decimal(i64),
    List(List<Primitive>),
    Vector(Vector<Primitive>),
    String(String),
    Boolean(bool),
    None,
}

impl<T> From<Option<T>> for Primitive
    where T: Into<Self> {
    fn from(val: Option<T>) -> Self {
        match val {
            Some(val) => val.into(),
            None => Primitive::None,
        }
    }
}

impl From<&str> for Primitive {
    fn from(val: &str) -> Self {
        Primitive::String(val.to_string())
    }
}
