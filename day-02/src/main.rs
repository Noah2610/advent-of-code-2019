//! Advent of Code 2019 - Day 02
//! https://adventofcode.com/2019/day/2

extern crate aoc_util as util;

type Num = u32;

fn main() {
    let input = util::get_input();

    let mut program = Program::from(
        input
            .split(",")
            .filter_map(|x| x.parse::<Num>().ok())
            .collect::<Vec<Num>>(),
    );

    program.run();
}

#[derive(Debug)]
struct Program {
    code: Vec<Op>,
}

impl Program {
    pub fn run(&mut self) {

    }
}

impl From<Vec<Num>> for Program {
    fn from(ops: Vec<Num>) -> Self {
        let mut code = Vec::new();

        for (i, num) in ops.iter().enumerate() {
            if i % 4 == 0 {
                match num {
                    1 => code.push(Op::Add(
                        *ops.get(i + 1).expect("Op::Add needs FIRST argument"),
                        *ops.get(i + 2).expect("Op::Add needs SECOND argument"),
                        *ops.get(i + 3).expect("Op::Add needs THIRD argument"),
                    )),
                    2 => code.push(Op::Mult(
                        *ops.get(i + 1).expect("Op::Mult needs FIRST argument"),
                        *ops.get(i + 2)
                            .expect("Op::Mult needs SECOND argument"),
                        *ops.get(i + 3).expect("Op::Mult needs THIRD argument"),
                    )),
                    99 => {
                        code.push(Op::Halt);
                        break;
                    }
                    _ => panic!("Invalid opcode '{}'", num),
                }
            }
        }

        Self { code }
    }
}

#[derive(Debug)]
enum Op {
    Add(Num, Num, Num),
    Mult(Num, Num, Num),
    Halt,
}
