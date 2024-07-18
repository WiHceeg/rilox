use std::cell::RefCell;
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
    declaration: Box<FunctionDeclaration>,
    closure: Rc<RefCell<Environment>>,  // 闭包，它 "封闭 "并保留着函数声明的外围变量
}

impl LoxFunction {
    pub fn new(fun_decl: &FunctionDeclaration, closure: Rc<RefCell<Environment>>) -> LoxFunction {

        LoxFunction{
            declaration: Box::new(fun_decl.clone()),
            closure: closure,
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
        env.borrow_mut().set_enclosing(Rc::clone(&self.closure));

        for i in 0..self.declaration.params.len() {
            env.borrow_mut().define(&self.declaration.params[i].lexeme, arguments[i].clone());
        }

        match interpreter.execute_block(&self.declaration.body, env) {
            Err(LoxErr::RuntimeReturn { ret_value }) => return Ok(ret_value),
            Err(other_lox_err) => return Err(other_lox_err),
            Ok(_) => (),

        }
        Ok(Object::None)

    }
}

