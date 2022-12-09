use std::collections::HashMap;

use crate::util::load_lines_of_file;

pub fn day_6_star_1_and_2() {
    let result = detect_marker(4);
    println!("Result of Advent of Code Day 6, Star 1: {}", result);
    let result = detect_marker(14);
    println!("Result of Advent of Code Day 6, Star 2: {}", result);
}

fn detect_marker(marker_length: usize) -> usize {
    let line = &load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day6.input")[0];

    if line.len() < marker_length {
        panic!(
            "Line needs to be {} chars long, but was {}",
            marker_length,
            line.len()
        );
    }

    let mut letter_counter: HashMap<&str, usize> = HashMap::with_capacity(marker_length);
    for i in 0..(marker_length - 1) {
        let letter = &line[i..i + 1];
        raise_letter_count(&mut letter_counter, letter);
    }

    for i in (marker_length - 1)..line.len() {
        let head = &line[i..i + 1];
        raise_letter_count(&mut letter_counter, head);

        if is_only_unique_letters(&letter_counter) {
            return i + 1;
        }

        let tail = &line[i - (marker_length - 1)..i - (marker_length - 2)];
        lower_letter_count(&mut letter_counter, tail);
    }

    0
}

fn raise_letter_count<'a>(letter_counter: &mut HashMap<&'a str, usize>, letter: &'a str) {
    match letter_counter.get_mut(letter) {
        Some(value) => {
            *value += 1;
        }
        None => {
            letter_counter.insert(letter, 1);
        }
    };
}

fn lower_letter_count<'a>(letter_counter: &mut HashMap<&'a str, usize>, letter: &'a str) {
    match letter_counter.get_mut(letter) {
        Some(value) => {
            *value -= 1;
        }
        None => panic!("Letter is not inserted: {}", letter),
    };
}

fn is_only_unique_letters(letter_counter: &HashMap<&str, usize>) -> bool {
    for (_, value) in letter_counter {
        if *value > 1 {
            return false;
        }
    }

    return true;
}
