# Advent of Code 2019
My solutions for [Advent of Code 2019][aoc2019].

---

## Running a day's puzzle
Run a specific day's puzzle using the `bin/run` script.  
The first argument must be the day (ex.: `day-01`).  
Any further arguments are passed to the underlying `cargo run` command.  
Example:
```
bin/run day-01
```

## Generate a new crate
To generate a new crate for a new day's puzzle, use the `bin/new-day` script.  
The first argument must be the new day (ex.: `day-01`).  
Any further arguments are passed to the underlying `cargo new` command.  
The script will also manipulate the generated `Cargo.toml` file to set the version  
to `0.0.0` and add the `aoc-util` crate (from `./util/`) as a dependency.  
The directory and crate name will be derived from the given day name.  
Example:
```
bin/new-day day-99
```
This will generated a crate with the name `aoc2019-day-99` in a new `day-99/` directory.  

---

## License
[MIT License][license]

[aoc2019]: https://adventofcode.com/2019
[license]: https://github.com/Noah2610/advent-of-code-2019/blob/master/LICENSE
