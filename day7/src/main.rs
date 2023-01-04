use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut numbers: Vec<isize> = text.split(",").map(|n| n.parse().unwrap()).collect();
        numbers.sort();
        let median = median(&numbers);
        let sum_dev: isize = numbers.iter().map(|n| (*n-median as isize).abs()).sum();
        println!("Total fuel: {}", sum_dev);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn median(array: &Vec<isize>)->f64{
    if (array.len() % 2)==0 {
        let ind_left = array.len()/2-1; 
        let ind_right = array.len()/2 ;
        (array[ind_left]+array[ind_right]) as f64 / 2.0

    } else {
            array[(array.len()/2)] as f64
    }
}