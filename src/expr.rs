use crate::token::{Token, TokenLiteral};
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
    //   Assign(Assign),
    Binary(BinaryExpr),
    //   Call(Call),
    //   Get(Get),
    Literal(LiteralExpr),
    //   Logical(Logical),
    Grouping(GroupingExpr),
    //   Set(Set),
    //   Super(Super),
    //   This(This),
    Unary(UnaryExpr),
    //   Variable(Variable),
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(v) => v.fmt(f),
            Expr::Literal(v) => v.fmt(f),
            Expr::Grouping(v) => v.fmt(f),
            Expr::Unary(v) => v.fmt(f),
        }
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
    pub literal: TokenLiteral,
}

impl LiteralExpr {
    pub fn new(literal: TokenLiteral) -> LiteralExpr {
        LiteralExpr { literal }
    }
}
impl fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.literal {
            TokenLiteral::None => write!(f, "nil"),
            TokenLiteral::String(v) => write!(f, "{}", v),
            TokenLiteral::Number(v) => write!(f, "{}", v),
            TokenLiteral::Bool(v) => write!(f, "{}", v),
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

#[cfg(test)]
mod tests {

    use crate::token_type::TokenType;

    use super::*;

    #[test]
    fn test_display() {
        let expression = Expr::Binary(BinaryExpr::new(
            Expr::Unary(UnaryExpr::new(
                Token::new(TokenType::Minus, "-".to_string(), TokenLiteral::None, 1),
                Expr::Literal(LiteralExpr::new(TokenLiteral::Number(123.))),
            )),
            Token::new(TokenType::Star, "*".to_string(), TokenLiteral::None, 1),
            Expr::Grouping(GroupingExpr::new(Expr::Literal(LiteralExpr::new(
                TokenLiteral::Number(45.67),
            )))),
        ));

        assert_eq!(
            expression.to_string(),
            "(* (- 123) (group 45.67))".to_string()
        );
    }
}
