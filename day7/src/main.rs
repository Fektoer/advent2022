use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut directories: HashMap<&str, i32> = HashMap::new();
    let mut commands = include_str!("../temp2.txt").lines().collect::<Vec<&str>>();

    recursive_search(&mut commands, &mut directories, &mut "", &mut 0);

    for (key, value) in directories {
        println!("{}: {}", key, value);
    }
}

fn recursive_search<'a>(
    commands: &mut Vec<&'a str>,
    directories: &mut HashMap<&'a str, i32>,
    dir_name: &mut str,
    file_size: &mut i32,
) {
    if commands.len() == 0 {
        return;
    }
    let line = commands.drain(0..1).next().unwrap();

    let re_dir = Regex::new(r"^dir ([a-z]+)").unwrap();
    match re_dir.captures(line) {
        Some(_) => recursive_search(commands, directories, dir_name, file_size),
        None => {}
    }

    let re_file = Regex::new(r"(\d+)\s").unwrap();
    match re_file.captures(line) {
        Some(caps) => {
            let size = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            *file_size = *file_size + size;
            recursive_search(commands, directories, dir_name, file_size)
        }
        None => {}
    }

    let re_cd = Regex::new(r"\$ cd ([a-z]+|/)").unwrap();
    match re_cd.captures(line) {
        Some(caps) => {
            let mut test = caps.get(1).unwrap().as_str();
            dir_name = test;
            recursive_search(commands, directories, dir_name, file_size)
        }
        None => {}
    }

    if line == "$ cd .." {
        directories.insert(dir_name, *file_size);
        recursive_search(commands, directories, dir_name, &mut 0)
    }

    if line == "$ ls" {
        recursive_search(commands, directories, dir_name, file_size)
    }
}
