use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Num(u32),
    Packets(VecDeque<Packet>),
}

impl Packet {
    fn get_number(&self) -> u32 {
        if let Packet::Num(c) = self {
            *c
        } else {
            panic!("Not a number")
        }
    }

    fn get_array(&self) -> VecDeque<Packet> {
        if let Packet::Packets(c) = self {
            c.clone()
        } else {
            panic!("Not an array")
        }
    }
}

fn main() {
    let sum = include_str!("../sample2.txt")
        .split("\n\n")
        .map(|pair| pair.lines().collect::<Vec<&str>>())
        .enumerate()
        .fold(0, |acc, pair| {
            let mut left = parse(&mut pair.1[0].chars().collect::<VecDeque<char>>());
            let mut right = parse(&mut pair.1[1].chars().collect::<VecDeque<char>>());

            let value = if compare(&mut left, &mut right) {
                pair.0 + 1
            } else {
                0
            };
            acc + value
        });
    println!("{:?}", sum);
}

fn compare(left: &mut VecDeque<Packet>, right: &mut VecDeque<Packet>) -> bool {
    let mut result_found: bool = false;
    let mut valid: bool = true;

    if left.len() == 0 && right.len() != 0 {
        valid = true;
    } else {
        while left.len() != 00 && !result_found {
            if let Some(left_packet) = left.pop_front() {
                if right.len() == 0 {
                    valid = false;
                    return valid;
                }
                let right_packet = right.pop_front().unwrap();

                match (left_packet.clone(), right_packet.clone()) {
                    (Packet::Num(_), Packet::Num(_)) => {
                        if !(left_packet.get_number() == right_packet.get_number()) {
                            valid = left_packet.get_number() < right_packet.get_number();
                            result_found = true;
                        }
                    }
                    (Packet::Packets(_), Packet::Packets(_)) => {
                        valid =
                            compare(&mut left_packet.get_array(), &mut right_packet.get_array());
                    }
                    (Packet::Num(_), Packet::Packets(_)) => {
                        let mut vec: VecDeque<Packet> = VecDeque::new();
                        vec.push_back(left_packet.clone());
                        valid = compare(&mut vec, &mut right_packet.get_array());
                    }
                    (Packet::Packets(_), Packet::Num(_)) => {
                        let mut vec: VecDeque<Packet> = VecDeque::new();
                        vec.push_back(right_packet.clone());
                        valid = compare(&mut left_packet.get_array(), &mut vec);
                    }
                }
            }
        }
    }
    return valid;
}

fn parse(input: &mut VecDeque<char>) -> VecDeque<Packet> {
    let mut vec_char: VecDeque<Packet> = VecDeque::new();
    let mut digit_char: Vec<char> = Vec::new();
    while input.len() > 0 {
        if let Some(char) = input.pop_front() {
            match char {
                '[' => vec_char.push_back(Packet::Packets(parse(input))),
                ',' => {
                    if digit_char.len() > 0 {
                        let digit_string: String = digit_char.iter().collect();
                        vec_char.push_back(Packet::Num(digit_string.parse::<u32>().unwrap()));
                        digit_char.clear();
                    }
                }
                ']' => {
                    if digit_char.len() > 0 {
                        let digit_string: String = digit_char.iter().collect();
                        vec_char.push_back(Packet::Num(digit_string.parse::<u32>().unwrap()));
                        digit_char.clear();
                    }
                    return vec_char;
                }
                _ => {
                    digit_char.push(char);
                }
            }
        } else {
        };
    }
    vec_char
}
