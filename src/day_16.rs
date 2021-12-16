struct Packet {
    version: u8,
    typ: PacketType,
}

impl Packet {
    fn parse_packet(packet: &str) -> (Self, usize) {
        let version = u8::from_str_radix(&packet[0..3], 2).unwrap();
        let type_id = u8::from_str_radix(&packet[3..6], 2).unwrap();

        if type_id == 4 {
            // literal value
            let mut value: u64 = 0;
            let mut i = 6;
            loop {
                let bits = &packet[i..i + 5];
                let bits_num = u64::from_str_radix(&bits[1..5], 2).unwrap();
                value *= 0b10000;
                value += bits_num;
                i += 5;
                if &bits[0..1] == "0" {
                    break;
                }
            }
            (
                Self {
                    version,
                    typ: PacketType::Literal(value),
                },
                i,
            )
        } else {
            let length_type_id = &packet[6..7];
            if length_type_id == "0" {
                let sub_packets_len = usize::from_str_radix(&packet[7..7 + 15], 2).unwrap();

                let mut sub_packets = vec![];
                let mut size = 0;
                while size < sub_packets_len {
                    let (op, add_size) = Self::parse_packet(&packet[7 + 15 + size..]);
                    size += add_size;
                    sub_packets.push(op);
                }
                (
                    Self {
                        version,
                        typ: PacketType::Operation(Operation::from_type_id(type_id), sub_packets),
                    },
                    sub_packets_len + 7 + 15,
                )
            } else {
                let num_sub_packets = usize::from_str_radix(&packet[7..7 + 11], 2).unwrap();

                let mut sub_packets = vec![];
                let mut size = 0;
                for _ in 0..num_sub_packets {
                    let (op, add_size) = Self::parse_packet(&packet[7 + 11 + size..]);
                    size += add_size;
                    sub_packets.push(op);
                }
                (
                    Self {
                        version,
                        typ: PacketType::Operation(Operation::from_type_id(type_id), sub_packets),
                    },
                    size + 7 + 11,
                )
            }
        }
    }

    fn add_versions(&self) -> u64 {
        let mut total = 0;
        total += self.version as u64;

        if let PacketType::Operation(_, sub_packets) = &self.typ {
            for p in sub_packets {
                total += p.add_versions();
            }
        }
        total
    }

    fn eval(&self) -> u64 {
        match &self.typ {
            PacketType::Literal(val) => *val,
            PacketType::Operation(op, sub_packets) => match op {
                Operation::Sum => sub_packets.iter().map(|p| p.eval()).sum(),
                Operation::Product => sub_packets.iter().map(|p| p.eval()).product(),
                Operation::Minimum => sub_packets.iter().map(|p| p.eval()).min().unwrap(),
                Operation::Maximum => sub_packets.iter().map(|p| p.eval()).max().unwrap(),
                Operation::GreaterThan => {
                    if sub_packets[0].eval() > sub_packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                Operation::LessThan => {
                    if sub_packets[0].eval() < sub_packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                Operation::EqualTo => {
                    if sub_packets[0].eval() == sub_packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

enum PacketType {
    Literal(u64),
    Operation(Operation, Vec<Packet>),
}

enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operation {
    fn from_type_id(type_id: u8) -> Operation {
        use Operation::*;
        match type_id {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            5 => GreaterThan,
            6 => LessThan,
            7 => EqualTo,
            _ => {
                println!("unhandled type id: {}", type_id);
                unreachable!();
            }
        }
    }
}

pub fn run(lines: &[String]) -> (u64, u64) {
    let mut bits = String::new();
    for c in lines[0].chars() {
        bits.push_str(&format!("{:04b}", c.to_digit(16).unwrap()).as_str());
    }

    let packet = Packet::parse_packet(&bits).0;

    (packet.add_versions(), packet.eval())
}
