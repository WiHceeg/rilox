use std::collections::HashMap;

use std::rc::Rc;
use std::cell::RefCell;

use crate::err::LoxErr;

use crate::object::Object;
use crate::token::Token;


#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new( Environment{
            enclosing: None,
            values: HashMap::new(),
        }))
    }

    pub fn set_enclosing(&mut self, enclosing: Rc<RefCell<Environment>>) {
        self.enclosing = Some(enclosing);   // Rc::clone 在外面，不然所有权就转移进来了
    }

    pub fn define(&mut self, name: &str, value: Object) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxErr> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    enclosing.borrow().get(name)
                } else {
                    Err(LoxErr::Runtime { line: name.line, message: format!("Undefined variable '{}'.", name.lexeme)})
                }
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxErr> {
        match self.values.get_mut(&name.lexeme) {
            Some(existing_value) => {
                *existing_value = value;
                Ok(())
            },
            None => {
                if let Some(enclosing) = &self.enclosing {
                    enclosing.borrow_mut().assign(name, value)
                } else {
                    Err(LoxErr::Runtime { line: name.line, message: format!("Undefined variable '{}'.", name.lexeme)})
                }
            }
        }
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Object {
        if distance == 0 {
            return self.values.get(name).unwrap().clone();
        }

        self.ancestor(distance).borrow().values.get(name).unwrap().clone()
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Object) {
        if distance == 0 {
            *self.values.get_mut(&name.lexeme).unwrap() = value;
        } else {
            *self.ancestor(distance).borrow_mut().values.get_mut(&name.lexeme).unwrap() = value;

        }
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        assert!(distance >= 1, "param distance should >= 1, now {}", distance);

        let mut ancestor = self.enclosing.clone();
        for _ in 1..distance {
            ancestor = ancestor.unwrap().borrow_mut().enclosing.clone();
        }
        ancestor.unwrap()
    }

}



#[cfg(test)]
mod tests {

    use super::*;
    use crate::token_type::TokenType;
    

    #[test]
    fn test_get_existing() {
        let env = Environment::new();
        let token = Token::new(TokenType::Identifier, "x".to_string(), Object::Number(42.0), 1);
        env.borrow_mut().define("x", Object::Number(42.0));
        match env.borrow().get(&token) {
            Ok(value) => assert_eq!(value, Object::Number(42.0)),
            Err(_) => panic!("Expected Ok(Object::Number(42.0))"),
        };
    }

    #[test]
    fn test_get_non_existing() {
        let env = Environment::new();
        let token = Token::new(TokenType::Identifier, "x".to_string(), Object::Number(0.0), 1);
        match env.borrow().get(&token) {
            Ok(_) => panic!("Expected an error for undefined variable"),
            Err(err) => match err {
                LoxErr::Runtime { line, message } => {
                    assert_eq!(line, 1);
                    assert_eq!(message, "Undefined variable 'x'.");
                }
                _ => panic!("Unexpected error type"),
            },
        };
    }

    #[test]
    fn test_assign_existing() {
        let env = Environment::new();
        let token = Token::new(TokenType::Identifier, "x".to_string(), Object::Number(42.0), 1);
        env.borrow_mut().define("x", Object::Number(42.0));
        assert!(env.borrow_mut().assign(&token, Object::Number(100.0)).is_ok());
        match env.borrow().get(&token) {
            Ok(value) => assert_eq!(value, Object::Number(100.0)),
            Err(_) => panic!("Expected Ok(Object::Number(100.0))"),
        };
    }

    #[test]
    fn test_assign_non_existing() {
        let env = Environment::new();
        let token = Token::new(TokenType::Identifier, "x".to_string(), Object::Number(0.0), 1);
        match env.borrow_mut().assign(&token, Object::Number(100.0)) {
            Ok(_) => panic!("Expected an error for undefined variable"),
            Err(err) => match err {
                LoxErr::Runtime { line, message } => {
                    assert_eq!(line, 1);
                    assert_eq!(message, "Undefined variable 'x'.");
                }
                _ => panic!("Unexpected error type"),
            },
        };
    }

}