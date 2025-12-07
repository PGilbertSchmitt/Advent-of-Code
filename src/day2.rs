use fxhash::FxHashSet;
use logos::Logos;

const SAMPLE: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

const INPUT: &'static str = include_str!("./inputs/day2.txt");

#[derive(Debug, Logos)]
#[logos(skip r"[,\-\n]")]
enum Token {
    #[regex("[0-9]+", |lex| lex.slice().to_owned())]
    Id(String),
}

fn parse_and_split(value: &str, up: bool) -> (usize, usize) {
    let is_even = value.len() % 2 == 0;
    let len = value.len();
    if is_even {
        (
            value.parse::<usize>().unwrap(),
            value[0..len / 2].parse().unwrap(),
        )
    } else {
        let new_len = if up { len } else { len - 1 } as u32;
        let full_value = (10 as usize).pow(new_len);
        let half_value = (10 as usize).pow(new_len / 2);
        if up {
            (full_value, half_value)
        } else {
            (full_value - 1, half_value - 1)
        }
    }
}

fn find_simple_invalid_ids(first: &str, second: &str) -> Vec<usize> {
    let mut invalid_ids = Vec::new();

    let (first_value, first_half_of_start) = parse_and_split(first, true);
    let (second_value, first_half_of_end) = parse_and_split(second, false);

    for x in first_half_of_start..first_half_of_end + 1 {
        let x_len = x.checked_ilog10().unwrap_or(0) + 1;
        let invalid_id = x * (10 as usize).pow(x_len) + x;
        if invalid_id >= first_value && invalid_id <= second_value {
            invalid_ids.push(invalid_id);
        }
    }

    invalid_ids
}

fn get_sum_of_invalids(input: &str) -> usize {
    let mut input = Token::lexer(input);
    let mut sum: usize = 0;
    while let Some(Ok(Token::Id(first))) = input.next() {
        let Token::Id(second) = input.next().unwrap().unwrap();
        sum += find_simple_invalid_ids(&first, &second)
            .iter()
            .sum::<usize>();
    }
    sum
}

fn possible_repeat_values(start: &str, end: &str) -> Vec<usize> {
    let len = start.len();
    if len != end.len() {
        let split_value = (10 as usize).pow(start.len() as u32);
        let mut all = possible_repeat_values(start, &(split_value - 1).to_string());
        all.append(&mut possible_repeat_values(&split_value.to_string(), end));
        return all;
    }

    let half_len = len / 2;
    let mut ids = Vec::new();
    for x in 1..half_len + 1 {
        if len % x == 0 {
            let range_start: usize = start[0..x].parse().unwrap();
            let range_end: usize = end[0..x].parse().unwrap();
            for segment in range_start..range_end + 1 {
                ids.push(segment.to_string().repeat(len / x).parse().unwrap());
            }
        }
    }

    ids
}

fn complex_id_sum(start: &str, end: &str) -> usize {
    let possible_ids: FxHashSet<usize> = possible_repeat_values(start, end).into_iter().collect();

    let start: usize = start.parse().unwrap();
    let end: usize = end.parse().unwrap();
    let mut invalid_ids = Vec::new();
    for x in possible_ids {
        if x >= start && x <= end {
            invalid_ids.push(x);
        }
    }

    invalid_ids.iter().sum()
}

fn get_sum_of_complex_invalids(input: &str) -> usize {
    let mut input = Token::lexer(input);
    let mut sum: usize = 0;
    while let Some(Ok(Token::Id(first))) = input.next() {
        let Token::Id(second) = input.next().unwrap().unwrap();
        sum += complex_id_sum(&first, &second);
    }
    sum
}

#[test]
fn part1() {
    assert_eq!(1227775554, get_sum_of_invalids(SAMPLE));
    assert_eq!(23560874270, get_sum_of_invalids(INPUT));
}

#[test]
fn part2() {
    assert_eq!(4174379265, get_sum_of_complex_invalids(SAMPLE));
    assert_eq!(44143124633, get_sum_of_complex_invalids(INPUT));
}
