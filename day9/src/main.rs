use std::collections::HashMap;
fn main() {
    let commands = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let temp = l.split_once(" ").unwrap();
            (temp.0, temp.1.parse::<i32>().unwrap())
        })
        .collect::<Vec<(&str, i32)>>();

    println!("9a {}", calculate_path(commands.clone(), 2));
    println!("9b {}", calculate_path(commands.clone(), 10));
}

fn calculate_path(commands: Vec<(&str, i32)>, num_knots: usize) -> i32 {
    let mut path_travelled: Vec<(i32, i32)> = Vec::new();
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); num_knots];

    for command in commands {
        // Split each movement command into steps of 1
        for _ in 0..command.1 {
            match command.0 {
                "R" => knots[0].0 += 1,
                "U" => knots[0].1 += 1,
                "L" => knots[0].0 -= 1,
                "D" => knots[0].1 -= 1,
                _ => unreachable!(),
            }

            // Move all the other knots in comparison to its predecessor
            for index in 0..knots.len() - 1 {
                let store_path_travelled = index == (knots.len() - 2);
                let first_kot = knots[index];
                let next_knot = &mut knots[index + 1];
                move_knot(
                    &first_kot,
                    next_knot,
                    store_path_travelled,
                    &mut path_travelled,
                );
            }
        }
    }

    // We're only interested in unique values
    let mut positions = HashMap::new();
    for position in path_travelled {
        positions.insert(position, true);
    }

    return positions.len() as i32;
}

fn move_knot(
    head: &(i32, i32),
    trail: &mut (i32, i32),
    store_path_travelled: bool,
    path_travelled: &mut Vec<(i32, i32)>,
) {
    // horizontal movement
    // 0,0 to 2,0 -> 1,0
    // 0,0 to -2,0 -> -1,0
    if head.1 == trail.1 {
        for _ in 0..(head.0 - trail.0).abs() - 1 {
            trail.0 += if head.0 > trail.0 { 1 } else { -1 };
        }
    }
    // vertical movement
    else if head.0 == trail.0 {
        // 0,0 to 0,2 -> 0,1
        // 0,0 to 0,-2 -> 0,-1
        for _ in 0..(head.1 - trail.1).abs() - 1 {
            trail.1 += if head.1 > trail.1 { 1 } else { -1 };
        }
    }
    // diagonal
    else {
        // Bishop hop: 0,0 to 2,2 -> 1,1
        //             0,0 to 2,-2 -> 1,-1
        //             0,0 to -2,-2 -> -1,-1
        //             0,0 to -2,2 -> -1,1
        if (head.1 - trail.1).abs() > 1 && (head.0 - trail.0).abs() > 1 {
            trail.1 += if head.1 > trail.1 { 1 } else { -1 };
            trail.0 += if head.0 > trail.0 { 1 } else { -1 };

        // Knight hop: 0,0 to 1,2 -> 1,1
        //             0,0 to 2,-1 -> 1,-1
        //             0,0 to -1,2 -> -1,1
        //             0,0 to -2,-1 -> -1,-1
        //             etc
        } else {
            if (head.1 - trail.1).abs() > 1 {
                trail.0 = head.0;
                for _ in 0..(head.1 - trail.1).abs() - 1 {
                    trail.1 += if head.1 > trail.1 { 1 } else { -1 };
                }
            } else if (head.0 - trail.0).abs() > 1 {
                trail.1 = head.1;
                for _ in 0..(head.0 - trail.0).abs() - 1 {
                    trail.0 += if head.0 > trail.0 { 1 } else { -1 };
                }
            }
        }
    }
    if store_path_travelled {
        path_travelled.push(*trail);
    }
}
