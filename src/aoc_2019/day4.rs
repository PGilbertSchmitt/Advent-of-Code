use std::ops::Range;

const INPUT: &str = include_str!("./inputs/day4.txt");

fn parse_input(input: &str) -> Range<u64> {
    let mut parts = input.trim().split("-");
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();
    start..end
}

fn dits(num: u64) -> (u64, u64, u64, u64, u64, u64) {
    (
        num / 100_000,
        (num % 100_000) / 10_000,
        (num % 10_000) / 1000,
        (num % 1000) / 100,
        (num % 100) / 10,
        num % 10,
    )
}

fn is_valid_password(num: u64) -> bool {
    let (x1, x2, x3, x4, x5, x6) = dits(num);
    x1 <= x2
        && x2 <= x3
        && x3 <= x4
        && x4 <= x5
        && x5 <= x6
        && (x1 == x2 || x2 == x3 || x3 == x4 || x4 == x5 || x5 == x6)
}

fn is_real_valid_password(num: u64) -> bool {
    let (x1, x2, x3, x4, x5, x6) = dits(num);
    (x1 == x2 && x2 != x3)
        || (x2 == x3 && x1 != x2 && x4 != x2)
        || (x3 == x4 && x2 != x3 && x5 != x3)
        || (x4 == x5 && x3 != x4 && x6 != x4)
        || (x5 == x6 && x4 != x5)
}

#[test]
fn solutions() {
    let mut part_1_passwords = 0;
    let mut part_2_passwords = 0;
    for password in parse_input(INPUT) {
        if is_valid_password(password) {
            part_1_passwords += 1;
            if is_real_valid_password(password) {
                part_2_passwords += 1;
            }
        }
    }
    assert_eq!(1929, part_1_passwords);
    assert_eq!(1306, part_2_passwords);
}
