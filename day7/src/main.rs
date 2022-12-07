use regex::Regex;

fn main() {
    let mut directories: Vec<(&str, i32)> = Vec::new();
    let mut commands = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    let _line = commands.drain(0..1).next().unwrap();
    recursive_search(&mut commands, &mut directories, "/");

    let sum: i32 = directories
        .iter()
        .filter(|dir| dir.1 <= 100000)
        .map(|dir| dir.1)
        .sum();
    println!("7a: {}", sum);

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

fn recursive_search<'a>(
    commands: &mut Vec<&'a str>,
    directories: &mut Vec<(&'a str, i32)>,
    dir_name: &'a str,
) -> i32 {
    let mut dir_size = 0;

    while commands.len() != 0 {
        let line = commands.drain(0..1).next().unwrap();

        let re_file = Regex::new(r"(\d+)\s").unwrap();
        match re_file.captures(line) {
            Some(caps) => {
                dir_size += caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            }
            None => {}
        }

        let re_cd = Regex::new(r"\$ cd ([a-z]+|/)").unwrap();
        match re_cd.captures(line) {
            Some(caps) => {
                let new_dir = caps.get(1).unwrap().as_str();
                dir_size += recursive_search(commands, directories, new_dir)
            }
            None => {}
        }

        if line == "$ cd .." {
            directories.push((dir_name, dir_size));
            return dir_size;
        }
    }
    directories.push((dir_name, dir_size));
    return dir_size;
}
