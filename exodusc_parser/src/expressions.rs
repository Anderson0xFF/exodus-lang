use exodusc_lexer::scanner::{Operator, Literal};

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Unary {
        operator: Option<Operator>,
        value: Literal,
    },
    Binary {
        x: Literal,
        operator: Operator,
        y: Literal,
    },
}