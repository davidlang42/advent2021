use std::env;
use std::fs;
use std::collections::VecDeque;

mod instructions;
mod alu;

use crate::instructions::{Instruction, Variable};
use crate::alu::{ArithmeticLogicUnit, ReverseArithmeticLogicUnit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.lines().map(|l| l.parse().unwrap()).collect();
        // test existing model number (shouldn't that have worked?)
        let test_input = "13579246899999";
        let inputs: VecDeque<isize> = test_input.chars().map(|c| c.to_digit(10).unwrap() as isize).collect();
        let mut alu = ArithmeticLogicUnit::new(inputs);
        let mut input_count = 0;
        for instruction in &instructions {
            if let Instruction::Input(_) = instruction {
                input_count += 1;
            }
            alu.run(instruction);
        }
        let valid = alu.get(&Variable::Z) == 0;
        println!("{} is {} model number", test_input, if valid { "a VALID" } else { "an INVALID"});
        // trace back to find requirements
        let mut rev_alu = ReverseArithmeticLogicUnit::new(input_count, Variable::Z);
        for instruction in instructions.iter().rev() {
            rev_alu.trace_back(instruction);
        }
        println!("Program required {} of {} inputs: {:?}", rev_alu.required_inputs.len(), input_count, rev_alu.required_inputs);
        println!("Program required {} of {} instructions", rev_alu.required_instructions.len(), instructions.len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
