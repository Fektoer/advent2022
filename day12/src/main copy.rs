use std::collections::HashMap;
const MAX_DISTANCE: i32 = 9999;
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn get_neighbours(
        &self,
        distances: &mut HashMap<Point, i32>,
        width: i32,
        height: i32,
    ) -> Vec<Point> {
        let mut neighbours: Vec<Point> = Vec::new();
        if self.x as i32 + 1 < width {
            neighbours.push(Point {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.x as i32 - 1 >= 0 {
            neighbours.push(Point {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y as i32 + 1 < height {
            neighbours.push(Point {
                x: self.x,
                y: self.y + 1,
            });
        }
        if self.y as i32 - 1 >= 0 {
            neighbours.push(Point {
                x: self.x,
                y: self.y - 1,
            });
        }

        neighbours.sort_by(|a, b| {
            let a_value = distances.get(a).unwrap();
            let b_value = distances.get(b).unwrap();
            a_value.cmp(b_value)
        });

        neighbours
    }
}

fn main() {
    let mut values = HashMap::new();
    let mut visited = HashMap::new();
    let mut distance = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    include_str!("../input.txt")
        .split("\n")
        .enumerate()
        .for_each(|line| {
            height += 1;
            width = line.1.chars().count();
            line.1.chars().enumerate().for_each(|char| {
                match char.1 {
                    'S' => {
                        start = Point {
                            x: line.0,
                            y: char.0,
                        }
                    }
                    'E' => {
                        end = Point {
                            x: line.0,
                            y: char.0,
                        }
                    }
                    _ => {}
                }
                values.insert(
                    Point {
                        x: line.0,
                        y: char.0,
                    },
                    char.1,
                );
            })
        });

    for value in &values {
        distance.insert(value.0.clone(), MAX_DISTANCE);
        visited.insert(value.0.clone(), false);
    }

    let current_value = values.remove(&start).unwrap() as u32;
    distance.insert(start.clone(), 0);
    visited.insert(start.clone(), true);

    let current_node = start;
    while values.len() > 0 {
        let neighbours = current_node.get_neighbours(&mut distance, width as i32, height as i32);
        if neighbours.len() > 0 {
            for neighbour in neighbours {
                let test = values.get(&neighbour).un;
                let distance_diff: u32 =
                    if *values.get(&neighbour).unwrap() as u32 - current_value > 1 {
                        MAX_DISTANCE as u32
                    } else {
                        1
                    };
                let neighbour_distance = *distance.get(&neighbour).unwrap() as i32;
                let total_distance: i32 = current_value as i32 + distance_diff as i32;
                if total_distance < neighbour_distance {
                    distance.insert(neighbour, total_distance);
                }
            }
        }
        let min_neighbour =
            &current_node.get_neighbours(&mut distance, width as i32, height as i32)[0];
        visited.insert(min_neighbour.clone(), true);
        values.remove(&min_neighbour);
    }

    // println!(
    //     "{:?}",
    //     start.get_neighbours(&mut distance, width as i32, height as i32)
    // );
    //for value in &values {

    println!("done");
}
