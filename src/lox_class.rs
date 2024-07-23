use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;

use crate::lox_callable::LoxCallable;
use crate::lox_function::LoxFunction;
use crate::lox_instance::LoxInstance;
use crate::object::Object;
use crate::interpreter::Interpreter;
use crate::err::LoxErr;




#[derive(Debug, PartialEq, Clone)]
pub struct LoxClass {
    pub name: String,
    pub superclass: Option<Box<LoxClass>>,
    pub methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(name: String, superclass: Option<Box<LoxClass>>, methods: HashMap<String, LoxFunction>) -> LoxClass {
        LoxClass{
            name: name,
            superclass: superclass,
            methods: methods,
        }
    }

    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }
        if let Some(exist_super) = &self.superclass {
            return exist_super.find_method(name);
        }
        None
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        let initializer = self.find_method("init");
        if let Some(exist_init) = initializer {
            return exist_init.arity();
        }
        0
    }

    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, LoxErr> {
        let instance = Rc::new(RefCell::new(LoxInstance::new(self.clone())));
        let initializer = self.find_method("init");
        if let Some(exist_init) = initializer {
            exist_init.bind(Rc::clone(&instance)).call(interpreter, arguments)?;    // 在返回 instance 前，调用它的 init 方法，在调用它的 init 方法前，让它 bind 一下找到 this
        }

        Ok(Object::Instance(instance))
    }
}