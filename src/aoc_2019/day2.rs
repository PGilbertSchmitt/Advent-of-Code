use crate::aoc_2019::intcode_cpu::IntCPU;

const SAMPLE_1: &str = "1,9,10,3,2,3,11,0,99,30,40,50";
const SAMPLE_2: &str = "1,0,0,0,99";
const SAMPLE_3: &str = "2,3,0,3,99";
const SAMPLE_4: &str = "2,4,4,5,99,0";
const SAMPLE_5: &str = "1,1,1,4,99,5,6,0,99";

const INPUT: &str = include_str!("./inputs/day2.txt");

fn find_noun_and_verb(cpu: &mut IntCPU, search_value: i64) -> i64 {
    for noun in 0..cpu.len() {
        for verb in 0..cpu.len() {
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
fn samples() {
    let mut cpu = IntCPU::new(SAMPLE_1);
    cpu.exec();
    assert_eq!(3500, cpu.get(0));

    let mut cpu = IntCPU::new(SAMPLE_2);
    cpu.exec();
    assert_eq!(2, cpu.get(0));

    let mut cpu = IntCPU::new(SAMPLE_3);
    cpu.exec();
    assert_eq!(2, cpu.get(0));

    let mut cpu = IntCPU::new(SAMPLE_4);
    cpu.exec();
    assert_eq!(2, cpu.get(0));

    let mut cpu = IntCPU::new(SAMPLE_5);
    cpu.exec();
    assert_eq!(30, cpu.get(0));
}

#[test]
fn solutions() {
    let mut cpu = IntCPU::new(INPUT);
    cpu.init(12, 2);
    cpu.exec();
    assert_eq!(4930687, cpu.get(0));
    assert_eq!(5335, find_noun_and_verb(&mut cpu, 19690720));
}
