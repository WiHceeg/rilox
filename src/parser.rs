use crate::err::LoxErr;
use crate::stmt::{FunctionDeclaration, Stmt};
use crate::token::Token;
use crate::object::Object;

use crate::expr::{AssignExpr, CallExpr, Expr, LogicalExpr};
use crate::expr::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr};
use crate::token_type::TokenType;


pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl Parser<'_> {

    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),

                // 原版是在 declaration 处理错误
                Err(lox_err) => {
                    eprintln!("{}", lox_err);
                    self.synchronize();
                }
            }            
        }
        statements
    }

    fn declaration(&mut self) -> Result<Stmt, LoxErr> {
        if self.matches(&[TokenType::Var]) {
            self.var_declaration()
        } else if self.matches(&[TokenType::Fun]) {
            self.function("function")
        } else {
            self.statement()
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxErr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxErr> {
        let expr = self.or()?;    // （在可能存在的等号前面的）表达式
        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?; // 等号后面的表达式
            if let Expr::Variable(v) = expr {
                let name = v.name;
                return Ok(Expr::Assign(AssignExpr::new(name, value)));
            }
            return Err(LoxErr::Parse { line: equals.line, lexeme: equals.lexeme, message: "Invalid assignment target.".to_string() })
        }
        Ok(expr)

    }

    fn or(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.and()?;
        while self.matches(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(LogicalExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.equality()?;
        while self.matches(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(LogicalExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn statement(&mut self) -> Result<Stmt, LoxErr> {
        if self.matches(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.matches(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.matches(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.matches(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.matches(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block { statements: self.block()? })
        }
        self.expression_statement()
    }  

    fn if_statement(&mut self) -> Result<Stmt, LoxErr> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If{
            condition: condition,
            then_branch: then_branch,
            else_branch: else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxErr> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition: condition, body: body })
    }

    // 语法糖，变成 while
    fn for_statement(&mut self) -> Result<Stmt, LoxErr> {
        
        self.consume(&TokenType::LeftParen, "Expect '(' after 'for'.")?;
        let initializer = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
        let condition = if self.check(&TokenType::Semicolon) {
            Expr::Literal(LiteralExpr::new(Object::Bool(true)))     // 没写条件时，视为 true
        } else {
            self.expression()?
        };
        self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.")?;
        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(&TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut for_body = self.statement()?;
        if increment.is_some() {
            for_body = Stmt::Block { 
                statements: vec![for_body, Stmt::Expression { expression: increment.unwrap() },]
            };
        }

        let mut desugar_res = Stmt::While { 
            condition: condition, 
            body: Box::new(for_body),
        };

        if initializer.is_some() {
            desugar_res = Stmt::Block { 
                statements: vec![initializer.unwrap(), desugar_res,] 
            };
        }

        Ok(desugar_res)
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxErr> {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print{expression: value})
    }

    fn return_statement(&mut self) -> Result<Stmt, LoxErr> {
        let keyword = self.previous().clone();
        let value = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(&TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return { keyword: keyword, value: value })
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxErr> {
        let name = self.consume(&TokenType::Identifier, "Expect variable name.")?.clone();
        
        let initializer: Option<Expr> = if self.matches(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(&TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var { name: name, initializer: initializer })
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxErr> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression{expression: expr})
    }

    // 函数定义
    fn function(&mut self, kind: &str) -> Result<Stmt, LoxErr> {
        let name = self.consume(&TokenType::Identifier, &format!("Expect {} name.", kind))?.clone();
        self.consume(&TokenType::LeftParen, &format!("Expect '(' after {} name.", kind))?;
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    eprintln!("{}", LoxErr::Parse { line: self.peek().line, lexeme: self.peek().lexeme.clone(), message: "Can't have more than 255 parameters.".to_string() });
                }
                parameters.push(self.consume(&TokenType::Identifier, "Expect parameter name.")?.clone());
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(&TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(&TokenType::LeftBrace, &format!("Expect '{{' before {} body.", kind))?;    // format 里的大括号需要使用两个连续的大括号 {{ 或 }}
        let body = self.block()?;
        Ok(Stmt::FunctionDeclaration { function_declaration: FunctionDeclaration {
            name: name,
            params: parameters,
            body: body,
        } })
    }

    // 调用 block 前要先消费掉开头的 `{` 
    fn block(&mut self) -> Result<Vec<Stmt>, LoxErr>{
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
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
        self.call()
    }

    fn call(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.primary()?;

        // 这里有个 loop，是因为一个 call 的结果可能也是 callee，比如f1(a1, a2) 的结果是 f2，可以 f1(a1, a2)(b1, b2) 这样调用
        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, LoxErr> {
        let mut arguments = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    // 它会报告这个错误，并继续执行解析
                    eprintln!("{}", LoxErr::Parse { line: self.peek().line, lexeme: self.peek().lexeme.clone(), message: "Can't have more than 255 arguments.".to_string() });
                    
                }
                arguments.push(self.expression()?);
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren = self.consume(&TokenType::RightParen, "Expect ')' after arguments.")?.clone();
        Ok(Expr::Call(CallExpr::new(callee, paren, arguments)))
    }

    fn primary(&mut self) -> Result<Expr, LoxErr> {
        // 原版用的是多个 if else 配合 self.matches，会自动 advance，所以这里记得要手动 advance
        match self.peek().token_type {

            TokenType::False | TokenType::True | TokenType::Nil | TokenType::Number | TokenType::String => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::new(self.previous().literal.clone())))
            }

            TokenType::Identifier => {
                self.advance();
                Ok(Expr::Variable(VariableExpr::new(self.previous().clone())))
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

    fn consume(&mut self, tt: &TokenType, message: &str) -> Result<&Token, LoxErr> {
        if self.check(tt) {
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

    fn check(&self, tt: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *tt
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

    // 校准到下一条语句
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }
    }

}
