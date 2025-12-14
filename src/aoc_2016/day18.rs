const SAMPLE_1: &str = "..^^.";
const SAMPLE_2: &str = ".^^.^.^^^^";
const INPUT: &str = include_str!("./inputs/day18.txt");

fn parse_input(input: &str) -> Vec<bool> {
    input.chars().map(|ch| ch == '^').collect()
}

fn count_safe_spaces(row: &Vec<bool>) -> usize {
    row.iter().filter(|x| !**x).count()
}

fn generate_row(row: Vec<bool>) -> Vec<bool> {
    let mut new_row = Vec::with_capacity(row.len());
    for (idx, middle) in row.iter().enumerate() {
        let left = if idx == 0 { false } else { *row.get(idx - 1).unwrap() };
        let right = if idx == row.len() - 1 { false } else { *row.get(idx + 1).unwrap() };
        new_row.push(
            match (left, *middle, right) {
                (true, true, false) | (false, true, true) | (true, false, false) | (false, false, true) => true,
                _ => false,
            }
        );
    }
    new_row
}

fn calculate_generations(input: &str, generations: usize) -> usize {
    let mut row = parse_input(input);
    let mut total_traps = count_safe_spaces(&row);
    for _ in 1..generations {
        row = generate_row(row);
        total_traps += count_safe_spaces(&row);
    }
    total_traps
}

#[test]
fn both_parts() {
    assert_eq!(6, calculate_generations(SAMPLE_1, 3));
    assert_eq!(38, calculate_generations(SAMPLE_2, 10));
    assert_eq!(1951, calculate_generations(INPUT, 40));
    assert_eq!(20_002_936, calculate_generations(INPUT, 400_000));
}

