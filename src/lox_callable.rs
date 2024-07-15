use crate::{err::LoxErr, interpreter::Interpreter, object::Object};

pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, LoxErr>;
}