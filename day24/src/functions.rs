use crate::instructions::Operator;
use std::{fmt, collections::HashSet};

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
        if let Some(l) = self.to_literal() {
            literal == l
        } else {
            false
        }
    }

    pub fn is_operation(&self, operator: &Operator) -> bool {
        if let Function::Operation(_f1, op, _f2) = self {
            operator == op
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
        } else if let (Some(min), Some(max)) = (self.min_value(), self.max_value()) {
            if min == max {
                Some(min)
            } else {
                None
            }            
        } else {
            None
        }
    }

    const INPUT_MAX: isize = 9;
    const INPUT_MIN: isize = 1;

    pub fn literal_out_of_input_range(&self) -> bool {
        if let Function::Literal(l) = self {
            *l < Self::INPUT_MIN || *l > Self::INPUT_MAX
        } else {
            false
        }
    }

    pub fn must_be_less_than(&self, other: &Function) -> bool {
        if let (Some(self_max), Some(other_min)) = (self.max_value(), other.min_value()) {
            self_max < other_min
        } else {
            false
        }
    }

    pub fn cannot_be_equal_to(&self, other: &Function) -> bool {
        if let (Some(self_min), Some(self_max), Some(other_min), Some(other_max)) = (self.min_value(), self.max_value(), other.min_value(), other.max_value()) {
            self_max < other_min || self_min > other_max
        } else {
            false
        }
    }

    fn max_value(&self) -> Option<isize> {
        match self {
            Function::Literal(l) => Some(*l),
            Function::Input(_i) => Some(Self::INPUT_MAX),
            Function::Operation(f1, op, f2) => {
                match op {
                    Operator::Add => Some(f1.max_value()? + f2.max_value()?),
                    Operator::Multiply => {
                        let max1 = f1.max_value()?;
                        let max2 = f2.max_value()?;
                        if max1.signum() == 1 && max2.signum() == 1 {
                            Some(max1 * max2)
                        } else {
                            None
                        }
                    },
                    Operator::Divide => {
                        let max1 = f1.max_value()?;
                        let min2 = f2.min_value()?;
                        if max1.signum() == 1 && min2.signum() == 1 {
                            Some(max1 / min2)
                        } else {
                            None
                        }
                    },
                    Operator::Modulo => f2.max_value(),
                    Operator::Equal => Some(1)
                }
            }
        }
    }

    fn min_value(&self) -> Option<isize> {
        match self {
            Function::Literal(l) => Some(*l),
            Function::Input(_i) => Some(Self::INPUT_MIN),
            Function::Operation(f1, op, f2) => {
                match op {
                    Operator::Add => Some(f1.min_value()? + f2.min_value()?),
                    Operator::Multiply => {
                        let min1 = f1.min_value()?;
                        let min2 = f2.min_value()?;
                        if min1.signum() == 1 && min2.signum() == 1 {
                            Some(min1 * min1)
                        } else {
                            None
                        }
                    },
                    Operator::Divide => {
                        let min1 = f1.min_value()?;
                        let max2 = f2.max_value()?;
                        if min1.signum() == 1 && max2.signum() == 1 {
                            Some(min1 / max2)
                        } else {
                            None
                        }
                    },
                    Operator::Modulo => Some(0),
                    Operator::Equal => Some(0)
                }
            }
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

    pub fn refers_to_inputs(&self) -> HashSet<usize> {
        match self {
            Function::Literal(_l) => HashSet::new(),
            Function::Input(i) => HashSet::from([*i]),
            Function::Operation(f1, _op, f2) => {
                let mut set = f1.refers_to_inputs();
                for i in f2.refers_to_inputs().drain() {
                    set.insert(i);
                }
                set
            }
        }
    }
}