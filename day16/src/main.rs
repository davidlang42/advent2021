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
    Operator(Vec<Packet>)
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
            Message::Literal(value) => writeln!(f, "{}", value)?,
            Message::Operator(sub_packets) => {
                for sub_packet in sub_packets {
                    writeln!(f, "{}", sub_packet)?;
                }
            }
        }
        Ok(())
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
        let mut sum = self.version as usize;
        if let Message::Operator(sub_packets) = &self.message {
            for sub_packet in sub_packets {
                sum += sub_packet.version_sum();
            }
        }
        sum
    }
}

impl Message {
    fn from_stream(stream: &mut dyn Iterator<Item = char>, type_id: u8) -> Result<Self, String> {
        Ok(match type_id {
            4 => Message::Literal(Self::read_literal(stream)?),
            _ => Message::Operator({
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
                sub_packets
            })
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