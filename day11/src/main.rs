use regex::Regex;
use std::time::Instant;

struct Monkey {
    items: Vec<i64>,
    inspect_modifier: i64,
    inspect_operator: char,
    test_value: i64,
    true_monkey: usize,
    false_monkey: usize,
    num_inspect: i64,
}

impl Monkey {
    fn has_items(&self) -> bool {
        self.items.len() > 0
    }

    fn inspect(&mut self, lcm: i64) {
        let item = self.items.first().unwrap();
        let modifier = if self.inspect_modifier == 0 {
            item
        } else {
            &self.inspect_modifier
        };
        let mut value = match self.inspect_operator {
            '*' => item * modifier,
            '+' => item + modifier,
            _ => unreachable!(),
        };

        if lcm > 0 {
            value = value % lcm;
        }

        self.items[0] = value;
        self.num_inspect += 1;
    }

    fn is_bored(&mut self) {
        let value = (*self.items.first().unwrap() as f64 / 3.0).floor() as i64;
        self.items[0] = value;
    }

    fn throw_shit(&mut self) -> (usize, i64) {
        let item = self.items.drain(0..1).collect::<Vec<i64>>()[0];
        if item % self.test_value == 0 {
            (self.true_monkey, item)
        } else {
            (self.false_monkey, item)
        }
    }

    fn catch_shit(&mut self, item: i64) {
        self.items.push(item);
    }
}

fn main() {
    let start = Instant::now();
    println!(
        "11a {:?} in {:?}",
        calculate_monkey_business(20, false),
        start.elapsed()
    );
    println!(
        "11b {:?} in {:?}",
        calculate_monkey_business(10000, true),
        start.elapsed()
    );
}

fn calculate_monkey_business(rounds: i32, use_lcm: bool) -> i64 {
    let mut monkeys = parse_monkeys();
    let mut shit: Vec<(usize, i64)> = Vec::new();

    let lcm = monkeys.iter().fold(1, |mut acc, curr| {
        acc *= curr.test_value;
        acc
    });

    for _ in 0..rounds {
        for index in 0..monkeys.len() {
            let monkey = &mut monkeys[index];
            while monkey.has_items() {
                if use_lcm {
                    monkey.inspect(lcm);
                } else {
                    monkey.inspect(0);
                    monkey.is_bored();
                }
                shit.push(monkey.throw_shit());
            }
            while shit.len() > 0 {
                let (target, item) = shit.drain(0..1).collect::<Vec<(usize, i64)>>()[0];
                monkeys[target].catch_shit(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.num_inspect.cmp(&a.num_inspect));
    return monkeys[0].num_inspect * monkeys[1].num_inspect;
}

fn parse_monkeys() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    include_str!("../input.txt")
        .split("\n\n")
        .for_each(|monkey_batch| {
            let monkey_lines = monkey_batch.split("\n").collect::<Vec<&str>>();

            let mut items: Vec<i64> = Vec::new();
            let re_items = Regex::new(r"(\d+)").unwrap();
            re_items.find_iter(monkey_lines[1]).for_each(|cap| {
                items.push(cap.as_str().parse::<i64>().unwrap());
            });

            let mut inspect_operator = '\0';
            let mut inspect_modifier: i64 = 0;
            let re_operator = Regex::new(r"(\+|\*)\s(\d+|old)").unwrap();
            match re_operator.captures(monkey_lines[2]) {
                Some(caps) => {
                    inspect_operator = caps.get(1).unwrap().as_str().parse::<char>().unwrap();
                    inspect_modifier = if caps.get(2).unwrap().as_str() == "old" {
                        0
                    } else {
                        caps.get(2).unwrap().as_str().parse::<i64>().unwrap()
                    };
                }
                None => {}
            }

            let mut test_value = 0;
            let re_test = Regex::new(r"(\d+)").unwrap();
            match re_test.captures(monkey_lines[3]) {
                Some(caps) => {
                    test_value = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
                }
                None => {}
            }

            let mut true_monkey: usize = 0;
            let re_test = Regex::new(r"(\d+)").unwrap();
            match re_test.captures(monkey_lines[4]) {
                Some(caps) => {
                    true_monkey = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                }
                None => {}
            }

            let mut false_monkey: usize = 0;
            let re_test = Regex::new(r"(\d+)").unwrap();
            match re_test.captures(monkey_lines[5]) {
                Some(caps) => {
                    false_monkey = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                }
                None => {}
            }

            let monkey = Monkey {
                items,
                inspect_operator,
                inspect_modifier,
                test_value,
                true_monkey,
                false_monkey,
                num_inspect: 0,
            };

            monkeys.push(monkey);
        });
    monkeys
}
