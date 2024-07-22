use std::fmt;
use std::collections::HashMap;

use crate::err::LoxErr;
use crate::lox_class::LoxClass;
use crate::object::Object;
use crate::token::Token;


#[derive(Debug, PartialEq, Clone)]
pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> LoxInstance {
        LoxInstance {
            class: class,
            fields: HashMap::new(), // 字段
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxErr> {
        // field（字段）是直接保存在实例中的命名状态。propetry（属性）是 get 表达式可能返回的已命名的东西。每个 field 都是一个 propetry，并非每个 propetry 都是一个 field。
        match self.fields.get(&name.lexeme) {
            Some(existing_property) => Ok(existing_property.clone()),
            None => {
                // 这意味着字段会遮蔽方法
                if let Some(method) = self.class.find_method(&name.lexeme) {
                    Ok(Object::Function(method))
                } else {
                    Err(LoxErr::Runtime { line: name.line, message: format!("Undefined property {}.", name.lexeme) })
                }
            }
        }
    }

    pub fn set(&mut self, name: &Token, value: Object) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} instance>", self.class.name)
    }
}

