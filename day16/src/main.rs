use std::env;
use std::fs;
use std::str::FromStr;

struct Packet {
    version: u8,
    type_id: u8,
    number: u128
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
        let binary = hex_to_binary(text);
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
            number: u128::from_str_radix(&num_str, 2).unwrap()
        })
    }
}

fn hex_to_binary(hex_str: &str) -> String {
    let mut result = vec![];
    for hex_c in hex_str.chars() {
        let bin_str = match hex_c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("Invalid char: {}", hex_c)
        };
        for bin_c in bin_str.chars() {
            result.push(bin_c);
        }
    }
    result.into_iter().collect()
}