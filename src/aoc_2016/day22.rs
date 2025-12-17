use fxhash::FxHashMap;
use logos::Logos;

const INPUT: &str = include_str!("./inputs/day22.txt");

fn parse_coord(s: &str) -> u32 {
    s[2..].parse::<u32>().unwrap()
}

fn remove_t(s: &str) -> u32 {
    s[0..s.len() - 1].parse::<u32>().unwrap()
}

#[derive(Logos)]
#[logos(
    skip r"[\n ]",
    skip "root@ebhq-gridcenter#",
    skip "df",
    skip "-h",
    skip "Filesystem",
    skip "Size",
    skip "Used",
    skip "Avail",
    skip "Use%",
    skip "/dev/grid/node",
    skip r"[0-9]*%", // We don't need the Use% value to calculate the storage values
)]
enum Token {
    #[regex(r"\-x[0-9]+", |lex| parse_coord(lex.slice()))]
    X(u32),

    #[regex(r"\-y[0-9]+", |lex| parse_coord(lex.slice()))]
    Y(u32),

    #[regex("[0-9]+T", |lex| remove_t(lex.slice()))]
    Num(u32),
}

impl Token {
    fn value(self) -> u32 {
        match self {
            Token::X(x) => x,
            Token::Y(x) => x,
            Token::Num(x) => x,
        }
    }
}

type Key = (i32, i32);

#[derive(PartialEq, Eq)]
struct Node {
    used: u32,
    size: u32,
    avail: u32,
}

type Network = FxHashMap<Key, Node>;

fn parse_network(input: &str) -> Network {
    let mut grid: Network = FxHashMap::default();
    let mut tokens = Token::lexer(input);
    while let Some(Ok(token)) = tokens.next() {
        let x = token.value();
        let y = tokens.next().unwrap().unwrap().value();
        let size = tokens.next().unwrap().unwrap().value();
        let used = tokens.next().unwrap().unwrap().value();
        let avail = tokens.next().unwrap().unwrap().value();
        grid.insert((x as i32, y as i32), Node { used, size, avail });
    }
    grid
}

fn count_viable_pairs(grid: &Network) -> u32 {
    let non_empty_nodes: Vec<(&Key, &Node)> = grid
        .iter()
        .filter_map(|(k, v)| if v.used > 0 { Some((k, v)) } else { None })
        .collect();

    let mut count = 0;
    for (non_empty_key, non_empty_node) in non_empty_nodes {
        for (other_key, other_node) in grid.iter() {
            if non_empty_key != other_key && non_empty_node.used <= other_node.avail {
                count += 1;
            }
        }
    }

    count
}

// This works because of the crafted input. After printing the grid, I could identify several
// important features Every node falls into one of three camps:
// - Regular node, or "space", which has between 80T and 100T of total size, but is filled with between
// 60T and 80T of data. No
// - Nearly full giant node, or "wall", which will have at most 16T of space available out of over 500.
// These can not be moved anywhere
// - Empty node, or "hole", of which there is only one, which is big enough to hold the data of any regular
// node but not a giant node
// It's obvious that the only way to get data from one node to another is to move it from a space node
// to the one hole. This causes the hold to shift, sliding the data from the original node. Since every
// space node can fit data into any other regular node (provided that it is first emptied by the hole),
// the solution is to find the number of moves to get the space to x0-y(max-1), then it's a straight shot
// to move the data from x0-y(max) to x0-y0. That's just the manhattan distance between the hole and
// x0-y(max-1), then the hold is cyclically moved to transfer the data upwards. Each data move takes 5
// moves of the hole to bring the data to x0-y1 with the hold on x0-y0, then it's one last move to finish.
fn shortest_number_of_moves(grid: &Network) -> i32 {
    let (key, _) = grid.iter().find(|(_, v)| v.used == 0).unwrap();
    let max_x = grid.iter().fold(0, |max, ((x, _), _)| max.max(*x));
    let min_wall_x = grid
        .iter()
        .filter(|(_, v)| v.size > 200)
        .fold(i32::MAX, |min, (k, _)| min.min(k.0));
    let man_dist_around_wall = key.1 + (max_x - key.0).abs() + ((key.0 - min_wall_x + 1) * 2);

    println!(
        "{} + {} + {} = {}",
        key.1,
        (max_x - key.0).abs(),
        ((key.0 - min_wall_x + 1) * 2),
        key.1 + (max_x - key.0).abs() + ((key.0 - min_wall_x + 1) * 2)
    );

    // subtract 1 from max since by moving it the `man_dist_around_wall` amound, we would have pushed the
    // desired data into the next space with the hold on the opposite end as the goal. That means we only
    // have (max_x - 1) rotation moves, which takes 5 moves to perform
    man_dist_around_wall + ((max_x - 1) * 5)
}

#[test]
fn part_1() {
    let grid = parse_network(INPUT);

    // I've already guaranteed that all nodes are connected to a single network,
    // rather than there being multiple disconnected networks.
    assert_eq!(946, count_viable_pairs(&grid));
    assert_eq!(195, shortest_number_of_moves(&grid));

    // print_grid(&grid);
}

// This print function was used to understand the input
// - __ is an empty space
// - [] is a fillable space
// - ## is a wall
// - !! is the goal
// - XX is the data we want
// fn print_grid(grid: &Network) {
//     for y in 0..30 {
//         let mut line = Vec::<String>::new();
//         for x in 0..32 {
//             if y == 0 && x == 0 {
//                 line.push(String::from("!!"));
//             } else if y == 0 && x == 31 {
//                 line.push(String::from("XX"));
//             } else {
//                 match grid.get(&(x, y)) {
//                     None => line.push(String::from("     ")),
//                     Some(node) => {
//                         line.push(String::from(if node.used == 0 {
//                             "__"
//                         } else if node.used > 300 {
//                             "##"
//                         } else {
//                             "[]"
//                         }))
//                     }
//                 }
//             }
//         }
//         println!("{}", line.join(" "));
//     };
// }
