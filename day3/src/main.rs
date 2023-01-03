use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: Vec<Vec<usize>>  = text.lines().map(|l| l.chars().map(|c| c as usize - '0' as usize).collect()).collect();
        let mut most = Vec::new();
        let mut least = Vec::new();
        for i in 0..numbers[0].len() {
            if common_bit(&numbers, i, true) == 1 {
                most.push('1');
                least.push('0');
            } else {
                most.push('0');
                least.push('1');
            }
        }
        let gamma = usize::from_str_radix(&most.iter().collect::<String>(), 2).unwrap();
        let epsilon = usize::from_str_radix(&least.iter().collect::<String>(), 2).unwrap();
        println!("Power: {:?} x {:?} = {}", most, least, gamma*epsilon);
        let oxygen = filter_until_single(numbers.clone(), 0, true);
        let co2 = filter_until_single(numbers.clone(), 0, false);
        let oxygen_decimal = usize::from_str_radix(&oxygen.iter().map(|d| (*d as u8 + '0' as u8) as char).collect::<String>(), 2).unwrap();
        let co2_decimal = usize::from_str_radix(&co2.iter().map(|d| (*d as u8 + '0' as u8) as char).collect::<String>(), 2).unwrap();
        println!("Life: {:?} x {:?} = {}", oxygen, co2, oxygen_decimal*co2_decimal);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn common_bit(numbers: &Vec<Vec<usize>>, bit: usize, most: bool) -> usize {
    let n1: usize = numbers.iter().map(|n| n[bit]).sum();
    let n0 = numbers.len() - n1;
    if n1 >= n0 {
        if most {
            1
        } else {
            0
        }
    } else {
        if most {
            0
        } else {
            1
        }
    }
}

fn filter_until_single(numbers: Vec<Vec<usize>>, bit: usize, most: bool) -> Vec<usize> {
    if numbers.len() == 1 {
        numbers.into_iter().next().unwrap()
    } else {
        let target = common_bit(&numbers, bit, most);
        filter_until_single(numbers.into_iter().filter(|n| n[bit] == target).collect(), bit + 1, most)
    }
}