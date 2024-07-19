use std::collections::HashMap;


use crate::err::LoxErr;
use crate::expr::{AssignExpr, BinaryExpr, CallExpr, Expr, GroupingExpr, LogicalExpr, UnaryExpr, VariableExpr};

use crate::resolvable::Resolvable;
use crate::stmt::{FunctionDeclaration, Stmt};
use crate::token::Token;

pub struct Resolver {
    pub had_resolve_error: bool,
    scopes: Vec<HashMap<String, bool>>,
}


impl Resolver {

    pub fn new() -> Resolver {
        Resolver {
            had_resolve_error: false,
            scopes: Vec::new(),
        }
    }

    pub fn resolve(&mut self, statements: &mut Vec<Stmt>) {
        for statement in statements {
            if let Err(lox_err) = self.resolve_stmt(statement) {
                eprintln!("{}", lox_err);
                self.had_resolve_error = true;
            }
        }
    }

    fn resolve_stmt(&mut self, stmt: &mut Stmt) -> Result<(), LoxErr> {
        match stmt {
            Stmt::Block { statements } => self.visit_block_stmt(statements),
            Stmt::Expression { expression } => self.visit_expression_stmt(expression),
            Stmt::FunctionDeclaration { function_declaration } => self.visit_function_declaration_stmt(function_declaration),
            Stmt::If { condition, then_branch, else_branch } => self.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::While { condition, body } => self.visit_while_stmt(condition, body),
            Stmt::Print { expression } => self.visit_print_stmt(expression),
            Stmt::Return { keyword: _, value } => self.visit_return_stmt(value),
            Stmt::Var { name, initializer } => self.visit_var_stmt(name, initializer),
        }
    }

    fn resolve_expr(&mut self, expr: &mut Expr) -> Result<(), LoxErr> {
        match expr {
            Expr::Assign(assign_expr) => self.visit_assign_expr(assign_expr),
            Expr::Binary(binary_expr) => self.visit_binary_expr(binary_expr),
            Expr::Call(call_expr) => self.visit_call_expr(call_expr),
            Expr::Grouping(grouping_expr) => self.visit_grouping_expr(grouping_expr),
            Expr::Literal(_literal_expr) => self.visit_literal_expr(),
            Expr::Logical(logical_expr) => self.visit_logical_expr(logical_expr),
            Expr::Unary(unary_expr) => self.visit_unary_expr(unary_expr),
            Expr::Variable(variable_expr) => self.visit_variable_expr(variable_expr),
        }
    }

    fn resolve_function(&mut self, function_declaration: &mut FunctionDeclaration) -> Result<(), LoxErr> {
        self.begin_scope();
        for param in &function_declaration.params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve(&mut function_declaration.body);
        self.end_scope();
        Ok(())
    }


    fn resolve_local(&mut self, resolvable: &mut impl Resolvable) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&resolvable.name().lexeme) {
                resolvable.set_distance(self.scopes.len() - 1 - i);
                return;
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }





    fn declare(&mut self, name: &Token) -> Result<(), LoxErr> {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
            
        }
        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn visit_block_stmt(&mut self, statements: &mut Vec<Stmt>) -> Result<(), LoxErr> {
        self.begin_scope();
        self.resolve(statements);
        self.end_scope();
        Ok(())
    }

    fn visit_expression_stmt(&mut self, expression: &mut Expr) -> Result<(), LoxErr> {
        self.resolve_expr(expression)?;
        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &mut Option<Expr>) -> Result<(), LoxErr> {
        self.declare(name)?;
        if let Some(expr) = initializer {
            self.resolve_expr(expr)?;
        }
        self.define(name);

        Ok(())
    }

    fn visit_function_declaration_stmt(&mut self, function_declaration: &mut FunctionDeclaration) -> Result<(), LoxErr> {
        self.declare(&function_declaration.name)?;
        self.define(&function_declaration.name);
        self.resolve_function(function_declaration)?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, condition: &mut Expr, then_branch: &mut Box<Stmt>, else_branch: &mut Option<Box<Stmt>>) -> Result<(), LoxErr> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(then_branch)?;
        if let Some(exist_else_branch) = else_branch {
            self.resolve_stmt(exist_else_branch)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, expression: &mut Expr) -> Result<(), LoxErr> {
        self.resolve_expr(expression)?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, value: &mut Option<Expr>) -> Result<(), LoxErr> {
        if let Some(exist_ret_value) = value {
            self.resolve_expr(exist_ret_value)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &mut Expr, body: &mut Box<Stmt>) -> Result<(), LoxErr> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(body)?;
        Ok(())
    }

    fn visit_variable_expr(&mut self, variable_expr: &mut VariableExpr) -> Result<(), LoxErr> {
        if let Some(scope) = self.scopes.last() {
            if scope.get(&variable_expr.name.lexeme) == Some(&false) {
                // 在初始化式中引用一个变量是错误的。如果初始化式使用了要初始化的变量，则解释器在编译时或运行时都会失败。
                return Err(LoxErr::Resolve { line: variable_expr.name.line, message: "Can't read local variable in its own initializer.".to_string() })

            }
        }
        self.resolve_local(variable_expr);
        Ok(())
    }

    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr) -> Result<(), LoxErr> {
        self.resolve_expr(&mut *(*assign_expr).value)?;
        self.resolve_local(assign_expr);
        Ok(())
    }

    fn visit_binary_expr(&mut self, binary_expr: &mut BinaryExpr) -> Result<(), LoxErr> {
        self.resolve_expr(&mut *(*binary_expr).left)?;
        self.resolve_expr(&mut *(*binary_expr).right)?;
        Ok(())
    }
    
    fn visit_call_expr(&mut self, call_expr: &mut CallExpr) -> Result<(), LoxErr> {
        self.resolve_expr(&mut *(*call_expr).callee)?;
        for argument in &mut call_expr.arguments {
            self.resolve_expr(argument)?;
        }
        Ok(())
    }

    fn visit_literal_expr(&self) -> Result<(), LoxErr> {
        Ok(())
    }

    fn visit_grouping_expr(&mut self, grouping_expr: &mut GroupingExpr) -> Result<(), LoxErr> {
        self.resolve_expr(&mut *(*grouping_expr).expression)?;
        Ok(())
    }

    fn visit_logical_expr(&mut self, logical_expr: &mut LogicalExpr) -> Result<(), LoxErr> {
        self.resolve_expr(&mut *(*logical_expr).left)?;
        self.resolve_expr(&mut *(*logical_expr).right)?;
        Ok(())
    }

    fn visit_unary_expr(&mut self, unary_expr: &mut UnaryExpr) -> Result<(), LoxErr> {
        self.resolve_expr(&mut *(*unary_expr).right)?;
        Ok(())
    }

}