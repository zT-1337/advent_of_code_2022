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

pub struct Vec2d {
    pub x: usize,
    pub y: usize,
}

impl Vec2d {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}
