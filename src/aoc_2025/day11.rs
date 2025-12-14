use fxhash::FxHashMap;
use logos::Logos;

const SAMPLE: &'static str = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

const SAMPLE_2: &'static str = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

const INPUT: &'static str = include_str!("./inputs/day11.txt");

#[derive(Logos)]
#[logos(skip r"[ \n]")]
enum Token {
    #[regex("[a-z]{3}:", |lex| lex.slice()[0..3].to_owned())]
    Input(String),

    #[regex("[a-z]{3}", |lex| lex.slice().to_owned())]
    Output(String),
}

type Network = FxHashMap<String, Vec<String>>;

// This could be faster if I convert every key into a number by inserting into a hash
// with an incrementing counter as a value. Then I would just need a Vec<Vec<u64>>
// to record the graph structure for lightning-fast lookups, but that's premature
// optimization, and we don't need that for a graph containing only ~600 nodes and
// a few thousand connections (or maybe...)
fn parse_network(input: &str) -> Network {
    let mut network = FxHashMap::default();

    let mut cur_input_node = String::from("");
    let mut cur_outputs = Vec::new();
    for token in Token::lexer(input) {
        match token.unwrap() {
            Token::Input(val) => {
                if !cur_outputs.is_empty() {
                    network.insert(cur_input_node, cur_outputs);
                }
                cur_outputs = Vec::new();
                cur_input_node = val;
            }
            Token::Output(val) => {
                cur_outputs.push(val);
            }
        }
    }
    // the last node
    network.insert(cur_input_node, cur_outputs);

    // Since "out" is a terminator, we can give it an empty output to make iteration easier
    // network.insert(String::from("out"), Vec::new());

    network
}

// This works as long as the network is not cyclical and that "out" is the only terminal, both of which I've verified
fn count_unique_paths(
    node: &String,
    end: &String,
    network: &Network,
    visited: &mut FxHashMap<String, usize>,
) -> usize {
    if let Some(paths) = visited.get(node) {
        return *paths;
    }

    if node == end {
        return 1;
    }

    let paths = match network.get(node) {
        None => 0,
        Some(outputs) => {
            let mut sum = 0;
            for output in outputs {
                let uniq_paths = count_unique_paths(output, end, network, visited);
                sum += uniq_paths
            }
            sum
        }
    };

    visited.insert(node.clone(), paths);

    paths
}

fn count_problem_paths(network: &Network) -> usize {
    let svr = String::from("svr");
    let fft = String::from("fft");
    let dac = String::from("dac");
    let out = String::from("out");

    let fft_to_dac = count_unique_paths(&fft, &dac, &network, &mut FxHashMap::default());
    if fft_to_dac > 0 {
        // Path goes svr~>fft~>dac~>out
        let start_path = count_unique_paths(&svr, &fft, &network, &mut FxHashMap::default());
        let end_path = count_unique_paths(&dac, &out, &network, &mut FxHashMap::default());
        start_path * fft_to_dac * end_path
    } else {
        // Path goes svr~>dac~>fft~>out
        let start_path = count_unique_paths(&svr, &dac, &network, &mut FxHashMap::default());
        let dac_to_fft = count_unique_paths(&dac, &fft, &network, &mut FxHashMap::default());
        let end_path = count_unique_paths(&fft, &out, &network, &mut FxHashMap::default());
        start_path * dac_to_fft * end_path
    }
}

#[test]
fn part1() {
    let you = String::from("you");
    let out = String::from("out");
    let paths = count_unique_paths(
        &you,
        &out,
        &parse_network(SAMPLE),
        &mut FxHashMap::default(),
    );
    assert_eq!(5, paths);
    let paths = count_unique_paths(&you, &out, &parse_network(INPUT), &mut FxHashMap::default());
    assert_eq!(749, paths);
}

#[test]
fn part2() {
    assert_eq!(2, count_problem_paths(&parse_network(SAMPLE_2)));
    assert_eq!(
        420_257_875_695_750,
        count_problem_paths(&parse_network(INPUT))
    );
}
