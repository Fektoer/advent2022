use std::cmp::Ordering;
use std::collections::VecDeque;
use std::time::Instant;

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

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Copy)]
struct PacketString<'a>(&'a str);

impl Ord for PacketString<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut left = parse(&mut self.0.chars().collect::<VecDeque<char>>());
        let mut right = parse(&mut other.0.chars().collect::<VecDeque<char>>());
        let (valid, _) = compare(&mut left, &mut right);
        let ordering = if valid {
            Ordering::Less
        } else {
            Ordering::Greater
        };
        ordering
    }
}

fn main() {
    let start = Instant::now();
    println!("13a {:?} in {:?}", part1(), start.elapsed());
    println!("13b {:?} in {:?}", part2(), start.elapsed());
}

fn part1() -> usize {
    return include_str!("../input.txt")
        .split("\n\n")
        .map(|pair| pair.lines().collect::<Vec<&str>>())
        .enumerate()
        .fold(0, |acc, pair| {
            let mut left = parse(&mut pair.1[0].chars().collect::<VecDeque<char>>());
            let mut right = parse(&mut pair.1[1].chars().collect::<VecDeque<char>>());

            let (valid, _) = compare(&mut left, &mut right);
            let value = if valid { pair.0 + 1 } else { 0 };
            acc + value
        });
}

fn part2() -> usize {
    let divider2 = PacketString("[[2]]");
    let divider6 = PacketString("[[6]]");

    let binding = include_str!("../input.txt").replace("\n\n", "\n");
    let mut packets = binding
        .split("\n")
        .map(|s| PacketString(s))
        .collect::<Vec<PacketString>>();

    packets.push(divider2);
    packets.push(divider6);

    packets.sort_by(|a, b| a.cmp(b));

    let index2 = packets.iter().position(|&r| r == divider2).unwrap();
    let index6 = packets.iter().position(|&r| r == divider6).unwrap();

    return (index2 + 1) * (index6 + 1);
}

fn compare(left: &mut VecDeque<Packet>, right: &mut VecDeque<Packet>) -> (bool, bool) {
    let mut result_found: bool = false;
    let mut valid: bool = true;

    while !result_found {
        if left.len() == 0 {
            if right.len() == 0 {
                return (valid, result_found);
            } else {
                return (true, true);
            }
        } else {
            if let Some(left_packet) = left.pop_front() {
                if right.len() == 0 {
                    valid = false;
                    return (valid, true);
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
                        (valid, result_found) =
                            compare(&mut left_packet.get_array(), &mut right_packet.get_array());
                    }
                    (Packet::Num(_), Packet::Packets(_)) => {
                        let mut vec: VecDeque<Packet> = VecDeque::new();
                        vec.push_back(left_packet.clone());
                        (valid, result_found) = compare(&mut vec, &mut right_packet.get_array());
                    }
                    (Packet::Packets(_), Packet::Num(_)) => {
                        let mut vec: VecDeque<Packet> = VecDeque::new();
                        vec.push_back(right_packet.clone());
                        (valid, result_found) = compare(&mut left_packet.get_array(), &mut vec);
                    }
                }
            }
        }
    }
    return (valid, result_found);
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
