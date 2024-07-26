use crate::resolvable::Resolvable;
use crate::token::Token;
use crate::object::Object;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Comma(CommaExpr),
    Conditional(ConditionalExpr), // 三元操作符表达式 ? :
    Call(CallExpr),
    Get(GetExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Set(SetExpr),
    Super(SuperExpr),
    This(ThisExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

// 用 Display 替代原版 Java 里的 AstPrinter 类
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Assign(v) => v.fmt(f),
            Expr::Binary(v) => v.fmt(f),
            Expr::Call(v) => v.fmt(f),
            Expr::Comma(v) => v.fmt(f),
            Expr::Conditional(v) => v.fmt(f),
            Expr::Get(v) => v.fmt(f),

            Expr::Literal(v) => v.fmt(f),
            Expr::Logical(v) => v.fmt(f),
            Expr::Grouping(v) => v.fmt(f),

            Expr::Set(v) => v.fmt(f),
            Expr::Super(v) => v.fmt(f),
            Expr::This(v) => v.fmt(f),
            Expr::Unary(v) => v.fmt(f),
            Expr::Variable(v) => v.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,   // Rust 需要在编译期确定大小，所以用 Box
    distance: Option<usize>,
}

impl AssignExpr {
    pub fn new(name: Token, value: Expr) -> AssignExpr {
        AssignExpr {
            name: name,
            value: Box::new(value),
            distance: None,
        }
    }
}

impl Resolvable for AssignExpr {
    fn name(&self) -> &Token {
        &self.name
    }

    fn set_distance(&mut self, distance: usize) {
        self.distance = Some(distance);
    }

    fn get_distance(&self) -> Option<usize> {
        self.distance
    }
}

impl fmt::Display for AssignExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(= {} {})", self.name.lexeme, self.value)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: Token, right: Expr) -> BinaryExpr {
        BinaryExpr {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
        }
    }
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,  // 这个 Expr 应该是 Variable
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

impl CallExpr {
    pub fn new(callee: Expr, paren: Token, arguments: Vec<Expr>) -> CallExpr {
        CallExpr {
            callee: Box::new(callee),
            paren: paren,
            arguments: arguments,
        }
    }
}

impl fmt::Display for CallExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(call {} {:?})", self.callee, self.arguments)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CommaExpr {
    pub exprs: Vec<Expr>,
}

impl CommaExpr {
    pub fn new(exprs: Vec<Expr>) -> CommaExpr {
        CommaExpr {
            exprs: exprs,
        }
    }
}

impl fmt::Display for CommaExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(comma {:?})", self.exprs)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalExpr {
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Box<Expr>,
}

impl ConditionalExpr {
    pub fn new(condition: Expr, then_branch: Expr, else_branch: Expr) -> ConditionalExpr {
        ConditionalExpr {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }
}

impl fmt::Display for ConditionalExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}? {})", self.condition, self.then_branch, self.else_branch)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct GetExpr {
    pub object: Box<Expr>,
    pub name: Token,
}

impl GetExpr {
    pub fn new(object: Expr, name: Token) -> GetExpr {
        GetExpr {
            object: Box::new(object),
            name: name,
        }
    }
}

impl fmt::Display for GetExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(. {} {})", self.object, self.name.lexeme)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> GroupingExpr {
        GroupingExpr {
            expression: Box::new(expression),
        }
    }
}

impl fmt::Display for GroupingExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}


/*
一个表达式树的叶子节点（构成其它表达式的语法原子单位）是字面量。
字面符号几乎已经是值了，但两者的区别很重要。
字面量是产生一个值的语法单元。字面量总是出现在用户的源代码中的某个地方。
而很多值是通过计算产生的，并不存在于代码中的任何地方，这些都不是字面量。
字面量来自于解析器(parser)领域，而值是一个解释器(interpreter)的概念，是运行时(runtime)世界的一部分。
*/
#[derive(Debug, PartialEq, Clone)]
pub struct LiteralExpr {
    pub literal: Object,
}

impl LiteralExpr {
    pub fn new(literal: Object) -> LiteralExpr {
        LiteralExpr { literal }
    }
}
impl fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.literal, f)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl LogicalExpr {
    pub fn new(left: Expr, operator: Token, right: Expr) -> LogicalExpr {
        LogicalExpr {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
        }
    }
}

impl fmt::Display for LogicalExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetExpr {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

impl SetExpr {
    pub fn new(object: Expr, name: Token, value: Expr) -> SetExpr {
        SetExpr {
            object: Box::new(object),
            name: name,
            value: Box::new(value),
        }
    }
}

impl fmt::Display for SetExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(={} {} {})", self.object, self.name.lexeme, self.value)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct SuperExpr {
    pub keyword: Token,
    pub method: Token,
    distance: Option<usize>,
}

impl SuperExpr {
    pub fn new(keyword: Token, method: Token) -> SuperExpr {
        SuperExpr {
            keyword: keyword,
            method: method,
            distance: None,
        }
    }
}

impl fmt::Display for SuperExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(super {})", self.method.lexeme)
    }
}

impl Resolvable for SuperExpr {
    fn name(&self) -> &Token {
        &self.keyword
    }

    fn set_distance(&mut self, distance: usize) {
        self.distance = Some(distance);
    }

    fn get_distance(&self) -> Option<usize> {
        self.distance
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThisExpr {
    pub keyword: Token,
    distance: Option<usize>,
}

impl ThisExpr {
    pub fn new(keyword: Token) -> ThisExpr {
        ThisExpr {
            keyword: keyword,
            distance: None,
        }
    }
}

impl fmt::Display for ThisExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "this")
    }
}

impl Resolvable for ThisExpr {
    fn name(&self) -> &Token {
        &self.keyword
    }

    fn set_distance(&mut self, distance: usize) {
        self.distance = Some(distance);
    }

    fn get_distance(&self) -> Option<usize> {
        self.distance
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Expr) -> UnaryExpr {
        UnaryExpr {
            operator: operator,
            right: Box::new(right),
        }
    }
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.operator.lexeme, self.right)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableExpr {
    pub name: Token,
    distance: Option<usize>,
}

impl VariableExpr {
    pub fn new(name: Token) -> VariableExpr {
        VariableExpr {
            name: name,
            distance: None,
        }
    }
}

impl fmt::Display for VariableExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.literal)
    }
}

impl Resolvable for VariableExpr {
    fn name(&self) -> &Token {
        &self.name
    }

    fn set_distance(&mut self, distance: usize) {
        self.distance = Some(distance);
    }

    fn get_distance(&self) -> Option<usize> {
        self.distance
    }
}

#[cfg(test)]
mod tests {

    use crate::token_type::TokenType;

    use super::*;

    #[test]
    fn test_display() {
        let expression = Expr::Binary(BinaryExpr::new(
            Expr::Unary(UnaryExpr::new(
                Token::new(TokenType::Minus, "-".to_string(), Object::None, 1),
                Expr::Literal(LiteralExpr::new(Object::Number(123.))),
            )),
            Token::new(TokenType::Star, "*".to_string(), Object::None, 1),
            Expr::Grouping(GroupingExpr::new(Expr::Literal(LiteralExpr::new(
                Object::Number(45.67),
            )))),
        ));

        assert_eq!(
            expression.to_string(),
            "(* (- 123) (group 45.67))".to_string()
        );
    }
}
