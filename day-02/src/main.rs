//! Advent of Code 2019 - Day 02
//! https://adventofcode.com/2019/day/2

extern crate aoc_util as util;

type Num = usize;

fn main() {
    let input = util::get_input();

    let mut opcodes = input
        .split(",")
        .filter_map(|x| x.trim().parse::<Num>().ok())
        .collect::<Vec<Num>>();

    // noun = 39
    // verb = 52
    opcodes.get_mut(10).map(|x| *x = 39);
    opcodes.get_mut(11).map(|x| *x = 52);

    let program = Program::from(opcodes);
    println!("{}", program.run().unwrap());

    // Find inputs that will result in the value:
    //     19690720

    // const INPUT_ONE_POS: Num = 1;
    // const INPUT_TWO_POS: Num = 2;
    // const INPUT_MAX: usize = 99;
    // const TARGET_RESULT: usize = 19690720;
    // let mut result = 0;
    // let mut input_one = 0;
    // let mut input_two = 0;

    // while result != TARGET_RESULT {
    //     let mut opcodes = orig_opcodes.clone();

    //     opcodes.get_mut(INPUT_ONE_POS).map(|x| *x = input_one);
    //     opcodes.get_mut(INPUT_TWO_POS).map(|x| *x = input_two);

    //     let program = Program::from(opcodes);

    //     result = program.run().unwrap();

    //     // Increment inputs
    //     if input_two < INPUT_MAX {
    //         input_two += 1;
    //     } else {
    //         if input_one < INPUT_MAX {
    //             input_one += 1;
    //             input_two = 0;
    //         } else {
    //             panic!("No result found :(");
    //         }
    //     }
    // }

    // println!(
    //     "noun = {}\nverb = {}\nresult = {}",
    //     input_one, input_two, result
    // );
}

#[derive(Debug)]
struct Program {
    opcodes: Vec<Num>,
}

impl Program {
    pub fn run(mut self) -> Option<Num> {
        for i in 0 .. self.opcodes.len() {
            let opcode = self
                .opcodes
                .get(i)
                .expect(&format!("opcode at index {} should exist", i));

            if i % 4 == 0 {
                let op = match opcode {
                    1 | 2 => {
                        let arg_one = *self
                            .opcodes
                            .get(i + 1)
                            .expect("Opcode needs FIRST argument");
                        let arg_two = *self
                            .opcodes
                            .get(i + 2)
                            .expect("Opcode needs SECOND argument");
                        let arg_three = *self
                            .opcodes
                            .get(i + 3)
                            .expect("Opcode needs THIRD argument");

                        match opcode {
                            1 => Op::Add(arg_one, arg_two, arg_three),
                            2 => Op::Mult(arg_one, arg_two, arg_three),
                            _ => unreachable!(),
                        }
                    }
                    99 => Op::Halt,
                    _ => panic!("Invalid opcode '{}'", opcode),
                };

                match op {
                    Op::Add(in_one, in_two, out)
                    | Op::Mult(in_one, in_two, out) => {
                        let in_one = self.get_at(in_one);
                        let in_two = self.get_at(in_two);
                        let out = self.get_at_mut(out);

                        match op {
                            Op::Add(_, _, _) => {
                                *out = in_one + in_two;
                            }
                            Op::Mult(_, _, _) => {
                                *out = in_one * in_two;
                            }
                            _ => unreachable!(),
                        }
                    }
                    Op::Halt => break,
                }
            }
        }

        self.opcodes.get(0).map(|x| *x)
    }

    fn get_at(&self, pos: usize) -> Num {
        *self
            .opcodes
            .get(pos)
            .expect(&format!("Opcode at position {} doesn't exist", pos))
    }

    fn get_at_mut(&mut self, pos: usize) -> &mut Num {
        self.opcodes
            .get_mut(pos)
            .expect(&format!("Opcode at position {} doesn't exist", pos))
    }
}

impl From<Vec<Num>> for Program {
    fn from(opcodes: Vec<Num>) -> Self {
        Self { opcodes }
    }
}

#[derive(Debug)]
enum Op {
    Add(Num, Num, Num),
    Mult(Num, Num, Num),
    Halt,
}
