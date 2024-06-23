use crate::expr::{Expr, BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::err::LoxErr;
use crate::stmt::Stmt;
use crate::object::Object;
use crate::token_type::TokenType;

/*
program        → statement* EOF ;

declaration    → varDecl
               | statement ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;

statement      → exprStmt
               | printStmt ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;
*/

pub struct Interpreter{
    pub had_runtime_error: bool,
}


impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            had_runtime_error: false,
        }
    }

    // pub fn interpret(&self, expr: &Expr) -> Result<(), LoxErr> {
    //     let value = self.evaluate(expr)?;
    //     println!("{}", value);
    //     Ok(())
    // }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            if let Err(lox_err) = self.execute(statement) {
                println!("{}", lox_err);
                self.had_runtime_error = true;
            }
        }
    }



    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxErr> {
        match expr {
            Expr::Literal(literal_expr) => self.visit_literal_expr(literal_expr),

            Expr::Binary(binary_expr) => self.visit_binary_expr(binary_expr),

            Expr::Grouping(grouping_expr) => self.visit_grouping_expr(grouping_expr),
            Expr::Unary(unary_expr) => self.visit_unary_expr(unary_expr),
            Expr::Variable(_) => todo!(),
            
        }
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), LoxErr>{
        match stmt {
            Stmt::Expression(_) => self.visit_expression_stmt(stmt)?,
            Stmt::Print(_) => self.visit_print_stmt(stmt)?,
            Stmt::Var { name, initializer } => todo!(),
            
        };
        Ok(())
    }

    fn visit_expression_stmt(&self, stmt: &Stmt) -> Result<(), LoxErr> {
        self.evaluate(
            match stmt {
                Stmt::Expression(expr) => expr,
                _ => unreachable!(),
            }
        )?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &Stmt) -> Result<(), LoxErr> {
        let tl = self.evaluate(match stmt {
                Stmt::Print(expr) => expr,
                _ => unreachable!(),
            }
        )?;
        println!("{}", tl);
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &Stmt) -> Result<(), LoxErr> {
        todo!()
    }

    fn visit_literal_expr(&self, literal_expr: &LiteralExpr) -> Result<Object, LoxErr> {
        Ok(literal_expr.literal.clone())
    }

    fn visit_grouping_expr(&self, grouping_expr: &GroupingExpr) -> Result<Object, LoxErr> {
        self.evaluate(&grouping_expr.expression)
    }

    fn visit_unary_expr(&self, unary_expr: &UnaryExpr) -> Result<Object, LoxErr> {
        let right = self.evaluate(&unary_expr.right)?;
        match unary_expr.operator.token_type {
            TokenType::Bang => {
                return Ok(Object::Bool(!Interpreter::is_truthy(&right)));
            },
            TokenType::Minus => {
                if let Object::Number(v) = right {
                    Ok(Object::Number(-v))
                } else {
                    Interpreter::number_err(unary_expr.operator.line)
                }
            },
            _ => unreachable!("Impossible operator for unary expr."),
        }
    }

    fn visit_binary_expr(&self, binary_expr: &BinaryExpr) -> Result<Object, LoxErr> {


        let left = self.evaluate(&binary_expr.left)?;
        let right = self.evaluate(&binary_expr.right)?;
        match binary_expr.operator.token_type {
            TokenType::EqualEqual => Ok(Object::Bool(left == right)),
            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::Greater => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Bool(left_number > right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::GreaterEqual => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Bool(left_number >= right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::Less => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Bool(left_number < right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::LessEqual => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Bool(left_number <= right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::Minus => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Number(left_number - right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::Slash => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    if right_number != 0.0 {
                        Ok(Object::Number(left_number / right_number))
                    } else {
                        Err(LoxErr::Runtime { line: binary_expr.operator.line, message: format!("Attempt to divide `{}` by zero.", left_number) })
                    }
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::Star => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Number(left_number * right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::Plus => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (&left, &right) {
                    return Ok(Object::Number(left_number + right_number));
                }
                if let (Object::String(left_string), Object::String(right_string)) = (&left, &right) {
                    return Ok(Object::String(format!("{}{}", left_string, right_string)));
                }
                Err(LoxErr::Runtime { line: binary_expr.operator.line, message: "Operands must be two numbers or two strings.".to_string() })
            }
            

            _ => unreachable!("Impossible operator for binary expr."),
        }
        
    }

    fn is_truthy(literal: &Object) -> bool {
        match literal {
            Object::None => false,
            Object::Bool(v) => *v,
            _ => true,
        }
    }

    fn number_err(line: usize) -> Result<Object, LoxErr> {
        Err(LoxErr::Runtime { line: line, message: "Operand must be a number.".to_string() })
    }
}