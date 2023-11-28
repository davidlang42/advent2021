use std::env;
use std::fs;
use std::collections::VecDeque;

mod instructions;
mod alu;
mod functions;

use crate::instructions::{Instruction, Variable};
use crate::alu::{ArithmeticLogicUnit, ReverseArithmeticLogicUnit, FunctionalArithmeticLogicUnit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.lines().map(|l| l.parse().unwrap()).collect();
        let from: usize = args[2].parse().unwrap();
        let to: usize = args[3].parse().unwrap();
        println!("Searching from {} to {} (descending)", from, to);

        for x in (to..(from+1)).rev() {
            let x_str: String = format!("{}", x);
            let inputs: VecDeque<isize> = x_str.chars().map(|c| c.to_digit(10).unwrap() as isize).collect();
            if inputs.iter().all(|&i| i != 0) {
                let mut alu = ArithmeticLogicUnit::new(inputs);
                for instruction in &instructions {
                    alu.run(instruction);
                }
                if alu.get(&Variable::Z) == 0 {
                    println!("{} is a VALID model number", x);
                    break;
                } else {
                    //println!("{} is an INVALID model number", x);
                }
            } else {
                //println!("Skipping {} because it contains a ZERO", x);
            }
        }
        println!("Not found.");
    } else {
        println!("Please provide 3 argument: Filename From To");
    }
}
