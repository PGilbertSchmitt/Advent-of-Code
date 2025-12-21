use std::collections::VecDeque;

type Program = Vec<i64>;

pub fn parse_program(input: &str) -> Program {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

pub struct IntCPU {
    program: Program,
    orig_program: Program,
    len: i64,
    ptr: i64,
    input_queue: VecDeque<i64>,
    pub output_queue: VecDeque<i64>,
}

const ADD: i64 = 1;
const MULT: i64 = 2;
const INPUT: i64 = 3;
const OUTPUT: i64 = 4;
const JUMP_IF_TRUE: i64 = 5;
const JUMP_IF_FALSE: i64 = 6;
const LESS_THAN: i64 = 7;
const EQUALS: i64 = 8;
const QUIT: i64 = 99;

impl IntCPU {
    fn parse_program(input: &str) -> Program {
        input
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect()
    }

    pub fn new(input: &str) -> Self {
        let program = Self::parse_program(input);
        Self {
            len: program.len() as i64,
            orig_program: program.clone(),
            program,
            ptr: 0,
            input_queue: VecDeque::new(),
            output_queue: VecDeque::new(),
        }
    }

    pub fn reset(&mut self) {
        self.ptr = 0;
        self.program = self.orig_program.clone();
        self.input_queue.clear();
        self.output_queue.clear();
    }

    pub fn init(&mut self, pos_1: i64, pos_2: i64) {
        self.reset();
        self.program[1] = pos_1;
        self.program[2] = pos_2;
    }

    pub fn push_input(&mut self, value: i64) {
        self.input_queue.push_back(value);
    }

    #[inline]
    pub fn len(&self) -> i64 {
        self.len
    }

    pub fn exec(&mut self) {
        loop {
            let (ins, mode_1, mode_2, _mode_3) = self.param_modes();
            match ins {
                ADD => self.add(mode_1, mode_2),
                MULT => self.mult(mode_1, mode_2),
                INPUT => self.input(),
                OUTPUT => self.output(mode_1),
                JUMP_IF_TRUE => self.jump(true, mode_1, mode_2),
                JUMP_IF_FALSE => self.jump(false, mode_1, mode_2),
                LESS_THAN => self.less_than(mode_1, mode_2),
                EQUALS => self.equals(mode_1, mode_2),
                QUIT => break,
                _ => unimplemented!(),
            }
        }
    }

    #[inline]
    pub fn get(&self, idx: i64) -> i64 {
        self.program[idx as usize]
    }

    fn get_as(&self, idx: i64, positional: bool) -> i64 {
        let value = self.program[idx as usize];
        if positional {
            self.get(value)
        } else {
            value
        }
    }

    #[inline]
    fn set(&mut self, idx: i64, value: i64) {
        self.program[idx as usize] = value;
    }

    fn param_modes(&mut self) -> (i64, bool, bool, bool) {
        let ptr = self.ptr;
        let op = self.program[ptr as usize];
        let op_code = op % 100;
        let param_1_positional = (op % 1000) / 100 == 0;
        let param_2_positional = (op % 10_000) / 1000 == 0;
        let param_3_positional = (op % 100_000) / 10_000 == 0;
        (op_code, param_1_positional, param_2_positional, param_3_positional)
    }

    fn add(&mut self, mode_1: bool, mode_2: bool) {
        let ptr = self.ptr;
        let a = self.get_as(ptr + 1, mode_1);
        let b = self.get_as(ptr + 2, mode_2);
        let out = self.get(ptr + 3);
        self.set(out, a + b);
        self.ptr += 4;
    }

    fn mult(&mut self, mode_1: bool, mode_2: bool) {
        let ptr = self.ptr;
        let a = self.get_as(ptr + 1, mode_1);
        let b = self.get_as(ptr + 2, mode_2);
        let out = self.get(ptr + 3);
        self.set(out, a * b);
        self.ptr += 4;
    }

    fn input(&mut self) {
        let pos = self.get(self.ptr + 1);
        let input = self.input_queue.pop_front();
        if let Some(input) = input {
            self.set(pos, input);
        } else {
            panic!("NO INPUT FOUND");
        }
        self.ptr += 2;
    }

    fn output(&mut self, mode_1: bool) {
        let out = self.get_as(self.ptr + 1, mode_1);
        self.output_queue.push_back(out);
        self.ptr += 2;
    }

    fn jump(&mut self, param_if: bool, mode_1: bool, mode_2: bool) {
        let ptr = self.ptr;
        let target = self.get_as(ptr + 1, mode_1);
        self.ptr = if (param_if && target > 0) || (!param_if && target == 0) {
            self.get_as(ptr + 2, mode_2)
        } else {
            ptr + 3
        }
    }

    // LESS_THAN
    fn less_than(&mut self, mode_1: bool, mode_2: bool) {
        let ptr = self.ptr;
        let a = self.get_as(ptr + 1, mode_1);
        let b = self.get_as(ptr + 2, mode_2);
        let out = self.get(ptr + 3);
        self.set(out, if a < b { 1 } else { 0 });
        self.ptr += 4;
    }

    // EQUALS

    // LESS_THAN
    fn equals(&mut self, mode_1: bool, mode_2: bool) {
        let ptr = self.ptr;
        let a = self.get_as(ptr + 1, mode_1);
        let b = self.get_as(ptr + 2, mode_2);
        let out = self.get(ptr + 3);
        self.set(out, if a == b { 1 } else { 0 });
        self.ptr += 4;
    }
}
