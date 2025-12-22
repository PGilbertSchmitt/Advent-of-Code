use crate::aoc_2019::intcode_cpu::{IntCPU, Program};

const INPUT: &str = include_str!("./inputs/day2.txt");

fn find_noun_and_verb(program: &Program, search_value: i128) -> i128 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut cpu = IntCPU::new(program);
            cpu.init(noun, verb);
            cpu.exec();
            if search_value == cpu.get(0) {
                return noun * 100 + verb;
            }
        }
    }
    -1
}

#[test]
fn solutions() {
    let program = IntCPU::parse_program(INPUT);
    let mut cpu = IntCPU::new(&program);
    cpu.init(12, 2);
    cpu.exec();
    assert_eq!(4930687, cpu.get(0));
    assert_eq!(5335, find_noun_and_verb(&program, 19690720));
}
