use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};


use crate::environment::Environment;
use crate::token::Token;
use crate::expr::{AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr};
use crate::err::LoxErr;
use crate::stmt::Stmt;
use crate::object::Object;
use crate::token_type::TokenType;



pub struct Interpreter{
    pub had_runtime_error: bool,
    environment: Rc<RefCell<Environment>>,
}


impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            had_runtime_error: false,
            environment: Environment::new(),
        }
    }

    fn get_env(&self) -> Ref<Environment> {
        self.environment.borrow()
    }

    fn get_env_mut(&self) -> RefMut<Environment> {
        self.environment.borrow_mut()
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
            Expr::Variable(variable_expr) => self.visit_variable_expr(variable_expr),
            Expr::Assign(assign) => self.visit_assign_expr(assign),
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxErr>{
        match stmt {
            Stmt::Block { statements: stmts } => self.visit_block_stmt(stmts)?,
            Stmt::Expression{ expression: expr} => self.visit_expression_stmt(expr)?,
            Stmt::If { condition, then_branch, else_branch } => self.visit_if_stmt(condition, then_branch, else_branch)?,

            Stmt::Print{ expression: expr} => self.visit_print_stmt(expr)?,
            Stmt::Var { name, initializer } => self.visit_var_stmt(name, initializer)?,
            
        };
        Ok(())
    }

    fn execute_block(&mut self, stmts: &Vec<Stmt>, environment: Rc<RefCell<Environment>>) -> Result<(), LoxErr> {
        let previous = Rc::clone(&self.environment);
        self.environment = environment;
        // let mut f = || {
        //     for stmt in stmts {
        //         self.execute(stmt)?;
        //     }
        //     Result::<(), LoxErr>::Ok(())
        // };
        // let ret = f();
        let ret = stmts.iter().try_for_each(|stmt| self.execute(stmt)); // 这一行代替上面那么多真是妙啊

        self.environment = previous;
        ret
    }

    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), LoxErr> {
        let block_env = Environment::new();
        block_env.borrow_mut().set_enclosing(Rc::clone(&self.environment));
        self.execute_block(stmts, block_env)
    }

    fn visit_expression_stmt(&mut self, expr: &Expr) -> Result<(), LoxErr> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Box<Stmt>, else_branch: &Option<Box<Stmt>>) -> Result<(), LoxErr> {
        if Interpreter::is_truthy(&self.evaluate(condition)?) {
            self.execute(&*then_branch)?;
        } else if let Some(exist_else_branch) = else_branch {
            self.execute(&*exist_else_branch)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), LoxErr> {
        let tl: Object = self.evaluate(expr)?;
        println!("{}", tl);
        Ok(())
    }

    fn visit_var_stmt(&self, name: &Token, initializer: &Option<Expr>) -> Result<(), LoxErr> {
        let value = if initializer.is_some() {
            self.evaluate(initializer.as_ref().unwrap())?
        } else {
            Object::None
        };
        self.get_env_mut().define(&name.lexeme, value);
        Ok(())
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

    fn visit_variable_expr(&self, variable_expr: &VariableExpr) -> Result<Object, LoxErr> {
        self.get_env().get(&variable_expr.name)
    }

    fn visit_assign_expr(&self, assign_expr: &AssignExpr) -> Result<Object, LoxErr> {
        let value = self.evaluate(&assign_expr.value)?;
        self.get_env_mut().assign(&assign_expr.name, value.clone())?;
        Ok(value)   // 赋值表达式可以嵌套在其它表达式里，比如：print a = 2;
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



#[cfg(test)]
mod tests {

    use super::*;
    use crate::token_type::TokenType;
    use crate::lox::Lox;

    #[test]
    fn test_block() {
        let mut lox = Lox::new();
        let code = r#"var a = "global a";
var b = "global b";
var c = "global c";
{
var a = "outer a";
var b = "outer b";
{
    var a = "inner a";
    print a;
    print b;
    print c;
}
print a;
print b;
print c;
}
print a;
print b;
print c;
        "#;
        // let code = r#"print "code";"#;

        lox.test_code(code);
    }
}