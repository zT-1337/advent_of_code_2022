use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    day1_star_1();
    day1_star_2();
}

fn load_lines_of_file(path: &str) -> BufReader<File> {
    let path = Path::new(path);
    let file = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("Failed to open file {}: {}", path.display(), why)
    };

    file
}

fn day1_star_1() {
    let file = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day1.input");

    let mut current_max: u32 = 0;
    let mut current_val: u32 = 0;

    for line in file.lines() {
        let line = match line {
            Ok(value) => value,
            Err(why) => panic!("Reading line failed: {}", why),
        };

        if line == "" {
            current_max = if current_val > current_max {current_val} else {current_max};
            current_val = 0;
            continue;
        }

        match line.parse::<u32>() {
            Ok(value) => {current_val += value;}
            Err(why) => panic!("Could not parse line '{}': {}", line, why),
        };
    }

    println!("Advent of Code Day 1 result: {}", current_max);
}

fn day1_star_2() {
    let file = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day1.input");
    let mut top_three_elves = vec![0, 0, 0];
    let mut current_elve = 0;

    for line in file.lines() {
        let line = match line {
            Ok(value) => value,
            Err(why) => panic!("Reading line failed: {}", why),
        };

        if line == "" {
            for i in 0..top_three_elves.len() {
                if current_elve > top_three_elves[i] {
                    top_three_elves.insert(i, current_elve);
                    top_three_elves.pop();
                    break;
                }
            }

            current_elve = 0;
            continue;
        }

        match line.parse::<u32>() {
            Ok(value) => {current_elve += value;}
            Err(why) => panic!("Could not parse line '{}': {}", line, why),
        };
    }

    println!("Advent of Code Day 2 result: {}", top_three_elves.iter().sum::<u32>());
}
