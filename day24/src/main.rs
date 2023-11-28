use std::env;
use std::fs;

mod instructions;
mod alu;
mod functions;

use instructions::Variable;

use crate::instructions::Instruction;
use crate::alu::ArithmeticLogicUnit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.lines().map(|l| l.parse().unwrap()).collect();

        let mut count = 0;
        let factor = 100.0/((9.0_f64).powf(6.0));
        for a in (1..10).rev() {
            for b in (1..10).rev() {
                for c in (1..10).rev() {
                    for d in (1..10).rev() {
                        for e in (1..10).rev() {
                            for f in (1..10).rev() {
                                for g in (1..10).rev() {
                                    for h in (1..10).rev() {
                                        for i in (1..10).rev() {
                                            for j in (1..10).rev() {
                                                for k in (1..10).rev() {
                                                    for l in (1..10).rev() {
                                                        for m in (1..10).rev() {
                                                            for n in (1..10).rev() {
                                                                if test_model_number([a, b, c, d, e, f, g, h, i, j, k, l, m, n], &instructions) {
                                                                    println!("VALID: {}{}{}{}{}{}{}{}{}{}{}{}{}{}", a, b, c, d, e, f, g, h, i, j, k, l, m, n);
                                                                    return;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                count += 1;
                                println!("{:.4}%", (count as f64)*factor);
                            }
                        }
                    }
                }
            }
        }
        
        println!("Not found.");
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn test_model_number(inputs: [isize; 14], instructions: &Vec<Instruction>) -> bool {
    let mut alu = ArithmeticLogicUnit::new(inputs.into_iter().collect());
    for instruction in instructions {
        alu.run(instruction);
    }
    alu.get(&Variable::Z) == 0
}