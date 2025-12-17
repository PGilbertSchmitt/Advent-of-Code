use logos::{Lexer, Logos};

#[derive(Logos)]
#[logos(skip r"[\n ]")]
pub enum Token {
    #[token("cpy")]
    Copy,

    #[token("inc")]
    Increment,

    #[token("dec")]
    Decrement,

    #[token("jnz")]
    JumpNotZero,

    #[token("tgl")]
    Toggle,

    #[token("out")]
    Out,

    #[regex("[a-z]", |lex| lex.slice().chars().nth(0))]
    Register(char),

    #[regex("-?[0-9]+", |lex| lex.slice().parse::<isize>().unwrap())]
    Value(isize),
}

pub enum Operand {
    Register(usize),
    Value(isize),
}

impl Operand {
    pub fn to_value(&self, registers: &Registers) -> isize {
        match self {
            Self::Value(val) => *val,
            Self::Register(reg) => registers[*reg],
        }
    }
}

impl From<Token> for Operand {
    fn from(value: Token) -> Self {
        match value {
            Token::Value(val) => Operand::Value(val),
            Token::Register(reg) => Operand::Register(match reg {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                _ => unreachable!(),
            }),
            _ => panic!("Can't convert Token to Datum"),
        }
    }
}

pub enum Instruction {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
    Tgl(Operand),
    Out(Operand),
}

fn get_operand(tokens: &mut Lexer<Token>) -> Operand {
    Operand::from(tokens.next().unwrap().unwrap())
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut tokens = Token::lexer(input);

    while let Some(Ok(token)) = tokens.next() {
        match token {
            Token::Copy => {
                let x = get_operand(&mut tokens);
                let y = get_operand(&mut tokens);
                instructions.push(Instruction::Cpy(x, y));
            }
            Token::Increment => {
                let x = get_operand(&mut tokens);
                instructions.push(Instruction::Inc(x));
            }
            Token::Decrement => {
                let x = get_operand(&mut tokens);
                instructions.push(Instruction::Dec(x));
            }
            Token::JumpNotZero => {
                let x = get_operand(&mut tokens);
                let y = get_operand(&mut tokens);
                instructions.push(Instruction::Jnz(x, y));
            }
            Token::Toggle => {
                let x = get_operand(&mut tokens);
                instructions.push(Instruction::Tgl(x));
            }
            Token::Out => {
                let x = get_operand(&mut tokens);
                instructions.push(Instruction::Out(x));
            }
            _ => panic!("Unexpected token"),
        }
    }

    instructions
}

pub type Registers = [isize; 4];

pub fn execute(ins: &Vec<Instruction>, a: isize, b: isize, c: isize, d: isize) -> isize {
    let mut ptr = 0;
    let mut state: Registers = [a, b, c, d];
    let len = ins.len();

    let mut toggles = vec![false; ins.len()];

    while ptr < len {
        let cur_ins = &ins[ptr];
        let toggle = toggles[ptr];

        match (toggle, cur_ins) {
            (false, Instruction::Cpy(x, y)) | (true, Instruction::Jnz(x, y)) => {
                if let Operand::Register(y) = y {
                    let src = x.to_value(&state);
                    state[*y] = src;
                }
                ptr += 1;
            }
            (false, Instruction::Inc(x))
            | (true, Instruction::Dec(x))
            | (true, Instruction::Tgl(x)) => {
                if let Operand::Register(x) = x {
                    state[*x] += 1;
                }
                ptr += 1;
            }
            (false, Instruction::Dec(x)) | (true, Instruction::Inc(x)) => {
                if let Operand::Register(x) = x {
                    state[*x] -= 1;
                }
                ptr += 1;
            }
            (false, Instruction::Jnz(x, y)) | (true, Instruction::Cpy(x, y)) => {
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
            (false, Instruction::Tgl(x)) => {
                let offset = x.to_value(&state);
                let toggled_ptr = (ptr as isize) + offset;
                if toggled_ptr >= 0 && toggled_ptr < toggles.len() as isize {
                    toggles[toggled_ptr as usize] = !toggles[toggled_ptr as usize];
                }
                ptr += 1;
            }
            // The program with Tgl doesn't have Out
            _ => unreachable!(),
        }
    }

    state[0]
}

const SAMPLE: &str = "
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

const INPUT: &str = include_str!("./inputs/day12.txt");

#[test]
fn both_parts() {
    let sample_ins = parse_instructions(SAMPLE);
    let input_ins = parse_instructions(INPUT);

    assert_eq!(42, execute(&sample_ins, 0, 0, 0, 0));
    assert_eq!(317993, execute(&input_ins, 0, 0, 0, 0));
    assert_eq!(9227647, execute(&input_ins, 0, 0, 1, 0));
}
