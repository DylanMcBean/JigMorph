use std::fs;
use std::path::Path;

pub fn handle_solutions_folder(solutions_folder: &str) {
    if !Path::new(solutions_folder).exists() {
        fs::create_dir_all(solutions_folder).unwrap();
    }
}
