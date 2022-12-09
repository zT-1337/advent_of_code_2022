use std::{collections::HashMap, mem::needs_drop};

use crate::util::load_lines_of_file;

struct FileSystem {
    total_space: usize,
    root: Directory,
    path_indices: Vec<usize>,
}

impl FileSystem {
    fn new() -> Self {
        let root = Directory::new("/");
        Self {
            total_space: 70_000_000,
            root,
            path_indices: vec![],
        }
    }

    fn from_logs(logs: &Vec<String>) -> Self {
        let mut result = FileSystem::new();

        let mut line_index: usize = 0;
        while line_index < logs.len() {
            let line = &logs[line_index];
            if line.len() == 0 {
                line_index += 1;
                continue;
            }

            let tokens: Vec<&str> = line.split_whitespace().collect();
            match tokens[1] {
                "cd" => result.execute_cd_command(tokens[2]),
                "ls" => line_index += result.add_files_from_ls_logs(&logs[line_index + 1..]),
                _ => panic!("Unexpected token: {}", tokens[1]),
            }

            line_index += 1;
        }

        result.change_to_root();
        result
    }

    fn execute_cd_command(&mut self, path: &str) {
        match path {
            ".." => self.change_to_prev_dir(),
            "/" => self.change_to_root(),
            dir_name => self.change_dir(dir_name),
        }
    }

    fn add_files_from_ls_logs(&mut self, logs: &[String]) -> usize {
        let mut ls_output_offset: usize = 0;
        let mut ls_line_tokens: Vec<&str> = logs[ls_output_offset].split_whitespace().collect();

        while ls_line_tokens.len() > 0 && ls_line_tokens[0] != "$" {
            match ls_line_tokens[0] {
                "dir" => self.add_sub_dir(Directory::new(ls_line_tokens[1])),
                _ => self.add_file(File::new(
                    ls_line_tokens[0].parse().unwrap(),
                    ls_line_tokens[1],
                )),
            }
            ls_output_offset += 1;
            ls_line_tokens = logs[ls_output_offset].split_whitespace().collect();
        }

        ls_output_offset
    }

    fn current_dir_mut(&mut self) -> &mut Directory {
        let mut current_dir = &mut self.root;
        for i in self.path_indices.iter() {
            current_dir = &mut current_dir.sub_dirs[*i];
        }

        current_dir
    }

    fn current_dir(&self) -> &Directory {
        let mut current_dir = &self.root;
        for i in self.path_indices.iter() {
            current_dir = &current_dir.sub_dirs[*i];
        }

        current_dir
    }

    fn change_dir(&mut self, name: &str) {
        let index = match self.current_dir().sub_dirs_pos.get(name) {
            Some(position) => position,
            None => return,
        };

        self.path_indices.push(*index);
    }

    fn change_to_prev_dir(&mut self) {
        if self.path_indices.len() > 0 {
            self.path_indices.pop();
        }
    }

    fn change_to_root(&mut self) {
        self.path_indices.clear();
    }

    fn add_sub_dir(&mut self, dir: Directory) {
        let current_dir = self.current_dir_mut();

        if current_dir.is_file_or_sub_dir_existing_with_name(&dir.name) {
            return;
        }

        current_dir
            .sub_dirs_pos
            .insert(dir.name.clone(), current_dir.sub_dirs.len());
        current_dir.sub_dirs.push(dir);
    }

    fn add_file(&mut self, file: File) {
        let current_dir = self.current_dir_mut();

        if current_dir.is_file_or_sub_dir_existing_with_name(&file.name) {
            return;
        }

        current_dir
            .files_pos
            .insert(file.name.clone(), current_dir.files.len());
        current_dir.files.push(file);
    }

    fn recursive_list(&self) -> String {
        let mut result = String::from("");
        let current_dir = self.current_dir();
        current_dir.recursive_list(0, &mut result);

        result
    }

    fn sizes_of_small_dirs(&self, max_small_dir_size: usize) -> usize {
        let mut sum = 0;
        FileSystem::add_small_dir_size(&self.root, &mut sum, &max_small_dir_size);
        sum
    }

    fn add_small_dir_size(dir: &Directory, sum: &mut usize, max_small_dir_size: &usize) {
        let dir_size = dir.total_size();
        if dir_size <= *max_small_dir_size {
            *sum += dir_size;
        }

        for sub_dir in dir.sub_dirs.iter() {
            FileSystem::add_small_dir_size(sub_dir, sum, max_small_dir_size);
        }
    }

    fn free_up_space(&self, needed_space: usize) -> Option<usize> {
        let mut big_enough_sizes: Vec<usize> = Vec::new();
        let needed_space = needed_space - (self.total_space - self.root.total_size());
        FileSystem::add_big_enough_dir_size(&self.root, &mut big_enough_sizes, needed_space);

        big_enough_sizes.iter().min().copied()
    }

    fn add_big_enough_dir_size(
        dir: &Directory,
        big_enough_sizes: &mut Vec<usize>,
        needed_space: usize,
    ) {
        let dir_size = dir.total_size();
        if dir_size >= needed_space {
            big_enough_sizes.push(dir_size);
        }

        for sub_dir in dir.sub_dirs.iter() {
            FileSystem::add_big_enough_dir_size(sub_dir, big_enough_sizes, needed_space);
        }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    files_pos: HashMap<String, usize>,
    sub_dirs: Vec<Directory>,
    sub_dirs_pos: HashMap<String, usize>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            files: Vec::new(),
            files_pos: HashMap::new(),
            sub_dirs: Vec::new(),
            sub_dirs_pos: HashMap::new(),
        }
    }

    fn total_size(&self) -> usize {
        let mut result = 0;

        for file in self.files.iter() {
            result += file.size;
        }

        for sub_dir in self.sub_dirs.iter() {
            result += sub_dir.total_size();
        }

        result
    }

    fn is_file_or_sub_dir_existing_with_name(&self, name: &str) -> bool {
        self.files_pos.contains_key(name) || self.sub_dirs_pos.contains_key(name)
    }

    fn recursive_list(&self, indent_level: usize, result: &mut String) {
        let indent_self = "  ".repeat(indent_level);
        result.push_str(&format!("{}- {} (dir)\n", indent_self, self.name));

        let indent_sub = "  ".repeat(indent_level + 1);
        for dir in self.sub_dirs.iter() {
            dir.recursive_list(indent_level + 1, result);
        }

        for file in self.files.iter() {
            result.push_str(&format!(
                "{}- {} (file, size={})\n",
                indent_sub, file.name, file.size
            ));
        }
    }
}

#[derive(Debug)]
struct File {
    size: usize,
    name: String,
}

impl File {
    fn new(size: usize, name: &str) -> Self {
        Self {
            size,
            name: String::from(name),
        }
    }
}

pub fn day_7_star_1() {
    let logs = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day7.input");
    let file_system = FileSystem::from_logs(&logs);
    println!(
        "Result of Advent of Code Day 7, Star 1: {}",
        file_system.sizes_of_small_dirs(100_000)
    );
    println!(
        "Result of Advent of Code Day 7, Star 2: {}",
        file_system.free_up_space(30_000_000).unwrap()
    );
}
