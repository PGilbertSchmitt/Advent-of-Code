use logos::Logos;
use itertools::Itertools;

const SAMPLE: &'static str = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

const INPUT: &'static str = include_str!("./inputs/day10.txt");

fn peel<'a>(s: &'a str) -> &'a str {
    &s[1..s.len()-1]
}

fn get_sequence(s: &str) -> Vec<u16> {
    peel(s).split(',').map(|s| s.parse::<u16>().unwrap()).collect()
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

#[derive(Debug)]
struct Diagram {
    indicators: u16,
    buttons: Vec<u16>,
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
                let indicator_value: u16 = indicator.iter().enumerate().map(|(idx, on)| {
                    if *on { 1 << idx } else { 0 }
                }).sum();
                cur_diagram.indicators = indicator_value;
            }
            Token::Button(seq) => {
                let sequence_value = seq.iter().map(|button| {
                    1 << button
                }).sum();
                cur_diagram.buttons.push(sequence_value);
            }
            Token::JoltageReq(seq) => {
                cur_diagram.joltage_reqs = seq;
                // Wait for part 2
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
        if set.iter().fold(0, |acc, button_value| acc ^ **button_value) == diagram.indicators {
            return set.len()
        }
    };

    0
}

#[derive(Default)]
struct Sequence {
    presses: usize,
    // cur_joltages: 
}

impl Sequence {
    fn expand(&self) -> Self {
        Self { presses: self.presses }
    }
}

// For part 2, pressing a button multiple times is now possible. However, I
// can still optimize: Button sequence A-A-B is equivalent to A-B-A and B-A-A.
// I can avoid these by batching each generated sequence with a min-idx into the
// button list. If button A is at idx 0 and a sequence is (A-B, 1), then it can't
// add more A buttons to itself for the next permutations.
fn fewest_button_presses_to_configure(diagram: &Diagram) -> usize {
    let mut sorted_joltages: Vec<(usize, u16)> = diagram.joltage_reqs.iter().enumerate().map(|(a, b)| (a, *b)).collect();
    sorted_joltages.sort_by(|a, b| a.1.cmp(&b.1));
    try_button_combos(Sequence::default(), &sorted_joltages, &diagram.buttons).unwrap()
}

fn try_button_combos(
    sequence: Sequence,
    remaining_joltages: &[(usize, u16)],
    buttons: &[u16],
) -> Option<usize> {
    let &(indicator_idx, joltage_req) = remaining_joltages.first().unwrap();
    let (applicable_buttons, remaining_buttons): (Vec<u16>, Vec<u16>) = buttons.iter().map(|x| *x).partition(|x| x & (1 << indicator_idx) > 0);

    let n = joltage_req as u128;
    let r = applicable_buttons.len() as u128;
    println!("Lowest Joltage req: {joltage_req} with {} buttons", applicable_buttons.len());
    // let num_combos = fact(n + r - 1) / (fact(r) * fact(n - 1));
    // println!("leads to {num_combos} combos");

    Some(0)
}

fn fact(x: u128) -> u128 {
    if x > 1 {
        x * fact(x - 1)
    } else {
        1
    }
}

fn buttons_to_configure_machines(input: &str) -> usize {
    let diagrams = parse_diagrams(input);
    diagrams.iter().map(fewest_button_presses_to_initialize).sum()
}

#[test]
fn part1() {
    assert_eq!(7, buttons_to_configure_machines(SAMPLE));
    assert_eq!(457, buttons_to_configure_machines(INPUT));
}

#[test]
fn part2() {
    let diagrams = parse_diagrams(INPUT);
    for d in diagrams.iter() {
        fewest_button_presses_to_configure(d);
    }
}
