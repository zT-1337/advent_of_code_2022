use crate::util::load_lines_of_file;
use std::io::BufRead;

#[derive(Clone)]
enum RpsChoice {
    Rock,
    Paper,
    Scissor,
}

impl RpsChoice {
    fn convert_from(from: &str) -> Self {
        match from {
            "A" => RpsChoice::Rock,
            "B" => RpsChoice::Paper,
            "C" => RpsChoice::Scissor,
            "X" => RpsChoice::Rock,
            "Y" => RpsChoice::Paper,
            "Z" => RpsChoice::Scissor,
            _ => panic!("Unexpected from: {}", from),
        }
    }

    fn convert_from_opponent_and_result(opponent: &RpsChoice, result: &str) -> Self {
        match (opponent, result) {
            (RpsChoice::Rock, "X") => RpsChoice::Scissor,
            (RpsChoice::Rock, "Z") => RpsChoice::Paper,
            (RpsChoice::Paper, "X") => RpsChoice::Rock,
            (RpsChoice::Paper, "Z") => RpsChoice::Scissor,
            (RpsChoice::Scissor, "X") => RpsChoice::Paper,
            (RpsChoice::Scissor, "Z") => RpsChoice::Rock,
            (_, "Y") => opponent.clone(),
            (_, _) => panic!("Unexpected result: {}", result),
        }
    }

    fn points_for_choice(&self) -> u32 {
        match self {
            RpsChoice::Rock => 1,
            RpsChoice::Paper => 2,
            RpsChoice::Scissor => 3,
        }
    }

    fn points_for_outcome(&self, opponent: &RpsChoice) -> u32 {
        match (self, opponent) {
            (RpsChoice::Rock, RpsChoice::Paper) => 0,
            (RpsChoice::Rock, RpsChoice::Scissor) => 6,
            (RpsChoice::Paper, RpsChoice::Scissor) => 0,
            (RpsChoice::Paper, RpsChoice::Rock) => 6,
            (RpsChoice::Scissor, RpsChoice::Rock) => 0,
            (RpsChoice::Scissor, RpsChoice::Paper) => 6,
            (_, _) => 3,
        }
    }
}

pub fn day_2_star_1() {
    let file = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day2.input");

    let mut score: u32 = 0;

    for line in file.lines() {
        let line = match line {
            Ok(value) => value,
            Err(why) => panic!("Reading line failed: {}", why),
        };

        if line.len() != 3 {
            break;
        }

        let opponent = RpsChoice::convert_from(&line[0..1]);
        let myself = RpsChoice::convert_from(&line[2..3]);

        score += myself.points_for_choice();
        score += myself.points_for_outcome(&opponent);
    }

    println!("Result of Advent of Code Day 2, Star 1: {}", score);
}

pub fn day_2_star_2() {
    let file = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day2.input");

    let mut score: u32 = 0;

    for line in file.lines() {
        let line = match line {
            Ok(value) => value,
            Err(why) => panic!("Reading line failed: {}", why),
        };

        if line.len() != 3 {
            break;
        }

        let opponent = RpsChoice::convert_from(&line[0..1]);
        let myself = RpsChoice::convert_from_opponent_and_result(&opponent, &line[2..3]);

        score += myself.points_for_choice();
        score += myself.points_for_outcome(&opponent);
    }

    println!("Result of Advent of Code Day 2, Star 2: {}", score);
}
