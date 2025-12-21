use crate::aoc_2019::intcode_cpu::IntCPU;

const INPUT: &str = include_str!("./inputs/day5.txt");

#[test]
fn part_1() {
    let mut cpu = IntCPU::new(INPUT);

    cpu.push_input(1);
    cpu.exec();
    assert_eq!(Some(&13285749), cpu.output_queue.iter().last());

    cpu.reset();
    cpu.push_input(5);
    cpu.exec();
    assert_eq!(Some(&5000972), cpu.output_queue.iter().last());
}
