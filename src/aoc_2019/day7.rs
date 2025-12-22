use itertools::Itertools;

use crate::aoc_2019::intcode_cpu::{IntCPU, Program};

const SAMPLE_1: &str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
const SAMPLE_2: &str = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
const SAMPLE_3: &str = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
const SAMPLE_4: &str =
    "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
const SAMPLE_5: &str = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
const INPUT: &str = include_str!("./inputs/day7.txt");

fn test_amplifier(program: &Program, config: Vec<i128>) -> i128 {
    let mut last_output = 0;
    for x in config {
        let mut cpu = IntCPU::new(program);
        cpu.push_input(x);
        cpu.push_input(last_output);
        cpu.next();
        last_output = cpu.last_output().unwrap();
    }
    last_output
}

fn test_amplifier_with_feedback(program: &Program, config: &Vec<i128>) -> i128 {
    let mut amplifiers: Vec<IntCPU> = config
        .iter()
        .map(|&x| {
            let mut cpu = IntCPU::new(program);
            cpu.push_input(x);
            cpu
        })
        .collect();

    let mut final_output = 0;
    'outer: loop {
        let mut last_output = final_output;
        for amplifier in amplifiers.iter_mut() {
            if amplifier.unfinished() {
                amplifier.push_input(last_output);
                last_output = amplifier.next().unwrap();
            } else {
                break 'outer;
            }
        }
        // This use of 2 trackers is incase the amplifiers don't finish at the same time
        final_output = last_output;
    }

    final_output
}

fn find_largest_amplification(program: &Program) -> i128 {
    let mut largest_output = 0;
    for config in (0..5).permutations(5) {
        largest_output = largest_output.max(test_amplifier(program, config));
    }
    largest_output
}

fn find_largest_amplification_with_feedback(program: &Program) -> i128 {
    let mut largest_output = 0;
    for config in (5..10).permutations(5) {
        largest_output = largest_output.max(test_amplifier_with_feedback(program, &config));
    }
    largest_output
}

#[test]
fn samples() {
    assert_eq!(
        43210,
        find_largest_amplification(&mut IntCPU::parse_program(SAMPLE_1))
    );
    assert_eq!(
        54321,
        find_largest_amplification(&mut IntCPU::parse_program(SAMPLE_2))
    );
    assert_eq!(
        65210,
        find_largest_amplification(&mut IntCPU::parse_program(SAMPLE_3))
    );

    assert_eq!(
        139629729,
        find_largest_amplification_with_feedback(&mut IntCPU::parse_program(SAMPLE_4))
    );
    assert_eq!(
        18216,
        find_largest_amplification_with_feedback(&mut IntCPU::parse_program(SAMPLE_5))
    );
}

#[test]
fn solutions() {
    let program = IntCPU::parse_program(INPUT);
    assert_eq!(46248, find_largest_amplification(&program));
    assert_eq!(54163586, find_largest_amplification_with_feedback(&program));
}
