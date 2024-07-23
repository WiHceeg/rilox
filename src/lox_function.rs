use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::environment::Environment;
use crate::err::LoxErr;
use crate::interpreter::Interpreter;
use crate::lox_instance::LoxInstance;
use crate::stmt::FunctionDeclaration;
use crate::lox_callable::LoxCallable;
use crate::object::Object;

#[derive(Debug, PartialEq, Clone)]
pub struct LoxFunction {
    declaration: Box<FunctionDeclaration>,
    closure: Rc<RefCell<Environment>>,  // 闭包，它 "封闭 "并保留着函数声明的外围变量
    is_initializer: bool,   // 如果该函数是一个初始化方法，我们会覆盖实际的返回值并强行返回`this`
}

impl LoxFunction {
    pub fn new(fun_decl: &FunctionDeclaration, closure: Rc<RefCell<Environment>>, is_initializer: bool) -> LoxFunction {

        LoxFunction{
            declaration: Box::new(fun_decl.clone()),
            closure: closure,
            is_initializer: is_initializer,
        }
    }

    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> Self {
        // bind 会返回一个能找到 this (即 instance 自身 ) 的方法
        // instance 的 .xx 是方法时，需要一个新的能找到 this 的 LoxFunction，这个新 LoxFunction 的 closure 里添加了 this，新 LoxFunction 的 enclosing 是原 method 的 closure
        let env = Environment::new();
        env.borrow_mut().set_enclosing(Rc::clone(&self.closure));
        env.borrow_mut().define("this", Object::Instance(Rc::clone(&instance)));
        LoxFunction::new(&self.declaration, env, self.is_initializer)
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
            Err(LoxErr::RuntimeReturn { ret_value }) => {
                if self.is_initializer {
                    // 仅当 init 里有空 return; 时会跑这里，返回 this
                    return Ok(self.closure.borrow().get_at(0, "this"));
                }
                return Ok(ret_value)
            }
            Err(other_lox_err) => return Err(other_lox_err),
            Ok(_) => (),
        }
        if self.is_initializer {
            return Ok(self.closure.borrow().get_at(0, "this"));
        }
        Ok(Object::None)

    }
}

