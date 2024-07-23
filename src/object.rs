use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::{self, Debug};
use std::time::{UNIX_EPOCH, SystemTime};

use crate::err::LoxErr;
use crate::lox_callable::LoxCallable;
use crate::interpreter::Interpreter;
use crate::lox_class::LoxClass;
use crate::lox_function::LoxFunction;
use crate::lox_instance::LoxInstance;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    None,
    Bool(bool),
    Class(LoxClass),
    Instance(Rc<RefCell<LoxInstance>>),
    String(String),
    Number(f64),
    Function(LoxFunction), // 函数对象
    NativeFunction(NativeFunction),

}

impl Default for Object {
    fn default() -> Self {
        Object::None
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            Object::None => write!(f, "nil"),
            Object::Bool(b) => fmt::Display::fmt(b, f),
            Object::Class(class) => fmt::Display::fmt(class, f),
            Object::String(s) => fmt::Display::fmt(s, f),
            Object::Number(n) => fmt::Display::fmt(n, f),
            Object::Function(func) => fmt::Display::fmt(func, f),
            Object::NativeFunction(native_func) => fmt::Display::fmt(native_func, f),
            Object::Instance(instance) => fmt::Display::fmt(&instance.borrow_mut(), f),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NativeFunction {
    pub name: String,
}

impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn {}", self.name)
    }
}

impl LoxCallable for NativeFunction {
    fn arity(&self) -> usize {
        match self.name.as_str() {
            "clock" => 0,
            _ => unreachable!("Invalid native fn arity."),
        }
    }

    fn call(&mut self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, LoxErr> {
        match self.name.as_str() {
            "clock" => Ok(Object::Number(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64())),
            
            _ => unreachable!("Invalid native fn call."),
        }
    }
}