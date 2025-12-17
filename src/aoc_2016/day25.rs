use crate::aoc_2016::day12::{Instruction, Operand, Registers, parse_instructions};

const INPUT: &str = include_str!("./inputs/day25.txt");

fn is_clock_signal(ins: &Vec<Instruction>, a: isize, max_attempts: usize) -> bool {
    let mut ptr = 0;
    let mut state: Registers = [a, 0, 0, 0];
    let len = ins.len();

    // first "out" needs to be 0
    let mut last_signal = 1;
    let mut attempts = 0;

    while ptr < len && attempts < max_attempts {
        let cur_ins = &ins[ptr];

        match cur_ins {
            Instruction::Cpy(x, y) => {
                if let Operand::Register(y) = y {
                    let src = x.to_value(&state);
                    state[*y] = src;
                }
                ptr += 1;
            }
            Instruction::Inc(x) => {
                if let Operand::Register(x) = x {
                    state[*x] += 1;
                }
                ptr += 1;
            }
            Instruction::Dec(x) => {
                if let Operand::Register(x) = x {
                    state[*x] -= 1;
                }
                ptr += 1;
            }
            Instruction::Jnz(x, y) => {
                if x.to_value(&state) != 0 {
                    let delta = y.to_value(&state);
                    if delta.is_negative() {
                        let delta_pos = delta.abs() as usize;
                        if delta_pos > ptr {
                            panic!("Can't make ptr negative");
                        }
                        ptr -= delta_pos;
                    } else {
                        ptr += delta as usize;
                    }
                } else {
                    ptr += 1;
                }
            }
            Instruction::Out(x) => {
                let x = x.to_value(&state);
                // print!("{x} ");
                if last_signal == 0 && x == 1 {
                    last_signal = 1;
                } else if last_signal == 1 && x == 0 {
                    last_signal = 0;
                } else {
                    return false;
                };
                attempts += 1;
                ptr += 1;
            }
            // The program with Out doesn't have Tgl and vice versa
            _ => unreachable!(),
        }
    }

    // state[0]
    true
}

fn search_for_signal(instructions: &Vec<Instruction>) -> isize {
    for x in 0..10000 {
        // println!("Attempting {x}:");
        if is_clock_signal(instructions, x, 10) {
            return x;
        }
        // println!("");
    }
    return -1;
}

#[test]
fn solution() {
    assert_eq!(192, search_for_signal(&parse_instructions(INPUT)));
}
