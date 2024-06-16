use std::fmt::{self, Debug};
use crate::token_type::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenLiteral {
    None,
    Bool(bool),
    String(String),
    Number(f64),
}

impl Default for TokenLiteral {
    fn default() -> Self {
        TokenLiteral::None
    }
}

impl fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
        match self {
            TokenLiteral::None => write!(f, "nil"),
            TokenLiteral::Bool(b) => fmt::Display::fmt(b, f),
            TokenLiteral::String(s) => fmt::Display::fmt(s, f),
            TokenLiteral::Number(n) => fmt::Display::fmt(n, f),
        }
    }
}

impl TokenLiteral {

    pub fn is_none(&self) -> bool {
        match self {
            TokenLiteral::None => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            TokenLiteral::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            TokenLiteral::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            TokenLiteral::Number(_) => true,
            _ => false,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        if let TokenLiteral::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn get_string(&self) -> Option<String> {
        if let TokenLiteral::String(s) = self {
            Some(s.clone())
        } else {
            None
        }
    }

    pub fn get_number(&self) -> Option<f64> {
        if let TokenLiteral::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String, // token 在代码中的字符串
    pub literal: TokenLiteral,   // 实际的值
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: TokenLiteral, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}