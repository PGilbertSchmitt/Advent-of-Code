const INPUT: &'static str = include_str!("./inputs/day12.txt");

struct Present {
    size: usize,
}

struct Tree {
    width: usize,
    height: usize,
    requirements: [usize; 6],
}

fn parse_presents(input: &str) -> (Vec<Present>, Vec<Tree>) {
    let mut sections: Vec<&str> = input.trim().split("\n\n").collect();
    let trees = sections.pop().unwrap();

    let mut presents = Vec::new();

    for present in sections {
        presents.push(Present {
            size: present[1..]
                .chars()
                .fold(0, |acc, ch| if ch == '#' { acc + 1 } else { acc }),
        });
    }

    let trees: Vec<Tree> = trees
        .split('\n')
        .map(|line| {
            let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
            println!("Parts: {:?}", parts);
            let shape: Vec<usize> = parts[0]
                .split('x')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let presents: Vec<usize> = parts[1]
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            Tree {
                width: shape[0],
                height: shape[1],
                requirements: [
                    presents[0],
                    presents[1],
                    presents[2],
                    presents[3],
                    presents[4],
                    presents[5],
                ],
            }
        })
        .collect();

    (presents, trees)
}

// This day is deceptive. It looks insanely challenging, but actually, the really basic optimization is all you need:
// All days in theory should lie within 3 buckets:
// - Those which are immediately impossible (all presents would take up more space than there exists under the tree)
// - Those which are immediately possible (even the loosest packing would sufficiently hold all presents)
// - Those which are not immediately known and need to be calculated.
// It turns out the input was crafted such that all trees fit into one of the first 2 buckets, so this is solvable
// by only counting how many trees have less space than the sum total of spaces in the presents. No fitting, packing,
// no memoization. That's it. That's the whole thing.
#[test]
fn part1() {
    let (presents, trees) = parse_presents(INPUT);

    let valid_tree_configs = trees
        .iter()
        .filter(|tree| {
            let tree_size = tree.height * tree.width;
            let presents_size = tree
                .requirements
                .iter()
                .enumerate()
                .fold(0, |acc, (idx, req)| req * presents[idx].size + acc);
            tree_size >= presents_size
        })
        .count();

    assert_eq!(555, valid_tree_configs);
}
