const SAMPLE: &'static str = "
987654321111111
811111111111119
234234234234278
818181911112111";

const INPUT: &'static str = include_str!("./inputs/day3.txt");

fn get_banks(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .split("\n")
        .map(|s| {
            s.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn get_largest_joltage(bank: &Vec<usize>) -> usize {
    let mut tens_place_idx = 0;
    let mut tens_place = bank[0];
    for idx in 1..bank.len() - 1 {
        let val = bank[idx];
        if val > tens_place {
            tens_place_idx = idx;
            tens_place = val;
        }
    }

    let ones_place_idx = tens_place_idx + 1;
    let mut ones_place = bank[ones_place_idx];
    for idx in ones_place_idx + 1..bank.len() {
        let val = bank[idx];
        if val > ones_place {
            ones_place = val;
        }
    }

    tens_place * 10 + ones_place
}

fn get_largest_extreme_joltage(bank: &[usize], digits: u32) -> usize {
    if digits == 0 {
        return 0;
    }

    let last_idx = bank.len() - (digits - 1) as usize;
    // println!("Bank: {bank:?}, digits left = {digits}, so last idx = {last_idx}");
    let search_space = &bank[0..last_idx];
    let mut cur_high_idx = 0;
    let mut cur_high = search_space[0];
    for idx in 1..search_space.len() {
        let val = search_space[idx];
        if val > cur_high {
            cur_high_idx = idx;
            cur_high = val;
        }
    }

    cur_high * (10 as usize).pow(digits - 1)
        + get_largest_extreme_joltage(&bank[cur_high_idx + 1..], digits - 1)
}

fn total_joltages(input: &str) -> usize {
    get_banks(input).iter().map(get_largest_joltage).sum()
}

fn total_extreme_joltages(input: &str) -> usize {
    get_banks(input)
        .iter()
        .map(|bank| get_largest_extreme_joltage(&bank[..], 12))
        .sum()
}

#[test]
fn part1() {
    assert_eq!(357, total_joltages(SAMPLE));
    assert_eq!(17435, total_joltages(INPUT));
}

#[test]
fn part2() {
    assert_eq!(3_121_910_778_619, total_extreme_joltages(SAMPLE));
    assert_eq!(172_886_048_065_379, total_extreme_joltages(INPUT));
}
