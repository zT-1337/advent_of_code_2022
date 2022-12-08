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

        let mut bloom_filter: u64 = 0;
        let middle = line.len() / 2;
        set_bloom_filter(&line[0..middle], &mut bloom_filter);
        sum_of_priority_collisions += u64::from(check_for_collision(
            &line[middle..line.len()],
            &bloom_filter,
        ));
    }

    println!(
        "Result of Advent of Code Day 3, Star 1: {}",
        sum_of_priority_collisions
    );
}

pub fn day_3_star_2() {
    let file = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day3.input");
    let mut sum_of_badge_priorities: u64 = 0;
    let mut lines: Vec<String> = vec![];

    for line in file.lines() {
        let line = match line {
            Ok(value) => value,
            Err(why) => panic!("Could not read line: {}", why),
        };

        if line == "" {
            break;
        }

        lines.push(line);
    }

    if lines.len() % 3 != 0 {
        panic!("Incomplete group of 3 elves");
    }

    let mut i = 0;
    while i < lines.len() {
        let mut first_bloom_filter: u64 = 0;
        let mut second_bloom_filter: u64 = 0;
        let mut third_bloom_filter: u64 = 0;

        set_bloom_filter(&lines[i], &mut first_bloom_filter);
        set_bloom_filter(&lines[i + 1], &mut second_bloom_filter);
        set_bloom_filter(&lines[i + 2], &mut third_bloom_filter);

        let intersection = first_bloom_filter & second_bloom_filter & third_bloom_filter;
        sum_of_badge_priorities += u64::from(search_for_set_bit(intersection));

        i += 3;
    }

    println!(
        "Result of Advent of Code Day 3, Star 2: {}",
        sum_of_badge_priorities
    );
}

fn set_bloom_filter(line: &str, bloom_filter: &mut u64) {
    let line = line.as_bytes();
    for i in 0..line.len() {
        let letter_priority = convert_letter_to_priority(line[i]);
        *bloom_filter |= 1 << (letter_priority - 1);
    }
}

fn search_for_set_bit(intersection: u64) -> u8 {
    let mut intersection = intersection;
    for i in 0..52 {
        if intersection & 1 == 1 {
            return i + 1;
        }

        intersection >>= 1;
    }

    0
}

fn check_for_collision(line: &str, bloom_filter: &u64) -> u8 {
    let line = line.as_bytes();
    for i in 0..line.len() {
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
