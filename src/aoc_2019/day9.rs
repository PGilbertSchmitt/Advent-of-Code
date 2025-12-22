use crate::aoc_2019::intcode_cpu::IntCPU;

const INPUT: &str = include_str!("./inputs/day9.txt");

#[test]
fn solutions() {
    let program = IntCPU::parse_program(INPUT);

    let mut cpu = IntCPU::new(&program);
    cpu.push_input(1);
    cpu.exec();
    assert_eq!(Some(2_427_443_564), cpu.last_output());

    let mut cpu = IntCPU::new(&program);
    cpu.push_input(2);
    cpu.exec();
    assert_eq!(Some(87221), cpu.last_output());
}
