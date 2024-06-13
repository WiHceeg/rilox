use crate::token::{Token, TokenLiteral};
use std::fmt;

/*
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
impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}
impl fmt::Display for GroupingExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LiteralExpr {
    pub value: TokenLiteral,
}
impl fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            TokenLiteral::None => write!(f, "nil"),
            TokenLiteral::String(v) => write!(f, "{}", v),
            TokenLiteral::Number(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
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
        let expression = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Unary(UnaryExpr {
                operator: Token::new(TokenType::Minus, "-".to_string(), TokenLiteral::None, 1),
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: TokenLiteral::Number(123.),
                })),
            })),
            operator: Token::new(TokenType::Star, "*".to_string(), TokenLiteral::None, 1),
            right: Box::new(Expr::Grouping(GroupingExpr {
                expression: Box::new(Expr::Literal(LiteralExpr {
                    value: TokenLiteral::Number(45.67),
                })),
            })),
        });

        assert_eq!(
            expression.to_string(),
            "(* (- 123) (group 45.67))".to_string()
        );
    }
}
