use std::collections::{HashMap, VecDeque, HashSet};
use crate::instructions::{Instruction, Variable, Expression, Operator};
use crate::functions::Function;

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

pub struct ReverseArithmeticLogicUnit {
    required_variables: HashSet<Variable>,
    pub required_inputs: HashSet<usize>,
    pub required_instructions: VecDeque<Instruction>,
    input_counter: isize
}

impl ReverseArithmeticLogicUnit {
    pub fn new(total_inputs: usize, required_variable: Variable) -> Self {
        Self {
            input_counter: total_inputs as isize - 1,
            required_variables: HashSet::from([required_variable]),
            required_inputs: HashSet::new(),
            required_instructions: VecDeque::new()
        }
    }

    pub fn trace_back(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Input(var) => {
                if self.input_counter < 0 {
                    panic!("Ran out of inputs");
                }
                if self.required_variables.remove(var) {
                    self.required_inputs.insert(self.input_counter as usize);
                    self.required_instructions.push_front(instruction.clone());
                }
                self.input_counter -= 1;
            },
            Instruction::Operation(var, _op, exp) => {
                if self.required_variables.contains(var) {
                    if let Expression::Variable(another_variable) = exp {
                        self.required_variables.insert(another_variable.clone());
                    }
                    if !instruction.redundant() {
                        self.required_instructions.push_front(instruction.clone());
                    }
                }
            }
        }
    }
}

pub struct FunctionalArithmeticLogicUnit {
    variables: HashMap<Variable, Function>,
    input_counter: usize
}

impl FunctionalArithmeticLogicUnit {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            input_counter: 0
        }
    }

    pub fn get(&self, var: &Variable) -> &Function {
        match self.variables.get(var) {
            Some(existing_function) => existing_function,
            None => &Function::Literal(0)
        }
    }

    fn remove(&mut self, var: &Variable) -> Function {
        match self.variables.remove(var) {
            Some(existing_function) => existing_function,
            None => Function::Literal(0)
        }
    }

    fn set(&mut self, var: Variable, value: Function) {
        self.variables.insert(var, value);
    }

    pub fn run(&mut self, instruction: &Instruction) {
        if !instruction.redundant() {
            match instruction {
                Instruction::Input(var) => {
                    self.set(var.clone(), Function::Input(self.input_counter));
                    self.input_counter += 1;
                },
                Instruction::Operation(var, op, exp) => {
                    let a = self.remove(var);
                    let tmp;
                    let b = match exp {
                        Expression::Variable(b_var) => self.get(b_var),
                        Expression::Literal(b_literal) => {
                            tmp = Function::Literal(*b_literal);
                            &tmp
                        }
                    };
                    self.set(var.clone(), match op {
                        Operator::Add if a.is_literal(0) => b.clone(),
                        Operator::Add if b.is_literal(0) => a,
                        Operator::Multiply if a.is_literal(0) || b.is_literal(0) => Function::Literal(0),
                        Operator::Multiply if a.is_literal(1) => b.clone(),
                        Operator::Multiply if b.is_literal(1) => a,
                        Operator::Divide if b.is_literal(1) => a,
                        Operator::Divide if a == *b => Function::Literal(1),
                        Operator::Equal if a == *b => Function::Literal(1),
                        Operator::Equal if a.is_input() && b.literal_out_of_input_range() => Function::Literal(0),
                        Operator::Equal if b.is_input() && a.literal_out_of_input_range() => Function::Literal(0),
                        _ => {
                            if let (Some(a_literal), Some(b_literal)) = (a.to_literal(), b.to_literal()) {
                                Function::Literal(op.operate(a_literal, b_literal))
                            } else {
                                Function::Operation(Box::new(a), op.clone(), Box::new(b.clone()))
                            }
                        }
                    });
                }
            }
        }
    }
}