use std::collections::HashMap;

use crate::util::{
    load_lines_of_file, Vec2d, DOWN_DIRECTION, LEFT_DIRECTION, RIGHT_DIRECTION, UP_DIRECTION,
};

struct Command {
    dir: &'static Vec2d,
    amount: u8,
}

impl Command {
    fn from_line(line: &str) -> Self {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() != 2 {
            panic!("Expected 2 tokens but was: {} {:?}", tokens.len(), tokens);
        }

        let dir = match tokens[0] {
            "R" => &RIGHT_DIRECTION,
            "L" => &LEFT_DIRECTION,
            "U" => &UP_DIRECTION,
            "D" => &DOWN_DIRECTION,
            _ => panic!("Unexpected direction: {}", tokens[0]),
        };

        let amount = match tokens[1].parse::<u8>() {
            Ok(value) => value,
            Err(why) => panic!("{}", why),
        };

        Self { dir, amount }
    }
}

struct Rope {
    knots: Vec<Vec2d>,
}

impl Rope {
    fn new(knot_count: usize) -> Self {
        if knot_count < 2 {
            panic!("Rope needs atleast two knots");
        }

        let mut result = Self {
            knots: Vec::with_capacity(knot_count),
        };

        for _ in 0..knot_count {
            result.knots.push(Vec2d::new(0, 0));
        }

        result
    }

    fn move_in_direction(&mut self, direction: &Vec2d) {
        self.head_mut().add_to_self(direction);

        for current_knot_index in 1..self.knots.len() {
            let prev_knot = &self.knots[current_knot_index - 1].clone();
            let current_knot = &mut self.knots[current_knot_index];

            if current_knot.is_touching(prev_knot) {
                break;
            }

            let mut current_knot_direction = Vec2d::subtract(prev_knot, current_knot);
            current_knot_direction.clamp(-1, 1);
            current_knot.add_to_self(&current_knot_direction);
        }
    }

    fn tail(&self) -> &Vec2d {
        self.knots.last().unwrap()
    }

    fn head_mut(&mut self) -> &mut Vec2d {
        &mut self.knots[0]
    }
}

pub fn day_9_star_1_and_2() {
    let mut lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day9.input");
    lines.pop();
    let lines = lines;

    let mut commands: Vec<Command> = Vec::with_capacity(lines.len());
    for line in lines {
        commands.push(Command::from_line(&line));
    }
    let commands = commands;

    let mut two_knot_line = Rope::new(2);
    let mut two_knot_tail_positions: HashMap<(i32, i32), bool> = HashMap::new();

    let mut ten_knot_line = Rope::new(10);
    let mut ten_knot_tail_positions: HashMap<(i32, i32), bool> = HashMap::new();

    for command in commands {
        for _ in 0..command.amount {
            two_knot_line.move_in_direction(&command.dir);
            let two_knot_tail = two_knot_line.tail();
            two_knot_tail_positions.insert((two_knot_tail.x, two_knot_tail.y), true);

            ten_knot_line.move_in_direction(&command.dir);
            let ten_knot_tail = ten_knot_line.tail();
            ten_knot_tail_positions.insert((ten_knot_tail.x, ten_knot_tail.y), true);
        }
    }

    println!(
        "Result of Advent of Code Day 9, Star 1: {}",
        two_knot_tail_positions.keys().len()
    );

    println!(
        "Result of Advent of Code Day 9, Star 2: {}",
        ten_knot_tail_positions.keys().len()
    );
}
