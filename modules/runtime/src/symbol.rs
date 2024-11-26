use std::fmt;

use crate::Primitive;

#[derive(Clone, PartialEq)]
pub enum Symbol {
    Qualified { namespace: String, name: String },
    Simple { name: String },
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Symbol::Qualified { namespace, name } => {
                write!(f, "{namespace}/{name}")
            },
            Symbol::Simple { name } => write!(f, "{}", name),
        }
    }
}

impl From<Symbol> for Primitive {
    fn from(sym: Symbol) -> Self {
        Primitive::Symbol(sym)
    }
}

impl From<&str> for Symbol {
    fn from(name: &str) -> Self {
        Self::Simple { name: name.to_string() }
    }
}
