use crate::token_type::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenLiteral {
    None,
    String(String),
    Number(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
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