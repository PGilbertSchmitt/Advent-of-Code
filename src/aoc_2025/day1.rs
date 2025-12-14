use logos::Logos;

#[derive(Logos, Debug)]
#[logos(skip r"\n")]
enum Token {
    #[regex("R[0-9]+", |lex| lex.slice()[1..].parse::<isize>().unwrap())]
    Right(isize),

    #[regex("L[0-9]+", |lex| lex.slice()[1..].parse::<isize>().unwrap())]
    Left(isize),
}

const SAMPLE: &'static str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

const INPUT: &'static str = include_str!("inputs/day1.txt");

fn count_zeros(input: &str) -> usize {
    let mut count = 0;
    let mut dial: isize = 50;

    let mut lex = Token::lexer(input);
    while let Some(Ok(token)) = lex.next() {
        match token {
            Token::Right(amt) => {
                dial += amt;
                dial %= 100;
            }
            Token::Left(amt) => {
                dial -= amt;
                dial %= 100;
            }
        }
        if dial == 0 {
            count += 1;
        }
    }

    count
}

fn count_clicks_at_zero(input: &str) -> isize {
    let mut count = 0;
    let mut dial: isize = 50;

    let mut lex = Token::lexer(input);
    while let Some(Ok(token)) = lex.next() {
        match token {
            Token::Right(amt) => {
                count += amt / 100;
                let new_amt = amt % 100;

                dial += new_amt;
                if dial > 99 {
                    count += 1;
                    dial -= 100;
                }
            }
            Token::Left(amt) => {
                count += amt / 100;
                let new_amt = amt % 100;

                let last_dial = dial;
                dial -= new_amt;
                if dial < 1 && last_dial != 0 {
                    count += 1;
                }
                if dial < 0 {
                    dial += 100;
                }
            }
        }
    }

    count
}

#[test]
fn part1() {
    assert_eq!(count_zeros(SAMPLE), 3);
    assert_eq!(count_zeros(INPUT), 1036);
}

#[test]
fn part2() {
    assert_eq!(count_clicks_at_zero(SAMPLE), 6);
    assert_eq!(count_clicks_at_zero(INPUT), 6228);
}
