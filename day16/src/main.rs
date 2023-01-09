use std::env;
use std::fs;
use std::str::FromStr;

struct Packet {
    version: u8,
    type_id: u8,
    number: u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let packet: Packet = text.parse().unwrap();
        println!("Packet version {}, type {}, containing {}", packet.version, packet.type_id, packet.number);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let number = u32::from_str_radix(text, 16).unwrap();
        let binary: String = format_radix(number, 2);
        //110100101111111000101000
        //VVVTTTAAAAABBBBBCCCCC
        let binary_chars: Vec<char> = binary.chars().collect();
        let mut num = Vec::new();
        let mut start = 6;
        while start + 5 < binary_chars.len() {
            for i in (start+1)..(start+5) {
                num.push(binary_chars[i]);
            }
            start += 5;
        }
        let num_str: String = num.iter().collect();
        Ok(Packet {
            version: u8::from_str_radix(&binary[0..2], 2).unwrap(),
            type_id: u8::from_str_radix(&binary[3..5], 2).unwrap(),
            number: u32::from_str_radix(&num_str, 2).unwrap()
        })
    }
}

fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}