use std::collections::VecDeque;

use itertools::Itertools;
use logos::{Lexer, Logos};

const INPUT: &str = include_str!("./inputs/day21.txt");

const SAMPLE: &str = "
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

#[derive(Logos, Debug)]
#[logos(skip r"[\n ]", skip "on", skip "with", skip "to", skip "through", skip "step", skip "steps", skip "position", skip "letter")]
enum Token {
    #[token("swap position")]
    SwapPositions,

    #[token("swap letter")]
    SwapLetters,

    #[token("move")]
    Move,

    #[token("reverse positions")]
    Reverse,

    #[token("rotate based on position of letter")]
    RotateBasedOn,
    
    #[token("rotate left")]
    RotateLeft,

    #[token("rotate right")]
    RotateRight,

    #[regex("[a-z]", |lex| lex.slice().chars().next().unwrap())]
    Char(char),

    #[regex("[0-9]", |lex| lex.slice().chars().next().unwrap().to_digit(10).unwrap())]
    Number(u32),
}

impl Token {
    fn get_num(self) -> usize {
        if let Self::Number(x) = self {
            x as usize
        } else {
            panic!("Was not a number")
        }
    }

    fn get_char(self) -> char {
        if let Self::Char(ch) = self {
            ch
        } else {
            panic!("Was not a char")
        }
    }
}

enum Instruction {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOn(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

fn next_num(tokens: &mut Lexer<Token>) -> usize {
    tokens.next().unwrap().unwrap().get_num()
}

fn next_char(tokens: &mut Lexer<Token>) -> char {
    tokens.next().unwrap().unwrap().get_char()
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut ins = Vec::new();
    let mut tokens = Token::lexer(input);
    while let Some(Ok(token)) = tokens.next() {
        match token {
            Token::SwapPositions => {
                let x = next_num(&mut tokens);
                let y = next_num(&mut tokens);
                ins.push(Instruction::SwapPositions(x, y));
            }
            Token::SwapLetters => {
                let x = next_char(&mut tokens);
                let y = next_char(&mut tokens);
                ins.push(Instruction::SwapLetters(x, y));
            }
            Token::RotateBasedOn => {
                let x = next_char(&mut tokens);
                ins.push(Instruction::RotateBasedOn(x));
            }
            Token::RotateLeft => {
                let x = next_num(&mut tokens);
                ins.push(Instruction::RotateLeft(x));
            }
            Token::RotateRight => {
                let x = next_num(&mut tokens);
                ins.push(Instruction::RotateRight(x));
            }
            Token::Reverse => {
                let x = next_num(&mut tokens);
                let y = next_num(&mut tokens);
                ins.push(Instruction::Reverse(x, y));
            }
            Token::Move => {
                let x = next_num(&mut tokens);
                let y = next_num(&mut tokens);
                ins.push(Instruction::Move(x, y));
            }
            Token::Char(_) | Token::Number(_) => panic!("Should not parse char or number outside of a command"),
        }
    }

    ins
}

/** Manual String */
type MStr = VecDeque<char>;

// SwapPositions(usize, usize),
fn swap_positions(s: &mut MStr, x: usize, y: usize) {
    let tmp = s.get(x).unwrap().to_owned();
    s[x] = s[y];
    s[y] = tmp;
}

// SwapLetters(char, char),
fn swap_letters(s: &mut MStr, a: char, b: char) {
    let a_locations: Vec<usize> = s
        .iter()
        .enumerate()
        .filter_map(|(i, &ch)| if ch == a { Some(i) } else { None })
        .collect();

    for ch in s.iter_mut() {
        if *ch == b {
            *ch = a;
        }
    }

    for loc in a_locations {
        s[loc] = b;
    };
}

// RotateLeft(usize),
fn rotate_left(s: &mut MStr, x: usize) {
    for _ in 0..x {
        let front = s.pop_front().unwrap();
        s.push_back(front);
    }
}

// RotateRight(usize),
fn rotate_right(s: &mut MStr, x: usize) {
    for _ in 0..x {
        let end = s.pop_back().unwrap();
        s.push_front(end);
    }
}

// RotateBasedOn(char),
fn rotate_based_on(s: &mut MStr, a: char) {
    let idx = s.iter().find_position(|&&ch| ch == a);
    if let Some((idx, _)) = idx {
        let rotations = idx + if idx >= 4 { 2 } else { 1 };
        rotate_right(s, rotations);
    } else {
        panic!("Could not find char {a} in string");
    }
}

// Reverse(usize, usize),
fn reverse(s: &mut MStr, x: usize, y: usize) {
    let sub_section: Vec<char> = s.range(x..y + 1).rev().cloned().collect();
    for (i, ch) in s.range_mut(x..y + 1).enumerate() {
        *ch = sub_section[i];
    }
}

// Move(usize, usize),
fn move_up(s: &mut MStr, x: usize, y: usize) {
    let moved_char = s[x];
    let sub_section: Vec<char> = s.range(x + 1..y + 1).cloned().collect();
    for (i, ch) in s.range_mut(x..y).enumerate() {
        *ch = sub_section[i];
    }
    s[y] = moved_char;
}

fn move_down(s: &mut MStr, x: usize, y: usize) {
    let moved_char = s[y];
    let sub_section: Vec<char> = s.range(x..y).cloned().collect();
    for (i, ch) in s.range_mut(x + 1..y + 1).enumerate() {
        *ch = sub_section[i];
    }
    s[x] = moved_char;
}

fn undo_rotate_based_on(s: &mut MStr, a: char) {
    // There is a way I could convert the indexes, but the input string is only 8 characters,
    // so it's a lot easier to just map them
    let idx = s.iter().find_position(|&&ch| ch == a).unwrap().0;
    match idx {
        0 => rotate_left(s, 1),
        1 => rotate_left(s, 1),
        2 => rotate_right(s, 2),
        3 => rotate_left(s, 2),
        4 => rotate_right(s, 1),
        5 => rotate_left(s, 3),
        6 => {},
        7 => rotate_right(s, 4),
        _ => unreachable!(),
    }
}

fn process_password(password: String, instructions: &Vec<Instruction>) -> String {
    let mut s = VecDeque::from_iter(password.chars());
    for instruction in instructions {
        match instruction {
            Instruction::SwapPositions(x, y) => swap_positions(&mut s, *x, *y),
            Instruction::SwapLetters(a, b) => swap_letters(&mut s, *a, *b),
            Instruction::RotateLeft(x) => rotate_left(&mut s, *x),
            Instruction::RotateRight(x) => rotate_right(&mut s, *x),
            Instruction::RotateBasedOn(ch) => rotate_based_on(&mut s, *ch),
            Instruction::Reverse(x, y) => reverse(&mut s, *x, *y),
            Instruction::Move(x, y) => {
                if *x < *y {
                    move_up(&mut s, *x, *y);
                } else {
                    move_down(&mut s, *y, *x);
                }
            }
        }
    }
    s.iter().join("")
}

fn unprocess_password(password: String, instructions: &Vec<Instruction>) -> String {
    let mut s = VecDeque::from_iter(password.chars());
    for instruction in instructions {
        match instruction {
            // Unchanged
            Instruction::SwapPositions(x, y) => swap_positions(&mut s, *x, *y),
            // Unchanged
            Instruction::SwapLetters(a, b) => swap_letters(&mut s, *a, *b),
            // Swapped between left and right
            Instruction::RotateLeft(x) => rotate_right(&mut s, *x),
            Instruction::RotateRight(x) => rotate_left(&mut s, *x),

            // Special instruction
            Instruction::RotateBasedOn(ch) => undo_rotate_based_on(&mut s, *ch),

            // Unchanged
            Instruction::Reverse(x, y) => reverse(&mut s, *x, *y),

            // Swap the vars
            Instruction::Move(y, x) => {
                if *x < *y {
                    move_up(&mut s, *x, *y);
                } else {
                    move_down(&mut s, *y, *x);
                }
            }
        }
    }
    s.iter().join("")
}

#[test]
fn both_parts() {
    let instructions: Vec<Instruction> = parse_instructions(SAMPLE);
    assert_eq!("decab", &process_password(String::from("abcde"), &instructions));

    let instructions: Vec<Instruction> = parse_instructions(INPUT);
    assert_eq!("cbeghdaf", &process_password(String::from("abcdefgh"), &instructions));

    let reverse_instructions: Vec<Instruction> = instructions.into_iter().rev().collect();
    assert_eq!("bacdefgh", &unprocess_password(String::from("fbgdceah"), &reverse_instructions));
}
