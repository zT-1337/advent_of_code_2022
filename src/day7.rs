use std::collections::HashMap;

use crate::util::load_lines_of_file;

struct FileSystem {
    root: Directory,
    path_indices: Vec<usize>,
}

impl FileSystem {

    fn new() -> Self {
        let root = Directory::new("/");
        Self {
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
                "ls" => line_index += result.add_files_from_ls_logs(&logs[line_index+1..]),
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
                "dir" => self.add_children(Directory::new(ls_line_tokens[1])),
                _ => self.add_file(File::new(ls_line_tokens[0].parse().unwrap(), ls_line_tokens[1])),
            }
            ls_output_offset += 1;
            ls_line_tokens = logs[ls_output_offset].split_whitespace().collect();
        }

        ls_output_offset
    }

    fn current_dir_mut(&mut self) -> &mut Directory {
        let mut current_dir = &mut self.root;
        for i in self.path_indices.iter() {
            current_dir = &mut current_dir.children[*i];
        }

        current_dir
    }

    fn current_dir(&self) -> &Directory {
        let mut current_dir = &self.root;
        for i in self.path_indices.iter() {
            current_dir = &current_dir.children[*i];
        }

        current_dir
    }

    fn change_dir(&mut self, name: &str) {
        let index = match self.current_dir().children_pos.get(name) {
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

    fn add_children(&mut self, dir: Directory) {
        let current_dir = self.current_dir_mut();

        if current_dir.is_file_or_child_existing_with_name(&dir.name) {
            return;
        }

        current_dir.children_pos.insert(dir.name.clone(), current_dir.children.len());
        current_dir.children.push(dir);
    }

    fn add_file(&mut self, file: File) {
        let current_dir = self.current_dir_mut();

        if current_dir.is_file_or_child_existing_with_name(&file.name) {
            return;
        }

        current_dir.files_pos.insert(file.name.clone(), current_dir.files.len());
        current_dir.files.push(file);
    }

    fn recursive_list(&self) -> String {
        let mut result = String::from("");
        let current_dir = self.current_dir();
        current_dir._recursive_list(0, &mut result);

        result
    }
}

struct Directory {
    name: String,
    files: Vec<File>,
    files_pos: HashMap<String, usize>,
    children: Vec<Directory>,
    children_pos: HashMap<String, usize>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            files: Vec::new(),
            files_pos: HashMap::new(),
            children: Vec::new(),
            children_pos: HashMap::new(),
        }
    }

    fn is_file_or_child_existing_with_name(&self, name: &str) -> bool {
        self.files_pos.contains_key(name) || self.children_pos.contains_key(name)
    }

    fn _recursive_list(&self, indent_level: usize, result: &mut String) {
        let indent_self = "  ".repeat(indent_level);
        result.push_str(&format!("{}- {} (dir)\n", indent_self, self.name));

        let indent_sub = "  ".repeat(indent_level+1);
        for dir in self.children.iter() {
            dir._recursive_list(indent_level + 1, result);
        }

        for file in self.files.iter() {
            result.push_str(&format!("{}- {} (file, size={})\n", indent_sub, file.name, file.size));
        }
    }
}

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
    println!("{}", file_system.recursive_list());
}
