use regex::Regex;
fn main() {
    let (mut stacks, instructions) = initialize_stacks();
    move_stacks(&mut stacks, instructions, true);
    println!("5a: {}", fetch_top_crates(&stacks));

    let (mut stacks, instructions) = initialize_stacks();
    move_stacks(&mut stacks, instructions, false);
    println!("5b: {}", fetch_top_crates(&stacks))
}

fn initialize_stacks() -> (Vec<Vec<String>>, Vec<&'static str>) {
    let lines = include_str!("../input.txt").lines();
    let mut binding = lines.collect::<Vec<&str>>();

    // Get the crates, reverse them because the bottom ones are the first one that need to pushed in the stacks
    // Remainder after the drain is the instructions to move the stacks around
    let mut crates = binding.drain(0..10).rev().collect::<Vec<&str>>();

    // Remove excess lines
    crates.drain(0..2);

    // Initialize n stacks
    let re1 = Regex::new(r"\[([A-Z])\]").unwrap();
    let stack_count = re1.captures_iter(crates[0]).count();
    let mut stacks = vec![Vec::new(); stack_count];

    for c in crates {
        for cap in re1.captures_iter(c) {
            let value = cap.get(1).unwrap().as_str();
            let index = (cap.get(1).unwrap().start() as i32 / 4).abs();
            stacks[index as usize].push(value.to_owned());
        }
    }
    return (stacks, binding);
}

fn move_stacks(stacks: &mut Vec<Vec<String>>, instructions: Vec<&str>, single: bool) {
    let re2 = Regex::new(r"\d+").unwrap();
    for instruction in instructions {
        let values = re2
            .captures_iter(instruction)
            .map(|c| c[0].parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let amount = values[0];
        let source = values[1] - 1;
        let target = values[2] - 1;

        // In case of moving a single crate, just move it directly into the target stack
        if single {
            for _ in 0..amount {
                let value = stacks[source as usize].pop().unwrap();
                stacks[target as usize].push(value);
            }

        // In case of multiple crates at once, move them into a temporary stack first
        // Moving 2 from 1 to 2 becomes
        // A
        // B            B                   B                                   A
        // C            C   A           C   A           C   A   B       C       B
        // 1    2       1   1'  2       1   1'  2       1   1'  2       1   1'  2
        } else {
            let mut temp = Vec::new();
            for _ in 0..amount {
                let value = stacks[source as usize].pop().unwrap();
                temp.push(value);
            }

            for _ in 0..amount {
                let value = temp.pop().unwrap();
                stacks[target as usize].push(value);
            }
        }
    }
}

fn fetch_top_crates(stacks: &Vec<Vec<String>>) -> String {
    let mut result = String::new();
    for stack in stacks {
        if stack.len() > 0 {
            let value = &stack[stack.len() - 1];
            result.push_str(value)
        }
    }
    return result;
}
