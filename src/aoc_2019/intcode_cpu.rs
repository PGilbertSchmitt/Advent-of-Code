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
}

const ADD: i64 = 1;
const MULT: i64 = 2;
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
        }
    }

    pub fn init(&mut self, pos_1: i64, pos_2: i64) {
        self.reset();
        self.program[1] = pos_1;
        self.program[2] = pos_2;
    }

    fn reset(&mut self) {
        self.ptr = 0;
        self.program = self.orig_program.clone();
    }

    pub fn len(&self) -> i64 {
        self.len
    }

    pub fn exec(&mut self) {
        loop {
            let ins = self.program[self.ptr as usize];
            match ins {
                ADD => self.add(),
                MULT => self.mult(),
                QUIT => break,
                _ => unimplemented!(),
            }
        }
    }

    #[inline]
    pub fn get(&self, idx: i64) -> i64 {
        self.program[idx as usize]
    }

    #[inline]
    fn set(&mut self, idx: i64, value: i64) {
        if idx <= self.len {
            // self.program.append(vec![0]);
        }
        self.program[idx as usize] = value;
    }

    #[inline]
    fn ensure(&self, operands: i64) {
        if self.ptr + operands + 1 > self.len {
            panic!("Not enough operands");
        }
    }

    fn add(&mut self) {
        let ptr = self.ptr;
        self.ensure(4);
        let a = self.get(self.get(ptr + 1));
        let b = self.get(self.get(ptr + 2));
        let out = self.get(ptr + 3);
        self.set(out, a + b);
        self.ptr += 4;
    }

    fn mult(&mut self) {
        let ptr = self.ptr;
        self.ensure(4);
        let a = self.get(self.get(ptr + 1));
        let b = self.get(self.get(ptr + 2));
        let out = self.get(ptr + 3);
        self.set(out, a * b);
        self.ptr += 4;
    }
}
