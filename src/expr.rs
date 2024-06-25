use crate::token::Token;
use crate::object::Object;

use std::fmt;

/*
后缀`*`允许前一个符号或组重复零次或多次
后缀`+`与此类似，但要求前面的生成式至少出现一次
后缀`?`表示可选生成式，它之前的生成式可以出现零次或一次，但不能出现多次

expression     → literal
               | unary  // 一元表达式
               | binary     // 二元表达式
               | grouping ;     // 括号

literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
                  | "+"  | "-"  | "*" | "/" ;
*/

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    //   Call(CallExpr),
    //   Get(GetExpr),
    Literal(LiteralExpr),
    //   Logical(LogicalExpr),
    Grouping(GroupingExpr),
    //   Set(SetExpr),
    //   Super(SuperExpr),
    //   This(ThisExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

// 用 Display 替代原版 Java 里的 AstPrinter 类
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Assign(v) => v.fmt(f),
            Expr::Binary(v) => v.fmt(f),


            Expr::Literal(v) => v.fmt(f),

            Expr::Grouping(v) => v.fmt(f),



            Expr::Unary(v) => v.fmt(f),
            Expr::Variable(v) => v.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,   // Rust 需要在编译期确定大小，所以用 Box
}

impl AssignExpr {
    pub fn new(name: Token, value: Expr) -> AssignExpr {
        AssignExpr {
            name: name,
            value: Box::new(value),
        }
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
        match &self.literal {
            Object::None => write!(f, "nil"),
            Object::String(v) => write!(f, "{}", v),
            Object::Number(v) => write!(f, "{}", v),
            Object::Bool(v) => write!(f, "{}", v),
        }
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
}

impl VariableExpr {
    pub fn new(name: Token) -> VariableExpr {
        VariableExpr {
            name: name,
        }
    }
}

impl fmt::Display for VariableExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.literal)
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
