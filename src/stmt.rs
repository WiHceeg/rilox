use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    
    ClassDeclaration {
        class_declaration: ClassDeclaration,
    },

    Expression {
        expression: Expr,
    },

    // 函数定义
    FunctionDeclaration {
        function_declaration: FunctionDeclaration,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Print {
        expression: Expr,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>, // 初始化表达式
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,

}


impl Stmt {
    pub fn is_function_declaration(&self) -> bool {
        match self {
            Stmt::FunctionDeclaration { .. } => true,
            _ => false,
        }
    }

    pub fn into_function_declaration(self) -> Option<FunctionDeclaration> {
        match self {
            Stmt::FunctionDeclaration { function_declaration } => Some(function_declaration),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration {
    pub name: Token,
    pub methods: Vec<FunctionDeclaration>,
}