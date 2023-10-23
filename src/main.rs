mod commands;
mod execute;

use colored::Colorize;
use execute::execute;
use std::io::{self, Write};

fn main() {
    let mut current_directory = std::env::current_dir().unwrap();

    loop {
        let mut input = String::new();

        print!(
            "{}{} ",
            current_directory.to_str().unwrap_or("").purple().bold(),
            ">".blue()
        );

        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from STDIN");

        print!("{}", execute(&input, &mut current_directory));
    }
}
