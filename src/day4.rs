use crate::util::load_lines_of_file;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: &str, end: &str) -> Self {
        let start = start.parse::<u32>().unwrap();
        let end = end.parse::<u32>().unwrap();

        Self { start, end }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.start && other.start <= self.end
            || other.start <= self.start && self.start <= other.end
    }
}

pub fn day_4_star_1_and_2() {
    let lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day4.input");

    let mut sum_of_contains = 0;
    let mut sum_of_overlaps = 0;

    for line in lines {
        if line == "" {
            break;
        }

        let values: Vec<&str> = line.split(&['-', ',']).collect();
        if values.len() != 4 {
            panic!("Expected value len 4 but was {}", values.len())
        }

        let elve_a = Range::new(values[0], values[1]);
        let elve_b = Range::new(values[2], values[3]);

        if elve_a.contains(&elve_b) || elve_b.contains(&elve_a) {
            sum_of_contains += 1;
        }

        if elve_a.overlaps(&elve_b) {
            sum_of_overlaps += 1;
        }
    }

    println!(
        "Result of Advent of Code Day 4, Star 1: {}",
        sum_of_contains
    );
    println!(
        "Result of Advent of Code Day 4, Star 2: {}",
        sum_of_overlaps
    );
}
