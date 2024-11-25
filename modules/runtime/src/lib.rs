use rpds::{List, Vector};

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum Primitive {
    Symbol(String),
    Decimal(i64),
    List(List<Primitive>),
    Vector(Vector<Primitive>),
}
