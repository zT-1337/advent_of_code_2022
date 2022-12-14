use crate::util::load_lines_of_file;

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    amount: u8,
}

impl Command {
    fn from_line(line: &str) -> Self {
        let tokens: Vec<&str> = line.split_whitespace().collect();
            
        if tokens.len() != 2 {
            panic!("Expected 2 tokens but was: {} {:?}", tokens.len(), tokens);
        }

        let dir = match tokens[0] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Unexpected direction: {}", tokens[0]),
        };

        let amount = match tokens[1].parse::<u8>() {
            Ok(value) => value,
            Err(why) => panic!("{}", why),
        };

        Self { dir, amount }
    }
}

pub fn day_9_star_1() {
    let mut lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day9.input");
    lines.pop();
    let lines = lines;

    let mut commands: Vec<Command> = Vec::with_capacity(lines.len());
    for line in lines {
        commands.push(Command::from_line(&line));
    }

    println!("{:?}", commands);
}
