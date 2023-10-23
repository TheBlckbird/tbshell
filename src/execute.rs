// use std::process::Command;

use crate::commands::{cd, exit, ls, pwd};
use colored::*;
use std::path::PathBuf;

pub fn execute(line: &str, current_directory: &mut PathBuf) -> String {
    let mut output = "".to_owned();
    let line_trimmed = line.trim();

    match line_trimmed {
        "ls" | "l" => {
            output = ls::command(current_directory);
        }
        "pwd" => output = pwd::command(current_directory),
        "exit" | "e" => exit::command(),
        "cd" => {
            output = cd::command("/", current_directory);
        }
        _ if line_trimmed.starts_with("cd ") => {
            output = cd::command(
                line_trimmed.strip_prefix("cd").unwrap().trim(),
                current_directory,
            );
        }
        "" => {}
        _ => output = format!("Unknown command: {}\n", line.trim().red()),
    }

    output
}
