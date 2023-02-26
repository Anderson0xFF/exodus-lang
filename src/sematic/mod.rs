use std::{collections::HashMap, hash::Hasher, process::exit};

use colored::Colorize;

use crate::{
    lexer::{tokens::Expression, types::Type},
    parse::{
        errors::ErrorCode::{*, self},
        statament::{self, Stantament},
        values::Value,
    },
};

pub struct Sematic {
    filename: String,
    objects: Vec<String>,
    ast: Vec<Stantament>,
}

impl Sematic {
    pub fn default(filename: &str, ast: Vec<Stantament>) -> Self{
        Self { filename: filename.to_string(), objects: Vec::new(), ast }
    }

    pub fn analyse(&mut self) -> Vec<Stantament> {
        let mut ast = self.ast.clone();
        for statament in &mut ast {
            match statament {
                Stantament::FUNC {name, parms, body, return_type, line} => self.analyse_func(parms, body),
                _ => panic!(),
            };
        }
        return ast;
    }

    fn analyse_func(&mut self, parms: &mut HashMap<String, Type>, body: &mut Vec<Stantament>) {
        let mut variables: HashMap<String, Type> = HashMap::new();
        variables.extend(parms.clone());
        self.analyse_scope(variables, body);
    }

    fn analyse_if(&mut self, previous_variables: HashMap<String, Type>, condition: Vec<Expression>, then: &mut Vec<Stantament>, or: &mut Vec<Stantament>) {
        self.analyse_scope(previous_variables.clone(), then);
        self.analyse_scope(previous_variables, or);
    }

    fn analyse_scope(&mut self, previous_variables: HashMap<String, Type>, body: &mut Vec<Stantament>) {
        let mut variables: HashMap<String, Type> = HashMap::new();
        variables.extend(previous_variables);

        for stat in body {
            match stat {
                Stantament::LET { name, typed, expr, line} => {
                    self.analyse_let(name.clone(), &mut variables, typed, expr, *line)
                },
                Stantament::IF {condition, then, or} => {
                    self.analyse_if(variables.clone(), condition.to_vec(), then, or)
                },
                _ => (),
            }
        }
    }

    fn analyse_let(&self, name: String, variables: &mut HashMap<String, Type>, typedef: &mut Type, exprs: &Vec<Expression>, line: usize) {
        for expr in exprs {
            match expr {
                Expression::Unary { value, operator } => self.analyse_unary(variables, value, typedef, line),
                Expression::Binary { x, operator, y } => todo!(),
            }
        }
        variables.insert(name, typedef.clone());
    }

    fn analyse_unary(&self, variables: &mut HashMap<String, Type>, value: &Value, typedef: &mut Type, line: usize) {
        let value_type = self.check_type(value, Some(variables), line);
        if typedef.is_any() {
            *typedef = value_type;
        } else if typedef != &value_type {
            self.report(STATUS_MISMATCHED_TYPES, line, format!("expected `{:?}`, found `{:?}`", typedef, value_type))
        }
    }

    fn check_type(&self, value: &Value, variables: Option<&HashMap<String, Type>>, line: usize) -> Type {
        match value {
            Value::CHAR(_) => Type::Char,
            Value::STRING(_) => Type::String,
            Value::I8(_) => Type::I8,
            Value::I16(_) => Type::I16,
            Value::I32(_) => Type::I32,
            Value::I64(_) => Type::I64,
            Value::U8(_) => Type::U8,
            Value::U16(_) => Type::U16,
            Value::U32(_) => Type::U32,
            Value::U64(_) => Type::U64,
            Value::VAR(var) => match variables.unwrap().get(var) {
                Some(t) => t.clone(),
                None => self.report(STATUS_NOT_DECLARED_VARIABLE, line, format!("'{var}' not declared in scope."))
            },
            Value::OBJECT(obj) => Type::Object(obj.to_string()),
        }
    }

    pub fn report(&self, status: ErrorCode, line: usize, note: String) -> ! {
        println!(
            "{}:{} [ status: {:?}, code: {}{}, note: {} ]",
            self.filename,
            line.to_string().bright_white(),
            status,
            "E0".bright_red(),
            status.code().to_string().bright_red(),
            note.cyan().bold()
        );
        exit(0)
    }
}
