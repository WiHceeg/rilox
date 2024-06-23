use std::fmt::{self, Debug};
use crate::token_type::TokenType;
use crate::object::Object;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String, // token 在代码中的字符串
    pub literal: Object,   // 实际的值
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}