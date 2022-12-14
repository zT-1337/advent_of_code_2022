use crate::util::load_lines_of_file;

pub fn day_1_star_1() {
    let lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day1.input");

    let mut current_max: u32 = 0;
    let mut current_val: u32 = 0;

    for line in lines {
        if line == "" {
            current_max = if current_val > current_max {
                current_val
            } else {
                current_max
            };
            current_val = 0;
            continue;
        }

        match line.parse::<u32>() {
            Ok(value) => {
                current_val += value;
            }
            Err(why) => panic!("Could not parse line '{}': {}", line, why),
        };
    }

    println!("Result of Advent of Code Day 1, Star 1: {}", current_max);
}

pub fn day_1_star_2() {
    let lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day1.input");
    let mut top_three_elves = vec![0, 0, 0];
    let mut current_elve = 0;

    for line in lines {
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
            Ok(value) => {
                current_elve += value;
            }
            Err(why) => panic!("Could not parse line '{}': {}", line, why),
        };
    }

    println!(
        "Result of Advent of Code Day 1, Star 2: {}",
        top_three_elves.iter().sum::<u32>()
    );
}
