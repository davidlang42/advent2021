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

    pub fn depth(&self) -> usize {
        match self {
            Function::Literal(_l) => 1,
            Function::Input(_i) => 1,
            Function::Operation(f1, _op, f2) => f1.depth().max(f2.depth()) + 1
        }
    }

    pub fn to_literal(&self) -> Option<isize> {
        if let Function::Literal(l) = self {
            Some(*l)
        } else {
            None
        }
    }

    pub fn literal_out_of_input_range(&self) -> bool {
        if let Function::Literal(l) = self {
            *l < 1 || *l > 9
        } else {
            false
        }
    }

    pub fn is_input(&self) -> bool {
        if let Function::Input(_i) = self {
            true
        } else {
            false
        }
    }

    pub fn _evaluate(&self, inputs: &Vec<usize>) -> isize {
        match self {
            Function::Literal(l) => *l,
            Function::Input(i) => *inputs.get(*i).expect("Ran out of inputs") as isize,
            Function::Operation(f1, op, f2) => op.operate(f1._evaluate(inputs), f2._evaluate(inputs))
        }
    }
}