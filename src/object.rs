use std::fmt::{self, Debug};



#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    None,
    Bool(bool),
    String(String),
    Number(f64),
}

impl Default for Object {
    fn default() -> Self {
        Object::None
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
        match self {
            Object::None => write!(f, "nil"),
            Object::Bool(b) => fmt::Display::fmt(b, f),
            Object::String(s) => fmt::Display::fmt(s, f),
            Object::Number(n) => fmt::Display::fmt(n, f),
        }
    }
}

impl Object {

    pub fn is_none(&self) -> bool {
        match self {
            Object::None => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Object::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Object::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Object::Number(_) => true,
            _ => false,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        if let Object::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn get_string(&self) -> Option<String> {
        if let Object::String(s) = self {
            Some(s.clone())
        } else {
            None
        }
    }

    pub fn get_number(&self) -> Option<f64> {
        if let Object::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }
}
