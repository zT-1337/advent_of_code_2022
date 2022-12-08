use crate::util::load_lines_of_file;

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

impl Move {
    fn from(line: &str) -> Self {
        let values = read_in_three_numbers_in_line(&line);
        Self {
            amount: values[0],
            from: values[1],
            to: values[2],
        }
    }
}

fn read_in_three_numbers_in_line(line: &str) -> [usize; 3] {
    let mut values: [usize; 3] = [0, 0, 0];

    //Find the beginning of each number and read until non digit appears
    //Then parse the number from found start until current i
    //Last number gets parsed after the loop because the line ends with a digit
    let mut start: usize = 0;
    let mut matching = false;
    let mut next_value_index: usize = 0;
    for (i, c) in line.chars().enumerate() {
        if !matching && !c.is_ascii_digit() {
            continue;
        }

        if !matching && c.is_ascii_digit() {
            matching = true;
            start = i;
            continue;
        }

        if matching && !c.is_ascii_digit() {
            matching = false;
            values[next_value_index] = match line[start..i].parse::<usize>() {
                Ok(value) => value,
                Err(why) => panic!("could not parse value: {}", why),
            };
            next_value_index += 1;
            continue;
        }
    }

    values[next_value_index] = match line[start..].parse::<usize>() {
        Ok(value) => value,
        Err(why) => panic!("could not parse value: {}", why),
    };

    values
}

pub fn day_5_star_1() {
    let mut lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day5.input");
    //Remove empty line at the end
    lines.pop();

    let moves = read_in_moves(&mut lines);
    let stack_count = read_in_stack_count(&mut lines);

    let mut stacks: Vec<Vec<String>> = init_stacks(stack_count, &mut lines);

    //Execute moves
    for stack_move in moves {
        for _i in 0..stack_move.amount {
            let to_move = stacks[stack_move.from - 1].pop().unwrap();
            stacks[stack_move.to - 1].push(to_move);
        }
    }

    println!(
        "Result of Advent of Code Day 5, Star 1: {}",
        build_result(&stacks)
    );
}

pub fn day_5_star_2() {
    let mut lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day5.input");
    //Remove empty line at the end
    lines.pop();

    let moves = read_in_moves(&mut lines);
    let stack_count = read_in_stack_count(&mut lines);

    let mut stacks: Vec<Vec<String>> = init_stacks(stack_count, &mut lines);

    //Execute moves
    for stack_move in moves {
        let mut moved_elements = Vec::with_capacity(stack_move.amount);
        for _i in 0..stack_move.amount {
            moved_elements.push(stacks[stack_move.from - 1].pop().unwrap());
        }

        while !moved_elements.is_empty() {
            stacks[stack_move.to - 1].push(moved_elements.pop().unwrap());
        }
    }

    println!(
        "Result of Advent of Code Day 5, Star 2: {}",
        build_result(&stacks)
    );
}

fn read_in_moves(lines: &mut Vec<String>) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    loop {
        let line = &lines.pop().unwrap();
        if line == "" {
            break;
        }

        moves.push(Move::from(line));
    }

    moves.reverse();
    moves
}

fn read_in_stack_count(lines: &mut Vec<String>) -> usize {
    lines
        .pop()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn init_stacks(count: usize, lines: &mut Vec<String>) -> Vec<Vec<String>> {
    let mut stacks = vec![];
    for _i in 0..count {
        stacks.push(Vec::new());
    }

    while !lines.is_empty() {
        let line = lines.pop().unwrap();
        read_stack_line(&mut stacks, line, count);
    }

    stacks
}

fn read_stack_line(stacks: &mut Vec<Vec<String>>, line: String, stack_count: usize) {
    let mut stack_index = 0;
    let mut letter_index = 1;
    while stack_index < stack_count {
        let letter = &line[letter_index..letter_index + 1];
        if letter != " " {
            stacks[stack_index].push(String::from(letter));
        }

        stack_index += 1;
        letter_index += 4;
    }
}

fn build_result(stacks: &Vec<Vec<String>>) -> String {
    let mut result = String::from("");
    for stack in stacks {
        result.push_str(stack.last().unwrap());
    }

    result
}
