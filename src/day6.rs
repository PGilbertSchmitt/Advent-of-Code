use logos::Logos;

const SAMPLE: &'static str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

const INPUT: &'static str = include_str!("./inputs/day6.txt");

#[derive(Logos)]
#[logos(skip r" ")]
enum Token {
    #[token("\n")]
    NewLine,

    #[token("+")]
    Plus,

    #[token("*")]
    Star,

    #[regex("[0-9]+", |lex| lex.slice().parse::<usize>().unwrap())]
    Number(usize),
}

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

impl Op {
    fn eval(&self, a: usize, b: usize) -> usize {
        match self {
            Op::Add => a + b,
            Op::Multiply => a * b,
        }
    }

    fn fold_init(&self) -> usize {
        match self {
            Op::Add => 0,
            Op::Multiply => 1,
        }
    }

    fn from(ch: &char) -> Option<Op> {
        match ch {
            '+' => Some(Self::Add),
            '*' => Some(Self::Multiply),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Homework {
    rows: Vec<Vec<usize>>,
    operations: Vec<Op>,
}

fn parse_homework(input: &str) -> Homework {
    let mut homework = Homework {
        rows: Vec::new(),
        operations: Vec::new(),
    };
    let mut lex = Token::lexer(input.trim());
    let mut cur_row = Vec::new();
    let mut ops = Vec::new();
    while let Some(Ok(token)) = lex.next() {
        match token {
            Token::Number(val) => {
                cur_row.push(val);
            }
            Token::NewLine => {
                if ops.is_empty() {
                    homework.rows.push(cur_row);
                    cur_row = Vec::new();
                }
            }
            Token::Plus => {
                ops.push(Op::Add);
            }
            Token::Star => {
                ops.push(Op::Multiply);
            }
        }
    }
    homework.operations = ops;

    for row in &homework.rows {
        if row.len() != homework.operations.len() {
            panic!("Homework was misaligned");
        }
    }

    homework
}

fn eval_cephalopod_homework(input: &str) -> usize {
    let mut input_rows: Vec<Vec<char>> = input
        .trim_matches('\n')
        .split("\n")
        .map(|row| row.chars().collect())
        .collect();
    let op_row = input_rows.pop().unwrap();

    for row in &input_rows {
        if row.len() != op_row.len() {
            panic!("Cephalopod Homework input was misaligned");
        }
    }

    let mut sum = 0;
    let mut cur_problem = 0;
    let mut cur_op = Op::Add; // Doesn't matter what it is, because it will be overwritten immedately
    for idx in 0..op_row.len() {
        // As long as the input is well formatted, this should stay in sync with the digit parser
        if let Some(op) = Op::from(&op_row[idx]) {
            cur_op = op;
            cur_problem = cur_op.fold_init();
        }

        let mut nil_colummn = true;
        let mut cur_value: usize = 0;
        for row in input_rows.iter() {
            if let Some(digit) = row[idx].to_digit(10) {
                cur_value = cur_value * 10 + digit as usize;
                nil_colummn = false;
            }
        }

        if nil_colummn {
            sum += cur_problem;
        } else {
            cur_problem = cur_op.eval(cur_problem, cur_value);
        }
    }
    // And the last column
    sum += cur_problem;

    sum
}

fn evaluate(homework: &Homework) -> usize {
    homework
        .operations
        .iter()
        .enumerate()
        .map(|(idx, op)| {
            homework.rows.iter().fold(op.fold_init(), |acc, row| {
                op.eval(acc, *row.get(idx).unwrap())
            })
        })
        .sum()
}

#[test]
fn part1() {
    assert_eq!(4_277_556, evaluate(&parse_homework(SAMPLE)));
    assert_eq!(6_171_290_547_579, evaluate(&parse_homework(INPUT)));
}

#[test]
fn part2() {
    assert_eq!(3_263_827, eval_cephalopod_homework(SAMPLE));
    assert_eq!(8_811_937_976_367, eval_cephalopod_homework(INPUT));
}
