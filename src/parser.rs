use crate::err::LoxErr;
use crate::token::{Token, TokenLiteral};
use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::token_type::TokenType;




/*
后缀`*`允许前一个符号或组重复零次或多次
后缀`+`与此类似，但要求前面的生成式至少出现一次
后缀`?`表示可选生成式，它之前的生成式可以出现零次或一次，但不能出现多次

expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;     // term 项，项之间通常通过加法或减法连接
factor         → unary ( ( "/" | "*" ) unary )* ;       // factor 因子，因子之间通常通过乘法或除法连接
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
*/


pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {

    pub fn parse(&mut self) {

    }
    fn expression(&mut self) -> Result<Expr, LoxErr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.term()?;

        while self.matches(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right: Expr = self.term()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right: Expr = self.unary()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxErr> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr::new(operator, right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxErr> {

        match self.peek().token_type {

            TokenType::False | TokenType::True | TokenType::Nil | TokenType::Number | TokenType::String => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::new(self.previous().literal.clone())))
            }

            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping(GroupingExpr::new(expr)))
            }
            _ => {
                Err(LoxErr::Parse { line: self.peek().line, lexeme: String::new(), message: "Expect expression.".to_string() })
            }
        }


    }

    fn consume(&mut self, t: &TokenType, message: &str) -> Result<&Token, LoxErr> {
        if self.check(t) {
            Ok(self.advance())
        } else {
            let peek = self.peek();
            match peek.token_type {
                TokenType::Eof => Err(LoxErr::Parse { line: peek.line, lexeme: "end".to_string(), message: message.to_string() }),
                _ => Err(LoxErr::Parse { line: peek.line, lexeme: format!("'{}'", peek.lexeme.clone()), message: message.to_string() }),
            }
        }
    }

    fn matches(&mut self, types: &[TokenType]) -> bool{
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *t
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => (),
                _ => {}
            }
            self.advance();
        }
    }

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }
}
