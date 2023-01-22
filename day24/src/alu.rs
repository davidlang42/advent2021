use std::collections::{HashMap, VecDeque};
use crate::instructions::{Instruction, Variable, Expression};

pub struct ArithmeticLogicUnit {
    variables: HashMap<Variable, isize>,
    inputs: VecDeque<isize>
}

impl ArithmeticLogicUnit {
    pub fn new(inputs: VecDeque<isize>) -> Self {
        Self {
            variables: HashMap::new(),
            inputs
        }
    }

    pub fn get(&self, var: &Variable) -> isize {
        *self.variables.get(var).unwrap_or(&0)
    }

    fn set(&mut self, var: Variable, value: isize) {
        self.variables.insert(var, value);
    }

    pub fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Input(var) => {
                let new_value = self.inputs.pop_front().expect("Ran out of input values");
                self.set(var.clone(), new_value);
            },
            Instruction::Operation(var, op, exp) => {
                let a = self.get(var);
                let b = match exp {
                    Expression::Variable(b_var) => self.get(b_var),
                    Expression::Literal(b_literal) => *b_literal
                };
                let new_value = op.operate(a, b);
                self.set(var.clone(), new_value);
            }
        }
    }
}
