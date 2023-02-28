use crate::{values::Value, scanner::Operator};


#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Unary {
        value: Value,
        operator: Option<Operator>,
    },
    Binary {
        x: Box<Expression>,
        operator: Operator,
        y: Box<Expression>,
    },
}