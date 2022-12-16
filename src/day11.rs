use crate::util::load_lines_of_file;
use std::{cmp::Ordering, collections::VecDeque};

#[derive(Debug, Clone)]
enum Operator {
    Addition,
    Multiplication,
}

impl Operator {
    fn from_token(token: &str) -> Self {
        match token {
            "+" => Operator::Addition,
            "*" => Operator::Multiplication,
            _ => panic!("Unexpected operator: {}", token),
        }
    }
}

#[derive(Debug, Clone)]
enum OperatorValue {
    Literal(u64),
    Itself,
}

impl OperatorValue {
    fn from_token(token: &str) -> Self {
        match token {
            "old" => OperatorValue::Itself,
            _ => OperatorValue::Literal(token.parse().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    right_hand_side: OperatorValue,
}

impl Operation {
    fn from_line(line: &str) -> Self {
        let operation_tokens: Vec<&str> = line[23..].split_whitespace().collect();
        Self {
            operator: Operator::from_token(operation_tokens[0]),
            right_hand_side: OperatorValue::from_token(operation_tokens[1]),
        }
    }

    fn execute(&self, left_hand_side: &mut u64) {
        match (&self.operator, &self.right_hand_side) {
            (Operator::Addition, OperatorValue::Itself) => *left_hand_side += *left_hand_side,
            (Operator::Multiplication, OperatorValue::Itself) => *left_hand_side *= *left_hand_side,
            (Operator::Addition, OperatorValue::Literal(value)) => *left_hand_side += *value,
            (Operator::Multiplication, OperatorValue::Literal(value)) => *left_hand_side *= *value,
        }
    }
}

struct Throw {
    item: u64,
    receiver: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_divisor: u64,
    true_receiving_monkey: usize,
    false_receiving_monkey: usize,
    inspected_item_count: usize,
}

impl Monkey {
    fn from_lines(lines: &mut Vec<String>) -> Self {
        let _monkey_line = lines.pop().unwrap();
        let starting_items_line = lines.pop().unwrap();
        let operation_line = lines.pop().unwrap();
        let test_divisor_line = lines.pop().unwrap();
        let true_receiving_monkey_line = lines.pop().unwrap();
        let false_receiving_monkey_line = lines.pop().unwrap();
        let _empty_line = lines.pop().unwrap();

        let test_divisor: u64 = test_divisor_line[21..].parse().unwrap();
        let true_receiving_monkey: usize = true_receiving_monkey_line[29..].parse().unwrap();
        let false_receiving_monkey: usize = false_receiving_monkey_line[30..].parse().unwrap();

        Self {
            items: Monkey::items_from_line(&starting_items_line),
            operation: Operation::from_line(&operation_line),
            test_divisor,
            true_receiving_monkey,
            false_receiving_monkey,
            inspected_item_count: 0,
        }
    }

    fn items_from_line(line: &str) -> VecDeque<u64> {
        let mut items: VecDeque<u64> = VecDeque::new();
        for token in line[18..].split(", ") {
            match token.parse() {
                Ok(item) => items.push_back(item),
                Err(_) => continue,
            }
        }

        items
    }

    //Look at chinese remainder theorem
    fn turn_with_relief(&mut self, relief_factor: u64, magic_divider: u64) -> Vec<Throw> {
        let mut result = Vec::with_capacity(self.items.len());
        while self.items.len() > 0 {
            let mut current_item = self.items.pop_front().unwrap();
            self.operation.execute(&mut current_item);
            current_item %= magic_divider;

            if relief_factor > 0 {
                current_item /= relief_factor;
            }

            let throwing_to_monkey = match current_item % self.test_divisor {
                0 => self.true_receiving_monkey,
                _ => self.false_receiving_monkey,
            };

            result.push(Throw {
                item: current_item,
                receiver: throwing_to_monkey,
            });
            self.inspected_item_count += 1;
        }

        result
    }

    fn compare_monkey(left: &Monkey, right: &Monkey) -> Ordering {
        right.inspected_item_count.cmp(&left.inspected_item_count)
    }
}

pub fn day_11_star_1_and_2() {
    let mut lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day11.input");
    lines.reverse();

    let mut monkeys_star_1: Vec<Monkey> = Vec::new();
    while lines.len() > 0 {
        monkeys_star_1.push(Monkey::from_lines(&mut lines));
    }
    let mut monkeys_star_2 = monkeys_star_1.clone();

    play_out_turns(&mut monkeys_star_1, 20, 3);
    play_out_turns(&mut monkeys_star_2, 10000, 0);

    monkeys_star_1.sort_by(Monkey::compare_monkey);
    monkeys_star_2.sort_by(Monkey::compare_monkey);

    println!(
        "Result of Advent of Code Day 11, Star 1: {}",
        monkeys_star_1[0].inspected_item_count * monkeys_star_1[1].inspected_item_count
    );

    println!(
        "Result of Advent of Code Day 11, Star 2: {}",
        monkeys_star_2[0].inspected_item_count * monkeys_star_2[1].inspected_item_count
    );
}

fn play_out_turns(monkeys: &mut Vec<Monkey>, round_count: usize, relief_factor: u64) {
    let magic_divider: u64 = monkeys.iter().map(|monkey| monkey.test_divisor).product();

    for _ in 0..round_count {
        play_out_single_turn(monkeys, relief_factor, magic_divider);
    }
}

fn play_out_single_turn(monkeys: &mut Vec<Monkey>, relief_factor: u64, magic_divider: u64) {
    for i in 0..monkeys.len() {
        let throws = monkeys[i].turn_with_relief(relief_factor, magic_divider);
        for throw in throws {
            monkeys[throw.receiver].items.push_back(throw.item);
        }
    }
}
