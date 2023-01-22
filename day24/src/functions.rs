use crate::instructions::Operator;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Function {
    Literal(isize),
    Input(usize),
    Operation(Box<Function>, Operator, Box<Function>)
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::Literal(l) => write!(f, "{}", l),
            Function::Input(i) => write!(f, "I{{{}}}", i),
            Function::Operation(f1, op, f2) => write!(f, "({} {} {})", f1, op, f2)
        }
    }
}

impl Function {
    pub fn is_literal(&self, literal: isize) -> bool {
        if let Function::Literal(l) = self {
            literal == *l
        } else {
            false
        }
    }
}