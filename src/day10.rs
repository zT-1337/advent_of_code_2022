use crate::util::load_lines_of_file;

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

struct Device {
    cycle_count: usize,
    register_x: i64,
    signal_strength: i64,
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Device {
    fn new() -> Self {
        Self {
            cycle_count: 0,
            register_x: 1,
            signal_strength: 0,
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    fn parse_and_execute_instruction(&mut self, instruction: &str) {
        let instruction_tokens: Vec<&str> = instruction.split_whitespace().collect();
        match instruction_tokens[0] {
            "addx" => self.add_to_x(instruction_tokens[1].parse().unwrap()),
            "noop" => self.noop(),
            _ => panic!("Unknown instruction token {}", instruction_tokens[0]),
        }
    }

    fn tick_cycle(&mut self) {
        self.render_pixel();

        self.cycle_count += 1;
        if self.cycle_count >= 20 && (self.cycle_count - 20) % 40 == 0 && self.cycle_count <= 220 {
            self.signal_strength += i64::try_from(self.cycle_count).unwrap() * self.register_x;
        }
    }

    fn render_pixel(&mut self) {
        if self.cycle_count >= SCREEN_WIDTH * SCREEN_HEIGHT {
            return;
        }

        let current_pixel_position = i64::try_from(self.cycle_count).unwrap();
        let current_row = self.cycle_count / SCREEN_WIDTH;
        let current_sprite_position =
            self.register_x + i64::try_from(current_row * SCREEN_WIDTH).unwrap();

        let is_current_pixel_lit = current_pixel_position >= current_sprite_position - 1
            && current_pixel_position <= current_sprite_position + 1;

        self.screen[self.cycle_count] = is_current_pixel_lit;
    }

    fn noop(&mut self) {
        self.tick_cycle();
    }

    fn add_to_x(&mut self, value_to_add: i64) {
        self.tick_cycle();
        self.tick_cycle();
        self.register_x += value_to_add;
    }

    fn draw_screen(&self) {
        for (index, pixel) in self.screen.iter().enumerate() {
            if index % SCREEN_WIDTH == 0 {
                println!("");
            }

            match pixel {
                true => print!("#"),
                false => print!("."),
            }
        }

        println!("");
    }
}

pub fn day_10_star_1() {
    let mut lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day10.input");
    lines.pop();

    let mut device = Device::new();
    for line in lines.iter() {
        device.parse_and_execute_instruction(line);
    }

    println!(
        "Result of Advent of Code Day 10, Star 1: {}",
        device.signal_strength
    );
    print!("Result of Advent of Code Day 10, Star 2:");
    device.draw_screen();
}
