use std::collections::VecDeque;

use fxhash::FxHashMap;

pub type Program = Vec<i128>;

pub fn parse_program(input: &str) -> Program {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

#[derive(Debug)]
pub struct IntCPU {
    program: Program,
    heap: FxHashMap<i128, i128>,
    ptr: i128,
    input_queue: VecDeque<i128>,
    output_queue: VecDeque<i128>,
    halted: bool,
    relative_base: i128,
}

const ADD: i128 = 1;
const MULT: i128 = 2;
const INPUT: i128 = 3;
const OUTPUT: i128 = 4;
const JUMP_IF_TRUE: i128 = 5;
const JUMP_IF_FALSE: i128 = 6;
const LESS_THAN: i128 = 7;
const EQUALS: i128 = 8;
const SET_RELATIVE: i128 = 9;
const QUIT: i128 = 99;

enum ParamMode {
    Positional,
    Immediate,
    Relative,
}

impl From<i128> for ParamMode {
    fn from(value: i128) -> Self {
        match value {
            0 => Self::Positional,
            1 => Self::Immediate,
            2 => Self::Relative,
            x => panic!("{x} is not a parameter mode"),
        }
    }
}

impl IntCPU {
    pub fn parse_program(input: &str) -> Program {
        input
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect()
    }

    pub fn from_str(input: &str) -> Self {
        let program = Self::parse_program(input);
        Self::new(&program)
    }

    pub fn new(program: &Program) -> Self {
        Self {
            program: program.clone(),
            heap: FxHashMap::default(),
            ptr: 0,
            input_queue: VecDeque::new(),
            output_queue: VecDeque::new(),
            halted: false,
            relative_base: 0,
        }
    }

    pub fn init(&mut self, pos_1: i128, pos_2: i128) {
        self.program[1] = pos_1;
        self.program[2] = pos_2;
    }

    pub fn push_input(&mut self, value: i128) {
        self.input_queue.push_back(value);
    }

    pub fn last_output(&mut self) -> Option<i128> {
        self.output_queue.iter().last().cloned()
    }

    pub fn exec(&mut self) {
        loop {
            if self.step(false) {
                break;
            }
        }
    }

    pub fn next(&mut self) -> Option<i128> {
        loop {
            if self.step(true) {
                return self.output_queue.iter().last().cloned();
            }
        }
    }

    pub fn unfinished(&self) -> bool {
        !self.halted
    }

    pub fn outputs(&self) -> Vec<i128> {
        self.output_queue.iter().map(|x| *x).collect()
    }

    fn step(&mut self, pause_on_output: bool) -> bool {
        let (ins, mode_1, mode_2, mode_3) = self.param_modes();
        match ins {
            ADD => self.add(mode_1, mode_2, mode_3),
            MULT => self.mult(mode_1, mode_2, mode_3),
            INPUT => self.input(mode_1),
            OUTPUT => {
                self.output(mode_1);
                if pause_on_output {
                    return true;
                }
            }
            JUMP_IF_TRUE => self.jump(true, mode_1, mode_2),
            JUMP_IF_FALSE => self.jump(false, mode_1, mode_2),
            LESS_THAN => self.less_than(mode_1, mode_2, mode_3),
            EQUALS => self.equals(mode_1, mode_2, mode_3),
            SET_RELATIVE => self.set_relative(mode_1),
            QUIT => {
                self.halted = true;
                return true;
            }
            _ => unimplemented!(),
        }
        // println!("{:?}", self.program);
        false
    }

    pub fn get(&mut self, idx: i128) -> i128 {
        if idx >= self.program.len() as i128 {
            *self.heap.get(&idx).unwrap_or(&0)
        } else {
            self.program[idx as usize]
        }
    }

    fn get_as(&mut self, idx: i128, mode: ParamMode) -> i128 {
        let value = self.program[idx as usize];
        match mode {
            ParamMode::Positional => self.get(value),
            ParamMode::Immediate => value,
            ParamMode::Relative => self.get(value + self.relative_base),
        }
    }

    fn set(&mut self, idx: i128, value: i128) {
        if idx >= self.program.len() as i128 {
            self.heap.insert(idx, value);
        } else {
            self.program[idx as usize] = value;
        }
    }

    fn set_with_mode(&mut self, idx: i128, value: i128, mode: ParamMode) {
        let ptr = self.program[idx as usize];
        // self.ensure(ptr);
        match mode {
            ParamMode::Positional => self.set(ptr, value),
            ParamMode::Immediate => panic!("Cannot set in immediate mode"),
            ParamMode::Relative => self.set(ptr + self.relative_base, value),
        }
    }

    // fn ensure(&mut self, idx: i128) {
    //     let idx_usize = idx as usize;
    //     if idx_usize >= self.program.len() {
    //         self.program.resize(idx_usize + 1, 0);
    //     }
    // }

    fn param_modes(&mut self) -> (i128, ParamMode, ParamMode, ParamMode) {
        let ptr = self.ptr;
        let op = self.program[ptr as usize];
        let op_code = op % 100;
        let param_1_positional = (op % 1000) / 100;
        let param_2_positional = (op % 10_000) / 1000;
        let param_3_positional = (op % 100_000) / 10_000;
        (
            op_code,
            param_1_positional.into(),
            param_2_positional.into(),
            param_3_positional.into(),
        )
    }

    fn add(&mut self, mode_1: ParamMode, mode_2: ParamMode, mode_3: ParamMode) {
        let ptr = self.ptr;
        let a = self.get_as(ptr + 1, mode_1);
        let b = self.get_as(ptr + 2, mode_2);
        self.set_with_mode(ptr + 3, a + b, mode_3);
        self.ptr += 4;
    }

    fn mult(&mut self, mode_1: ParamMode, mode_2: ParamMode, mode_3: ParamMode) {
        let ptr = self.ptr;
        let a = self.get_as(ptr + 1, mode_1);
        let b = self.get_as(ptr + 2, mode_2);
        self.set_with_mode(ptr + 3, a * b, mode_3);
        self.ptr += 4;
    }

    fn input(&mut self, mode_1: ParamMode) {
        // let pos = self.get_as(self.ptr + 1, mode_1);
        let input = self.input_queue.pop_front();
        if let Some(input) = input {
            self.set_with_mode(self.ptr + 1, input, mode_1);
        } else {
            panic!("NO INPUT FOUND");
        }
        self.ptr += 2;
    }

    fn output(&mut self, mode_1: ParamMode) {
        let out = self.get_as(self.ptr + 1, mode_1);
        self.output_queue.push_back(out);
        self.ptr += 2;
    }

    fn jump(&mut self, param_if: bool, mode_1: ParamMode, mode_2: ParamMode) {
        let ptr = self.ptr;
        let target = self.get_as(ptr + 1, mode_1);
        self.ptr = if (param_if && target > 0) || (!param_if && target == 0) {
            self.get_as(ptr + 2, mode_2)
        } else {
            ptr + 3
        }
    }

    fn less_than(&mut self, mode_1: ParamMode, mode_2: ParamMode, mode_3: ParamMode) {
        let ptr = self.ptr;
        let a = self.get_as(ptr + 1, mode_1);
        let b = self.get_as(ptr + 2, mode_2);
        self.set_with_mode(ptr + 3, if a < b { 1 } else { 0 }, mode_3);
        self.ptr += 4;
    }

    fn equals(&mut self, mode_1: ParamMode, mode_2: ParamMode, mode_3: ParamMode) {
        let ptr = self.ptr;
        let a = self.get_as(ptr + 1, mode_1);
        let b = self.get_as(ptr + 2, mode_2);
        self.set_with_mode(ptr + 3, if a == b { 1 } else { 0 }, mode_3);
        self.ptr += 4;
    }

    fn set_relative(&mut self, mode_1: ParamMode) {
        let ptr = self.ptr;
        let param = self.get_as(ptr + 1, mode_1);
        self.relative_base += param;
        self.ptr += 2;
    }
}

#[test]
fn day_2_samples() {
    let mut cpu = IntCPU::from_str("1,9,10,3,2,3,11,0,99,30,40,50");
    cpu.exec();
    assert_eq!(3500, cpu.get(0));

    let mut cpu = IntCPU::from_str("1,0,0,0,99");
    cpu.exec();
    assert_eq!(2, cpu.get(0));

    let mut cpu = IntCPU::from_str("2,3,0,3,99");
    cpu.exec();
    assert_eq!(2, cpu.get(0));

    let mut cpu = IntCPU::from_str("2,4,4,5,99,0");
    cpu.exec();
    assert_eq!(2, cpu.get(0));

    let mut cpu = IntCPU::from_str("1,1,1,4,99,5,6,0,99");
    cpu.exec();
    assert_eq!(30, cpu.get(0));
}

#[test]
fn day_9_samples() {
    let mut cpu = IntCPU::from_str("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    cpu.exec();
    assert_eq!(
        [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
        ],
        &cpu.outputs()[..]
    );

    let mut cpu = IntCPU::from_str("1102,34915192,34915192,7,4,7,99,0");
    assert_eq!(Some(1_219_070_632_396_864), cpu.next());
}
