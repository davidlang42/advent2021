use crate::instructions::Operator;
use std::{fmt::{self, Formatter, Display}, collections::HashSet};

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
    pub fn short_string(&self, goal_length: usize) -> String {
        match self {
            Function::Literal(l) => format!("{}", l),
            Function::Input(i) => format!("I{{{}}}", i),
            Function::Operation(f1, op, f2) => {
                if goal_length == 1 {
                    format!("...")
                } else {
                    let g = goal_length / 2;
                    format!("({} {} {})", f1.short_string(g), op, f2.short_string(g))
                }
            }
        }
    }
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

#[derive(Debug, Clone)]
pub struct DigitValue {
    possible: [bool; 10]
}

impl DigitValue {
    fn new() -> Self {
        let mut possible = [true; 10];
        possible[0] = false; // can't be 0
        Self { possible }
    }

    fn known(&self) -> Option<isize> {
        let mut value = None;
        for (v, p) in self.possible.iter().enumerate() {
            if *p {
                if value.is_none() {
                    value = Some(v as isize);
                } else {
                    return None;
                }
            }
        }
        value
    }

    fn impossible(&self) -> bool {
        for (v, p) in self.possible.iter().enumerate() {
            if *p {
                return false;
            }
        }
        true
    }

    fn min(&self) -> Option<isize> {
        for (v, p) in self.possible.iter().enumerate() {
            if *p {
                return Some(v as isize);
            }
        }
        None
    }

    fn max(&self) -> Option<isize> {
        for (v, p) in self.possible.iter().enumerate().rev() {
            if *p {
                return Some(v as isize);
            }
        }
        None
    }

    pub fn possibilities(&self) -> Vec<isize> {
        let mut values = Vec::new();
        for (v, p) in self.possible.iter().enumerate() {
            if *p {
                values.push(v as isize);
            }
        }
        values
    }

    pub fn must_be(&mut self, value: isize) {
        self.possible = [false; 10];
        self.possible[value as usize] = true;
    }
}

impl Display for DigitValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        if let Some(known) = self.known() {
            write!(f, "{}", known)
        } else {
            write!(f, "{{")?;
            for p in self.possibilities().into_iter() {
                write!(f, "{}", p)?;
            }
            write!(f, "}}")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub inputs: [DigitValue; 14]
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for d in &self.inputs {
            write!(f, "{}", d)?;
        }
        Ok(())
    }
}

impl Solution {
    pub fn new() -> Self {
        let mut vec = Vec::new();
        for i in 0..14  {
            vec.push(DigitValue::new());
        }
        Self {
            inputs: vec.try_into().unwrap()
        }
    }

    pub fn impossible(&self) -> bool {
        for input in &self.inputs {
            if input.impossible() {
                return true;
            }
        }
        false
    }

    pub fn known_value_of(&self, function: &Function) -> Option<isize> {
        match function {
            Function::Literal(l) => Some(*l),
            Function::Input(i) => self.inputs[*i].known(),
            Function::Operation(f1, op, f2) => {
                let known1 = self.known_value_of(f1)?;
                let known2 = self.known_value_of(f2)?;
                Some(op.operate(known1, known2))
            }
        }
    }

    // pub fn must_be_less_than(&self, other: &Function) -> bool {
    //     if let (Some(self_max), Some(other_min)) = (self.max_value(), other.min_value()) {
    //         self_max < other_min
    //     } else {
    //         false
    //     }
    // }

    // pub fn cannot_be_equal_to(&self, other: &Function) -> bool {
    //     if let (Some(self_min), Some(self_max), Some(other_min), Some(other_max)) = (self.min_value(), self.max_value(), other.min_value(), other.max_value()) {
    //         self_max < other_min || self_min > other_max
    //     } else {
    //         false
    //     }
    // }

    // fn max_value(&self, function: &Function) -> Option<isize> {
    //     match function {
    //         Function::Literal(l) => Some(*l),
    //         Function::Input(i) => self.inputs[*i].max(),
    //         Function::Operation(f1, op, f2) => {
    //             match op {
    //                 Operator::Add => Some(self.max_value(f1)? + self.max_value(f2)?),
    //                 Operator::Multiply => {
    //                     let max1 = f1.max_value()?;
    //                     let max2 = f2.max_value()?;
    //                     if max1.signum() == 1 && max2.signum() == 1 {
    //                         Some(max1 * max2)
    //                     } else {
    //                         None
    //                     }
    //                 },
    //                 Operator::Divide => {
    //                     let max1 = f1.max_value()?;
    //                     let min2 = f2.min_value()?;
    //                     if max1.signum() == 1 && min2.signum() == 1 {
    //                         Some(max1 / min2)
    //                     } else {
    //                         None
    //                     }
    //                 },
    //                 Operator::Modulo => f2.max_value(),
    //                 Operator::Equal => Some(1)
    //             }
    //         }
    //     }
    // }

    // fn min_value(&self, function: &Function) -> Option<isize> {
    //     match function {
    //         Function::Literal(l) => Some(*l),
    //         Function::Input(i) => self.inputs[*i].min(),
    //         Function::Operation(f1, op, f2) => {
    //             match op {
    //                 Operator::Add => Some(f1.min_value()? + f2.min_value()?),
    //                 Operator::Multiply => {
    //                     let min1 = f1.min_value()?;
    //                     let min2 = f2.min_value()?;
    //                     if min1.signum() == 1 && min2.signum() == 1 {
    //                         Some(min1 * min1)
    //                     } else {
    //                         None
    //                     }
    //                 },
    //                 Operator::Divide => {
    //                     let min1 = f1.min_value()?;
    //                     let max2 = f2.max_value()?;
    //                     if min1.signum() == 1 && max2.signum() == 1 {
    //                         Some(min1 / max2)
    //                     } else {
    //                         None
    //                     }
    //                 },
    //                 Operator::Modulo => Some(0),
    //                 Operator::Equal => Some(0)
    //             }
    //         }
    //     }
    // }

    pub fn find(mut self, function: &Function, must_equal: isize, return_first_solution: bool, depth: usize) -> Vec<Solution> {
        //println!("{}Solving {} == {}", show_depth(depth), function.short_string(8), must_equal);
        match function {
            Function::Literal(literal) => {
                if *literal != must_equal {
                    //println!("{}No Soln", show_depth(depth));
                    vec![]
                } else {
                    println!("{}Valid {}", show_depth(depth), self);
                    vec![self]
                }
            },
            Function::Input(input) =>  {
                if must_equal < 1 || must_equal > 9 || !self.inputs[*input].possible[must_equal as usize] {
                    //println!("{}No Soln", show_depth(depth));
                    vec![]
                } else {
                    self.inputs[*input].must_be(must_equal);
                    println!("{}Valid {}", show_depth(depth), self);
                    vec![self]
                }
            },
            Function::Operation(f1, op, f2) => {
                // if know both sides, just check it
                if let (Some(known1), Some(known2)) = (self.known_value_of(f1), self.known_value_of(f2)) {
                    return if op.operate(known1, known2) == must_equal {
                        println!("{}Valid {}", show_depth(depth), self);
                        vec![self]
                    } else {
                        //println!("{}No Soln", show_depth(depth));
                        vec![]
                    };
                }
                // partially process operator *if we can*
                match op {
                    Operator::Equal => {
                        if must_equal == 0 {
                            // f1 != f2
                            // no shortcut
                        } else if must_equal == 1 {
                            // f1 == f2
                            if let Some(known1) = self.known_value_of(f1) {
                                return self.find(f2, known1, false, depth);
                            } else if let Some(known2) = self.known_value_of(f2) {
                                return self.find(f1, known2, false, depth);
                            } else {
                                // no shortcut
                            }
                        }
                    },
                    Operator::Add => {
                        if let Some(known1) = self.known_value_of(f1) {
                            return self.find(f2, must_equal - known1, false, depth);
                        } else if let Some(known2) = self.known_value_of(f2) {
                            return self.find(f1, must_equal - known2, false, depth);
                        } else {
                            // no shortcut
                        }
                    },
                    Operator::Multiply => {
                        if let Some(known1) = self.known_value_of(f1) {
                            if must_equal.abs() % known1.abs() != 0 {
                                //println!("{}No Soln", show_depth(depth));
                                return vec![];
                            }
                            return self.find(f2, must_equal / known1, false, depth);
                        } else if let Some(known2) = self.known_value_of(f2) {
                            if must_equal.abs() % known2.abs() != 0 {
                                //println!("{}No Soln", show_depth(depth));
                                return vec![];
                            }
                            return self.find(f1, must_equal / known2, false, depth);
                        } else {
                            // no shortcut
                        }
                    },
                    Operator::Divide => {
                        // no shortcut
                    },
                    Operator::Modulo => {
                        // no shortcut
                    }
                }
                // otherwise choose a value for the highest order unknown digit and try again
                for i in 0..self.inputs.len() {
                    if self.inputs[i].known().is_some() {
                        continue;
                    }
                    let mut solutions  = Vec::new();
                    for p in self.inputs[i].possibilities().into_iter().rev() {
                        let mut set_one_value = self.clone();
                        set_one_value.inputs[i].must_be(p);
                        solutions.append(&mut set_one_value.find(function, must_equal, false, depth + 1)); // can I pass through return_first_solution?
                        if return_first_solution && solutions.len() > 0 {
                            break;
                        }
                    }
                    for s in &solutions {
                        println!("{}Many valid {}", show_depth(depth), s);
                    }
                    return solutions;
                }
                panic!("We knew all the inputs but somehow didn't solve it: {}", self);
            }
        }
    }
}

fn show_depth(depth: usize) -> String {
    format!("{}: ", depth)
}