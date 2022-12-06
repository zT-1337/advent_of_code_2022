use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn load_lines_of_file(path: &str) -> BufReader<File> {
    let path = Path::new(path);
    let file = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("Failed to open file {}: {}", path.display(), why)
    };

    file
}

