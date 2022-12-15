use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn load_lines_of_file(path: &str) -> Vec<String> {
    let path = Path::new(path);
    let file = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("Failed to open file {}: {}", path.display(), why),
    };
    let mut lines = vec![];

    for line in file.lines() {
        match line {
            Ok(value) => lines.push(value),
            Err(why) => panic!("Failed to read line: {}", why),
        }
    }

    lines
}

#[derive(Clone)]
pub struct Vec2d {
    pub x: i32,
    pub y: i32,
}

impl Vec2d {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn subtract(left: &Vec2d, right: &Vec2d) -> Vec2d {
        Vec2d {
            x: left.x - right.x,
            y: left.y - right.y,
        }
    }

    pub fn add_to_self(&mut self, other: &Vec2d) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn is_touching(&self, other: &Vec2d) -> bool {
        self.x.abs_diff(other.x) < 2 && self.y.abs_diff(other.y) < 2
    }

    pub fn clamp(&mut self, min: i32, max: i32) {
        self.x = i32::clamp(self.x, min, max);
        self.y = i32::clamp(self.y, min, max);
    }
}

pub const UP_DIRECTION: Vec2d = Vec2d { x: 0, y: 1 };
pub const DOWN_DIRECTION: Vec2d = Vec2d { x: 0, y: -1 };
pub const LEFT_DIRECTION: Vec2d = Vec2d { x: -1, y: 0 };
pub const RIGHT_DIRECTION: Vec2d = Vec2d { x: 1, y: 0 };
