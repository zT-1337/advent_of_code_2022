use std::io::BufRead;

use crate::util::load_lines_of_file;

pub fn day_3_star_1() {
    let file = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day3.input");

    let mut sum_of_priority_collisions: u64 = 0;

    for line in file.lines() {
        let line = match line {
            Ok(value) => value,
            Err(why) => panic!("Could not read line: {}", why),
        };

        if line == "" {
            break;
        }

        sum_of_priority_collisions += u64::from(check_for_collision(&line));
    }

    println!("Result of Advent of Code Day 3, Star 1: {}", sum_of_priority_collisions);
}

fn check_for_collision(line: &str) -> u8 {
    let line = line.as_bytes();
    let middle = line.len() / 2;
    let mut bloom_filter: u64 = 0;
    
    for i in 0..middle {
        let letter_priority = convert_letter_to_priority(line[i]);
        bloom_filter |= 1 << (letter_priority - 1);
    }

    for i in middle..line.len() {
        let letter_priority = convert_letter_to_priority(line[i]);
        if ((bloom_filter >> (letter_priority - 1)) & 1) == 1 {
            return letter_priority;
        }
    }

    0
}

fn convert_letter_to_priority(letter_code: u8) -> u8 {
    if letter_code >= 97 && letter_code <= 122 {
        return letter_code - 96;
    }

    if letter_code >= 65 && letter_code <= 90 {
        return letter_code - 64 + 26;
    }

    panic!("Invalid letter code: {}", letter_code);
}
