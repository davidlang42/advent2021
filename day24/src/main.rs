use std::env;
use std::fs;
use std::collections::VecDeque;

mod instructions;
mod alu;

use crate::instructions::{Instruction, Variable};
use crate::alu::ArithmeticLogicUnit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.lines().map(|l| l.parse().unwrap()).collect();
        let test_input = "13579246899999";
        let inputs: VecDeque<isize> = test_input.chars().map(|c| c.to_digit(10).unwrap() as isize).collect();
        let mut alu = ArithmeticLogicUnit::new(inputs);
        for instruction in &instructions {
            alu.run(instruction);
        }
        let valid = alu.get(&Variable::Z) == 0;
        println!("{} is {} model number", test_input, if valid { "a VALID" } else { "an INVALID"});
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
