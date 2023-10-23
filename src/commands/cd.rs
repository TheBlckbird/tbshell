use std::path::PathBuf;

use colored::Colorize;

pub fn command(new_directory: &str, current_directory: &mut PathBuf) -> String {
    let new_directory = new_directory.trim();
    let new_current_directory = current_directory.join(new_directory).canonicalize();

    *current_directory = match new_current_directory {
        Ok(path) => path,
        Err(_) => {
            return format!("Directory {} does not exist\n", new_directory.red());
        }
    };

    "".to_owned()
}
