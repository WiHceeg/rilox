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

impl TokenLiteral {
    fn as_bool(&self) -> Option<bool> {
        if let TokenLiteral::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    fn as_string(&self) -> Option<String> {
        if let TokenLiteral::String(s) = self {
            Some(s.clone())
        } else {
            None
        }
    }

    fn as_number(&self) -> Option<f64> {
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