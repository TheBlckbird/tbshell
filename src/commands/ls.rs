use std::{fs, path::Path};

pub fn command(directory: &Path) -> String {
    let paths = fs::read_dir(directory).unwrap();

    let mut output = "".to_owned();
    for path in paths {
        output.push_str(&format!(
            "{}\n",
            &path
                .unwrap()
                .path()
                .to_string_lossy()
                .strip_prefix(&format!("{}/", directory.to_str().unwrap()))
                .unwrap()
        ));
    }

    format!("{}\n", output.trim())
}
