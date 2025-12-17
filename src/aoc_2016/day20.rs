use std::u32;

use logos::Logos;

use crate::util::heap::MinHeap;

const SAMPLE: &str = "
5-8
0-2
4-7";

const INPUT: &str = include_str!("./inputs/day20.txt");

#[derive(Logos)]
#[logos(skip r"\n")]
enum Token {
    #[regex("[0-9]+-[0-9]+", |lex| {
        let mut parts = lex.slice().split('-');
        (
            parts.next().unwrap().parse::<u32>().unwrap(),
            parts.next().unwrap().parse::<u32>().unwrap(),
        )
    })]
    Range((u32, u32)),
}

fn parse_sorted_ranges(input: &str) -> MinHeap<u32, u32> {
    let mut sorted_by_start = MinHeap::default();
    let mut tokens = Token::lexer(input);

    while let Some(Ok(Token::Range((start, end)))) = tokens.next() {
        sorted_by_start.push(start, end);
    }

    sorted_by_start
}

fn lowest_unblocked_ip(non_overlapping_ranges: &Vec<(u32, u32)>) -> u32 {
    let mut lowest: u32 = 0;
    for &(start, end) in non_overlapping_ranges {
        if lowest < start {
            break;
        }
        if lowest >= start || lowest <= end {
            lowest = lowest.max(end + 1);
        }
    }

    lowest
}

fn merge_ranges(mut sorted_ranges: MinHeap<u32, u32>) -> Vec<(u32, u32)> {
    let mut ranges = Vec::new();
    ranges.push(sorted_ranges.pop().unwrap());

    while let Some((start, end)) = sorted_ranges.pop() {
        let (_, p_end) = ranges.last_mut().unwrap();
        if start <= *p_end {
            if end > *p_end {
                *p_end = end;
            }
        } else {
            ranges.push((start, end));
        }
    }

    ranges
}

fn total_unblocked_ips(max: u32, non_overlapping_ranges: &Vec<(u32, u32)>) -> u32 {
    let mut count = max;

    for &(start, end) in non_overlapping_ranges {
        println!("{start} -> {end}");
        count -= end - start + 1;
    }

    1 + count
}

#[test]
fn part_1() {
    let merged_ranges = merge_ranges(parse_sorted_ranges(SAMPLE));
    assert_eq!(3, lowest_unblocked_ip(&merged_ranges));
    assert_eq!(2, total_unblocked_ips(9, &merged_ranges));
    let merged_ranges = merge_ranges(parse_sorted_ranges(INPUT));
    assert_eq!(31053880, lowest_unblocked_ip(&merged_ranges));
    assert_eq!(117, total_unblocked_ips(u32::MAX, &merged_ranges));
}
