use std::fmt;
use std::rc::Rc;

use crate::environment::Environment;
use crate::err::LoxErr;
use crate::interpreter::Interpreter;
use crate::stmt::FunctionDeclaration;
use crate::lox_callable::LoxCallable;
use crate::object::Object;

#[derive(Debug, PartialEq, Clone)]
pub struct LoxFunction {
    pub declaration: Box<FunctionDeclaration>,
}

impl LoxFunction {
    pub fn new(fun_decl: &FunctionDeclaration) -> LoxFunction {

        LoxFunction{
            declaration: Box::new(fun_decl.clone()),
        }
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme)
    }
}


impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, LoxErr> {
        let env = Environment::new();
        env.borrow_mut().set_enclosing(Rc::clone(&interpreter.globals));

        
        for i in 0..self.declaration.params.len() {
            env.borrow_mut().define(&self.declaration.params[i].lexeme, arguments[i].clone());
        }
        interpreter.execute_block(&self.declaration.body, env)?;
        Ok(Object::None)

    }
}

