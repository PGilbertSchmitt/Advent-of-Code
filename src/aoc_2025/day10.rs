use itertools::Itertools;
use logos::Logos;

const SAMPLE: &'static str = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

const INPUT: &'static str = include_str!("./inputs/day10.txt");

fn peel<'a>(s: &'a str) -> &'a str {
    &s[1..s.len() - 1]
}

fn get_sequence(s: &str) -> Vec<u16> {
    peel(s)
        .split(',')
        .map(|s| s.parse::<u16>().unwrap())
        .collect()
}

#[derive(Logos, Debug)]
#[logos(skip r"[\n ]")]
enum Token {
    #[regex(r"\[[.#]+\]", |lex| {
        peel(lex.slice()).chars().map(|ch| ch == '#').collect::<Vec<bool>>()
    })]
    IndicatorLights(Vec<bool>),

    #[regex(r"\([0-9]+(,[0-9]+)*\)", |lex| get_sequence(lex.slice()))]
    Button(Vec<u16>),

    #[regex(r"\{[0-9]+(,[0-9]+)*\}", |lex| get_sequence(lex.slice()))]
    JoltageReq(Vec<u16>),
}

type Button = (u16, Vec<u16>);

#[derive(Debug)]
struct Diagram {
    indicators: u16,
    buttons: Vec<Button>,
    joltage_reqs: Vec<u16>,
}

impl Diagram {
    fn new() -> Self {
        Self {
            indicators: 0,
            buttons: Vec::new(),
            joltage_reqs: Vec::new(),
        }
    }
}

fn parse_diagrams(input: &str) -> Vec<Diagram> {
    let mut tokens = Token::lexer(input);

    let mut diagrams = Vec::new();
    let mut cur_diagram = Diagram::new();

    while let Some(Ok(token)) = tokens.next() {
        match token {
            Token::IndicatorLights(indicator) => {
                let indicator_value: u16 = indicator
                    .iter()
                    .enumerate()
                    .map(|(idx, on)| if *on { 1 << idx } else { 0 })
                    .sum();
                cur_diagram.indicators = indicator_value;
            }
            Token::Button(seq) => {
                let sequence_value = seq.iter().map(|button| 1 << button).sum();
                cur_diagram.buttons.push((sequence_value, seq));
            }
            Token::JoltageReq(seq) => {
                cur_diagram.joltage_reqs = seq;
                diagrams.push(cur_diagram);
                cur_diagram = Diagram::new();
            }
        }
    }

    diagrams
}

// Pressing a button twice will have the same effect as pressing it 0 times,
// therefore part 1 requires a trivial number of permutations (which no
// doubt will change for part 2).
fn fewest_button_presses_to_initialize(diagram: &Diagram) -> usize {
    // Thank you Itertools, I did NOT want to implement a powerset
    for set in diagram.buttons.iter().powerset() {
        if set
            .iter()
            .fold(0, |acc, &&(button_value, _)| acc ^ button_value)
            == diagram.indicators
        {
            return set.len();
        }
    }

    0
}

fn buttons_to_initialize_machines(input: &str) -> usize {
    let diagrams = parse_diagrams(input);
    diagrams
        .iter()
        .map(fewest_button_presses_to_initialize)
        .sum()
}

fn buttons_to_configure_machines(input: &str) -> usize {
    let diagrams = parse_diagrams(input);
    diagrams
        .iter()
        .map(|diagram| configure_joltages(&diagram.buttons, &diagram.joltage_reqs))
        .sum()
}

fn apply_joltages(buttons: &[&Button], cur_joltages: &[u16]) -> Option<Vec<u16>> {
    let mut new_joltages = Vec::from(cur_joltages);

    for (_, button) in buttons {
        for &button_part in button {
            if new_joltages[button_part as usize] == 0 {
                return None;
            }
            new_joltages[button_part as usize] -= 1;
        }
    }

    Some(new_joltages.into_iter().map(|x| x / 2).collect())
}

fn configure_joltages(buttons: &[Button], current_joltages: &[u16]) -> usize {
    if current_joltages.iter().all(|&j| j == 0) {
        return 0;
    }

    let mut odd_joltage_pattern = 0;
    for (idx, &j) in current_joltages.iter().enumerate() {
        if j % 2 == 1 {
            odd_joltage_pattern |= 1 << idx as u16;
        }
    }
    // Special case if next_buttons is still all even?

    let mut lowest = usize::MAX;
    for set in buttons.iter().powerset() {
        if set
            .iter()
            .fold(0, |acc, &(button_value, ..)| acc ^ button_value)
            == odd_joltage_pattern
        {
            if let Some(next_joltages) = apply_joltages(&set[..], current_joltages) {
                let sub_cost = configure_joltages(buttons, &next_joltages);
                if sub_cost != usize::MAX {
                    let solved = 2 * sub_cost + set.len();
                    lowest = lowest.min(solved);
                }
            }
        }
    }

    lowest
}

#[test]
fn part1() {
    assert_eq!(7, buttons_to_initialize_machines(SAMPLE));
    assert_eq!(457, buttons_to_initialize_machines(INPUT));
}

#[test]
fn part2() {
    assert_eq!(33, buttons_to_configure_machines(SAMPLE));
    // Takes 4 seconds on release mode, about a minute on debug mode
    // assert_eq!(17576, buttons_to_configure_machines(INPUT));
}
