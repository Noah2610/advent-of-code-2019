//! Advent of Code 2019 - shared utility functions for all puzzles

const INPUT_FILENAME: &str = "input.txt";

pub fn get_input() -> String {
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    // let dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    //     let executable = env::args().next().unwrap();
    //     PathBuf::from(executable);
    // };

    let mut file = File::open(INPUT_FILENAME).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
}
