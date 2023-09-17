use crate::tokens::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number {
        value: String,
        dimension: Option<Box<Expr>>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Call {
        name: Token,
        arguments: Vec<Expr>,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
}
