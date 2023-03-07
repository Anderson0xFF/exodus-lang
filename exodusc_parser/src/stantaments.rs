use crate::expressions::Expression;
use exodusc_lexer::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Stantament {
    Let {
        name: String,
        typedef: Type,
        expr: Vec<Expression>,
        line: usize,
    },

    If {
        condition: Vec<Expression>,
        then: Vec<Stantament>,
        or: Vec<Stantament>,
        line: usize,
    },

    Func {
        name: String,
        parms: HashMap<String, Type>,
        body: Vec<Stantament>,
        return_type: Type,
        line: usize,
    },

    While{
        condition: Vec<Expression>,
        body: Vec<Stantament>,
        line: usize
    },

    Return {
        expr: Vec<Expression>
    },
}
