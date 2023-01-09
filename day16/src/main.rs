use std::env;
use std::fs;
use std::fmt::Display;
use std::fmt::Formatter;

struct Packet {
    version: u8,
    type_id: u8,
    message: Message
}

enum Message {
    Literal(u128),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    GreaterThan(Box<Packet>, Box<Packet>),
    LessThan(Box<Packet>, Box<Packet>),
    EqualTo(Box<Packet>, Box<Packet>)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        for line in text.lines() {
            let binary = hex_to_binary(line);
            let packet = Packet::from_stream(&mut binary.chars()).unwrap();
            println!("{}", packet);
            println!("Version Sum: {}", packet.version_sum());
            println!("Value: {}", packet.value());
            println!("");
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        const TAB: &str = "  ";
        let type_name = match self.type_id {
            4 => "Literal".to_string(),
            id => format!("Operator({})", id)
        };
        writeln!(f, "{} Packet v{}:", type_name, self.version)?;
        let message = format!("{}", self.message);
        let lines: Vec<String> = message.lines().map(|l| format!("{}{}", TAB, l)).collect();
        write!(f, "{}", lines.join("\r\n"))?;
        Ok(())
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Message::Literal(value) => writeln!(f, "{}", value),
            Message::Sum(packets) => writeln!(f, "{}", packets.iter().map(|p| format!("{}", p)).collect::<Vec<String>>().join("\r\n")),
            Message::Product(packets) => writeln!(f, "{}", packets.iter().map(|p| format!("{}", p)).collect::<Vec<String>>().join("\r\n")),
            Message::Min(packets) => writeln!(f, "{}", packets.iter().map(|p| format!("{}", p)).collect::<Vec<String>>().join("\r\n")),
            Message::Max(packets) => writeln!(f, "{}", packets.iter().map(|p| format!("{}", p)).collect::<Vec<String>>().join("\r\n")),
            Message::GreaterThan(a, b) => writeln!(f, "{}\r\n{}", a, b),
            Message::LessThan(a, b) => writeln!(f, "{}\r\n{}", a, b),
            Message::EqualTo(a, b) => writeln!(f, "{}\r\n{}", a, b),
        }
    }
}

impl Packet {
    fn from_stream(stream: &mut dyn Iterator<Item = char>) -> Result<Self, String> {
        let version = u8::from_str_radix(&stream.take(3).collect::<String>(), 2).unwrap();
        let type_id = u8::from_str_radix(&stream.take(3).collect::<String>(), 2).unwrap();
        let message = Message::from_stream(stream, type_id)?;
        Ok(Self { version, type_id, message })
    }

    fn version_sum(&self) -> usize {
        self.version as usize + self.message.version_sum()
    }

    fn value(&self) -> u128 {
        self.message.value()
    }
}

impl Message {
    fn from_stream(stream: &mut dyn Iterator<Item = char>, type_id: u8) -> Result<Self, String> {
        Ok(match type_id {
            4 => Message::Literal(Self::read_literal(stream)?),
            _ => {
                let length_bit = stream.next().unwrap();
                let mut sub_packets = Vec::new();
                match length_bit {
                    '0' => {
                        let total_length = u16::from_str_radix(&stream.take(15).collect::<String>(), 2).unwrap();
                        let mut remaining: Vec<char> = stream.take(total_length.into()).collect();
                        while remaining.len() > 0 {
                            let mut remaining_stream = remaining.into_iter();
                            sub_packets.push(Packet::from_stream(&mut remaining_stream).unwrap());
                            remaining = remaining_stream.collect();
                        }
                    },
                    '1' => {
                        let total_sub_packets = u16::from_str_radix(&stream.take(11).collect::<String>(), 2).unwrap();
                        for _ in 0..total_sub_packets {
                            sub_packets.push(Packet::from_stream(stream).unwrap())
                        }
                    },
                    _ => return Err(format!("Invalid length bit: {}", length_bit))
                }
                match type_id {
                    0 => Message::Sum(sub_packets),
                    1 => Message::Product(sub_packets),
                    2 => Message::Min(sub_packets),
                    3 => Message::Max(sub_packets),
                    _ => {
                        if sub_packets.len() != 2 {
                            return Err(format!("Comparsion operators require exactly 2 sub-packets"));
                        }
                        let mut iter = sub_packets.into_iter();
                        let a = iter.next().unwrap();
                        let b = iter.next().unwrap();
                        match type_id {
                            5 => Message::GreaterThan(Box::new(a), Box::new(b)),
                            6 => Message::LessThan(Box::new(a), Box::new(b)),
                            7 => Message::EqualTo(Box::new(a), Box::new(b)),
                            _ => return Err(format!("Invalid type id: {}", type_id))
                        }
                    }
                }
            }
        })
    }

    fn read_literal(stream: &mut dyn Iterator<Item = char>) -> Result<u128, String> {
        let mut num = Vec::new();
        let mut last_byte = false;
        while !last_byte {
            last_byte = stream.next().unwrap() == '0';
            for _ in 0..4 {
                num.push(stream.next().unwrap());
            }
        }
        let num_str: String = num.iter().collect();
        let num_val = u128::from_str_radix(&num_str, 2).unwrap();
        Ok(num_val)
    }

    fn value(&self) -> u128 {
        match self {
            Message::Literal(literal) => *literal,
            Message::Sum(packets) => packets.iter().map(|p| p.value()).sum::<u128>(),
            Message::Product(packets) => packets.iter().map(|p| p.value()).product::<u128>(),
            Message::Min(packets) => packets.iter().map(|p| p.value()).min().unwrap(),
            Message::Max(packets) => packets.iter().map(|p| p.value()).max().unwrap(),
            Message::GreaterThan(a, b) => if a.value() > b.value() { 1 } else { 0 },
            Message::LessThan(a, b) => if a.value() < b.value() { 1 } else { 0 },
            Message::EqualTo(a, b) => if  a.value() == b.value() { 1 } else { 0 }
        }
    }

    fn version_sum(&self) -> usize {
        match self {
            Message::Literal(_) => 0,
            Message::Sum(packets) => packets.iter().map(|p| p.version_sum()).sum::<usize>(),
            Message::Product(packets) => packets.iter().map(|p| p.version_sum()).sum::<usize>(),
            Message::Min(packets) => packets.iter().map(|p| p.version_sum()).sum::<usize>(),
            Message::Max(packets) => packets.iter().map(|p| p.version_sum()).sum::<usize>(),
            Message::GreaterThan(a, b) => a.version_sum() + b.version_sum(),
            Message::LessThan(a, b) => a.version_sum() + b.version_sum(),
            Message::EqualTo(a, b) => a.version_sum() + b.version_sum()
        }
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