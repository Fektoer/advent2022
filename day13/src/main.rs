use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Num(u32),
    Packets(VecDeque<Packet>),
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
    let binding = include_str!("../input.txt").replace("\n\n", "\n");
    let mut packets = binding
        .split("\n")
        .map(|s| PacketString(s))
        .collect::<Vec<PacketString>>();

    let divider2 = PacketString("[[2]]");
    let divider6 = PacketString("[[6]]");
    packets.extend_from_slice(&[divider2, divider6]);
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
                    (Packet::Num(left_number), Packet::Num(right_number)) => {
                        if !(left_number == right_number) {
                            valid = left_number < right_number;
                            result_found = true;
                        }
                    }

                    (Packet::Packets(mut left_packets), Packet::Packets(mut right_packets)) => {
                        (valid, result_found) = compare(&mut left_packets, &mut right_packets);
                    }

                    (Packet::Num(_), Packet::Packets(mut right_packets)) => {
                        (valid, result_found) =
                            compare(&mut VecDeque::from_iter([left_packet]), &mut right_packets);
                    }

                    (Packet::Packets(mut left_packets), Packet::Num(_)) => {
                        (valid, result_found) =
                            compare(&mut left_packets, &mut VecDeque::from_iter([right_packet]));
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
        }
    }
    vec_char
}
