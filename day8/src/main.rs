fn main() {
    let tree_grid = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    println!("8a {}", calc_visible_trees(tree_grid.clone()));
    println!("8b {}", calc_max_scenic(tree_grid.clone()));
}
fn calc_max_scenic(tree_grid: Vec<Vec<i32>>) -> i32 {
    let mut scenic_vec: Vec<i32> = Vec::new();

    for row in tree_grid.iter().enumerate() {
        for tree in row.1.iter().enumerate() {
            let mut horizontal_index = tree.0.clone();
            let mut left_visible = true;
            let mut left_count = 0;
            while horizontal_index != 0 && left_visible {
                horizontal_index -= 1;
                left_visible = row.1[horizontal_index] < *tree.1;
                left_count += 1;
            }

            horizontal_index = tree.0.clone();
            let mut right_visible = true;
            let mut right_count = 0;
            while horizontal_index < row.1.len() - 1 && right_visible {
                horizontal_index += 1;
                right_visible = row.1[horizontal_index] < *tree.1;
                right_count += 1;
            }

            let mut vertical_index = row.0.clone();
            let mut top_visible = true;
            let mut top_count = 0;
            while vertical_index != 0 && top_visible {
                vertical_index -= 1;
                top_visible = tree_grid[vertical_index][tree.0] < *tree.1;
                top_count += 1;
            }

            vertical_index = row.0.clone();
            let mut bot_visible = true;
            let mut bot_count = 0;
            while vertical_index < tree_grid.len() - 1 && bot_visible {
                vertical_index += 1;
                bot_visible = tree_grid[vertical_index][tree.0] < *tree.1;
                bot_count += 1;
            }

            scenic_vec.push(left_count * right_count * bot_count * top_count)
        }
    }
    return *scenic_vec.iter().max().unwrap();
}

fn calc_visible_trees(tree_grid: Vec<Vec<i32>>) -> i32 {
    let mut visible = 0;
    for row in tree_grid.iter().enumerate() {
        for tree in row.1.iter().enumerate() {
            // edges
            if tree.0 == 0
                || tree.0 == row.1.len() - 1
                || row.0 == 0
                || row.0 == tree_grid.len() - 1
            {
                visible += 1;
            } else {
                let mut horizontal_index = tree.0.clone();
                let mut left_visible = true;
                while horizontal_index != 0 && left_visible {
                    horizontal_index -= 1;
                    left_visible = row.1[horizontal_index] < *tree.1;
                }

                horizontal_index = tree.0.clone();
                let mut right_visible = true;
                while horizontal_index < row.1.len() - 1 && right_visible {
                    horizontal_index += 1;
                    right_visible = row.1[horizontal_index] < *tree.1;
                }

                let mut vertical_index = row.0.clone();
                let mut top_visible = true;
                while vertical_index != 0 && top_visible {
                    vertical_index -= 1;

                    top_visible = tree_grid[vertical_index][tree.0] < *tree.1
                }

                vertical_index = row.0.clone();
                let mut bot_visible = true;
                while vertical_index < tree_grid.len() - 1 && bot_visible {
                    vertical_index += 1;
                    bot_visible = tree_grid[vertical_index][tree.0] < *tree.1
                }

                if left_visible || right_visible || top_visible || bot_visible {
                    visible += 1;
                }
            }
        }
    }
    return visible;
}
