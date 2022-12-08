use regex::Regex;

fn main() {
    let mut directories: Vec<(&str, i32)> = Vec::new();
    let mut commands = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    search_folder(&mut commands, &mut directories, "");

    // 7a: Filter all directories with size <= 10000 and sum those
    let sum: i32 = directories
        .iter()
        .filter(|dir| dir.1 <= 100000)
        .map(|dir| dir.1)
        .sum();
    println!("7a: {}", sum);

    // 7b: Find the first folder we can delete to have enough space for the update
    directories.sort_by(|a, b| a.1.cmp(&b.1));
    let total_free = 70000000 - directories.last().unwrap().1;
    let total_needed = 30000000 - total_free;

    let min_size = directories
        .into_iter()
        .find(|dir| dir.1 > total_needed)
        .unwrap()
        .1;
    println!("7b: {}", min_size);
}

fn search_folder<'a>(
    commands: &mut Vec<&'a str>,
    directories: &mut Vec<(&'a str, i32)>,
    root: &'a str,
) -> i32 {
    // Initialize the directory size to 0
    let mut dir_size = 0;

    while commands.len() != 0 {
        // Drain the next command
        let line = commands.drain(0..1).next().unwrap();

        // If it's a file, add the file size to the directory total
        let re_file = Regex::new(r"(\d+)\s").unwrap();
        match re_file.captures(line) {
            Some(caps) => {
                dir_size += caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            }
            None => {}
        }

        // If it's a change directory command, change the directory and start recursion
        let re_cd = Regex::new(r"\$ cd ([a-z]+|/)").unwrap();
        match re_cd.captures(line) {
            Some(caps) => {
                let new_root = caps.get(1).unwrap().as_str();
                dir_size += search_folder(commands, directories, new_root)
            }
            None => {}
        }

        // If we move out one level, push the directory plus it's size to storage and exit the recursion,
        // returning the size of this directory and all underlying ones
        if line == "$ cd .." {
            directories.push((root, dir_size));
            return dir_size;
        }

        // Ignore all other commands, they don't impact the outcome
    }

    // No more commands, push what we have to storage and exit recursion
    directories.push((root, dir_size));
    return dir_size;
}
