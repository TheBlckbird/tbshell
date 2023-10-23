use std::path::Path;

pub fn command(current_directory: &Path) -> String {
    format!("{}\n", current_directory.to_str().unwrap().to_owned())
}
