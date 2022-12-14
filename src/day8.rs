use std::collections::HashMap;

use crate::util::{load_lines_of_file, Vec2d};

pub fn day_8_star_2() {
    let mut lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day8.input");
    lines.pop();
    let lines = lines;

    let width = lines[0].len();
    let height = lines.len();
    let trees = init_trees_from_lines(&lines, width * height);

    let mut current_pos = Vec2d::new(0, 0);
    let dimensions = Vec2d::new(width, height);

    let mut max_scenic_score = 0;
    for col in 1..(width - 1) {
        for row in 1..(height - 1) {
            current_pos.set(col, row);
            let current_scenic_score = calc_scenic_score(&trees, &current_pos, &dimensions);

            if current_scenic_score > max_scenic_score {
                max_scenic_score = current_scenic_score;
            }
        }
    }

    println!(
        "Result of Advent of Code Day 8, Star 2: {}",
        max_scenic_score
    );
}

fn calc_scenic_score(trees: &Vec<u8>, position: &Vec2d, dimensions: &Vec2d) -> u64 {
    let mut score_top = 0;
    let mut score_bottom = 0;
    let mut score_left = 0;
    let mut score_right = 0;

    let current_tree = trees[position.x + position.y * dimensions.x];

    //scenic score top
    for row in (0..=position.y - 1).rev() {
        score_top += 1;
        if trees[position.x + row * dimensions.x] >= current_tree {
            break;
        }
    }

    //scenic score bottom
    for row in position.y + 1..dimensions.y {
        score_bottom += 1;
        if trees[position.x + row * dimensions.x] >= current_tree {
            break;
        }
    }

    //scenic score left
    for col in (0..=position.x - 1).rev() {
        score_left += 1;
        if trees[col + position.y * dimensions.x] >= current_tree {
            break;
        }
    }

    //scenic score right
    for col in position.x + 1..dimensions.x {
        score_right += 1;
        if trees[col + position.y * dimensions.x] >= current_tree {
            break;
        }
    }

    score_top * score_bottom * score_left * score_right
}

pub fn day_8_star_1() {
    let mut lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day8.input");
    lines.pop();
    let lines = lines;

    let width = lines[0].len();
    let height = lines.len();
    let trees = init_trees_from_lines(&lines, width * height);
    let mut visible_inner_trees: HashMap<(usize, usize), bool> =
        HashMap::with_capacity(width * height);

    //find any trees visible from top
    for col in 1..(width - 1) {
        let mut highest_tree = trees[col];
        for row in 1..(height - 1) {
            let current_tree = trees[col + row * width];
            if current_tree > highest_tree {
                highest_tree = current_tree;
                visible_inner_trees.insert((col, row), true);
            }
        }
    }

    //find any trees visible from bottom
    for col in 1..(width - 1) {
        let mut highest_tree = trees[col + (height - 1) * width];
        for row in (1..(height - 1)).rev() {
            let current_tree = trees[col + row * width];
            if current_tree > highest_tree {
                highest_tree = current_tree;
                visible_inner_trees.insert((col, row), true);
            }
        }
    }

    //find any trees visible from left
    for row in 1..(height - 1) {
        let mut highest_tree = trees[row * width];
        for col in 1..(width - 1) {
            let current_tree = trees[col + row * width];
            if current_tree > highest_tree {
                highest_tree = current_tree;
                visible_inner_trees.insert((col, row), true);
            }
        }
    }

    //find any trees visible from right
    for row in 1..(height - 1) {
        let mut highest_tree = trees[row * width + width - 1];
        for col in (1..(width - 1)).rev() {
            let current_tree = trees[col + row * width];
            if current_tree > highest_tree {
                highest_tree = current_tree;
                visible_inner_trees.insert((col, row), true);
            }
        }
    }

    let visible_outer_trees = 2 * width + 2 * height - 4;
    println!(
        "Result of Advent of Code Day 8, Star 1: {}",
        visible_inner_trees.keys().len() + visible_outer_trees
    );
}

fn init_trees_from_lines(lines: &Vec<String>, capacity: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(capacity);

    for line in lines.iter() {
        for i in 0..line.len() {
            result.push(line[i..i + 1].parse().unwrap());
        }
    }

    result
}
