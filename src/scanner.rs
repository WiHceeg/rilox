use crate::token::Token;
use crate::object::Object;

use crate::token_type::TokenType;
use crate::err::LoxErr;


pub struct Scanner {
    keywords: std::collections::HashMap<String, TokenType>,
    source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}


impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            keywords: std::collections::HashMap::from([
                ("and".to_string(), TokenType::And),
                ("class".to_string(), TokenType::Class),
                ("else".to_string(), TokenType::Else),
                ("false".to_string(), TokenType::False),
                ("for".to_string(), TokenType::For),
                ("fun".to_string(), TokenType::Fun),
                ("if".to_string(), TokenType::If),
                ("nil".to_string(), TokenType::Nil),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("super".to_string(), TokenType::Super),
                ("this".to_string(), TokenType::This),
                ("true".to_string(), TokenType::True),
                ("var".to_string(), TokenType::Var),
                ("while".to_string(), TokenType::While),
            ]),
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), LoxErr> {
        // let mut many_err = LoxErr::Many(Vec::new());
        let mut err_vec = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(err) = self.scan_one_token() {
                err_vec.push(err);
            }
        }
        
        if err_vec.len() > 1 {
            return Err(LoxErr::Many(err_vec));
        } else if err_vec.len() == 1 {
            return Err(err_vec.remove(0));
        }

        self.tokens.push(Token::new(TokenType::Eof, String::new(), Object::None, self.line));
        
        Ok(())
    }

    fn scan_one_token(&mut self) -> Result<(), LoxErr> {

        let c = self.advance();
        
        match c {
            '(' => self.push_token(TokenType::LeftParen, Object::None),
            ')' => self.push_token(TokenType::RightParen, Object::None),
            '{' => self.push_token(TokenType::LeftBrace, Object::None),
            '}' => self.push_token(TokenType::RightBrace, Object::None),
            ',' => self.push_token(TokenType::Comma, Object::None),
            '.' => self.push_token(TokenType::Dot, Object::None),
            '-' => self.push_token(TokenType::Minus, Object::None),
            '+' => self.push_token(TokenType::Plus, Object::None),
            ';' => self.push_token(TokenType::Semicolon, Object::None),
            '*' => self.push_token(TokenType::Star, Object::None),

            '!' => {
                let tt = if self.match_char('=') {TokenType::BangEqual} else {TokenType::Bang};
                self.push_token(tt, Object::None);
            }
            '=' => {
                let tt = if self.match_char('=') {TokenType::EqualEqual} else {TokenType::Equal};
                self.push_token(tt, Object::None);
            }
            '<' => {
                let tt = if self.match_char('=') {TokenType::LessEqual} else {TokenType::Less};
                self.push_token(tt, Object::None);
            }
            '>' => {
                let tt = if self.match_char('=') {TokenType::GreaterEqual} else {TokenType::Greater};
                self.push_token(tt , Object::None);

            }

            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    self.block_comment()?;
                } else {
                    self.push_token(TokenType::Slash, Object::None);
                }
            }

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string()?,

            '0'..='9' => self.number(),

            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

            _ => return Err(LoxErr::Scan { line: self.line, message: "Unexpected character.".to_string() }),

        }
        Ok(())
    }



    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn push_token(&mut self, token_type: TokenType, literal: Object) {
        let text: String = self.source[self.start..self.current].iter().collect::<String>();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    /*
        处理类似这样的块注释
     */
    fn block_comment(&mut self) -> Result<(), LoxErr>{
        while self.peek() != '*' && self.peek_next() != '/' {
            if self.is_at_end() {
                return Err(LoxErr::Scan { line: self.line, message: "Unterminated block comment.".to_string() });
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        self.advance();
        self.advance();
        Ok(())
    }
    
    fn string(&mut self) -> Result<(), LoxErr>{
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxErr::Scan { line: self.line, message: "Unterminated string.".to_string() });
        }

        self.advance();
        let value: String = self.source[self.start + 1 .. self.current - 1].iter().collect::<String>();
        self.push_token(TokenType::String, Object::String(value));
        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let value_s = self.source[self.start .. self.current].iter().collect::<String>();
        self.push_token(TokenType::Number, Object::Number(value_s.parse::<f64>().unwrap()));
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = self.source[self.start..self.current].iter().collect::<String>();
        let tt = if let Some(word) = self.keywords.get(&text) {
            *word
        } else {
            TokenType::Identifier
        };

        match tt {
            // Bool 值特殊处理
            TokenType::False => self.push_token(tt, Object::Bool(false)),
            TokenType::True => self.push_token(tt, Object::Bool(true)),
            _ => self.push_token(tt, Object::None),
        }

    }

}

