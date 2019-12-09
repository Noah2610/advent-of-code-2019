//! Advent of Code 2019 - shared utility functions for all puzzles

const INPUT_FILENAME: &str = "input.txt";

/// Returns the content of the input file for the current day's puzzle as a String.
/// The input file must be in the puzzle's crate root (next to the crate's `Cargo.toml` file)
/// or in the directory of the compiled binary, with the name `input.txt`.
/// If the program is run with `cargo run`, then it will check for the input file
/// in the crate's root directory. If the compiled binary is executed directly,
/// then it will check for the input file in the same directory as the executable.
pub fn get_input() -> String {
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    let mut path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir)
    } else {
        let executable = env::args().next().unwrap();
        PathBuf::from(executable)
            .parent()
            .map(Into::into)
            .unwrap_or(PathBuf::from("."))
    };

    path.push(INPUT_FILENAME);

    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
}
