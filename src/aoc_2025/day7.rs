use fxhash::{FxHashMap, FxHashSet};

const SAMPLE: &'static str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

const INPUT: &'static str = include_str!("./inputs/day7.txt");

type Pair = (isize, isize);

struct Manifold {
    start: Pair,
    max_row: isize,
    splitters: FxHashSet<Pair>,
}

enum BeamResult {
    None,
    Down(Pair),
    Split(Pair, Pair),
}

impl Manifold {
    fn move_beam(&self, beam: &Pair) -> BeamResult {
        let coor_down = (beam.0, beam.1 + 1);
        if coor_down.1 > self.max_row {
            // Reached the bottom of the manifold
            BeamResult::None
        } else if self.splitters.contains(&coor_down) {
            let coor_left = (coor_down.0 - 1, coor_down.1);
            let coor_right = (coor_down.0 + 1, coor_down.1);
            BeamResult::Split(coor_left, coor_right)
        } else {
            BeamResult::Down(coor_down)
        }
    }

    // fn within(&self, &(col, row): &Pair) -> bool {
    //     col >= 0 && col <= self.max_col && row >= 0 && row <= self.max_row
    // }
}

fn parse_manifold(input: &str) -> Manifold {
    let mut manifold = Manifold {
        start: (isize::MAX, isize::MAX),
        max_row: 0,
        splitters: FxHashSet::default(),
    };

    for (row_idx, row) in input.trim().split("\n").enumerate() {
        for (col_idx, char) in row.chars().enumerate() {
            let coor = (col_idx as isize, row_idx as isize);
            match char {
                'S' => {
                    manifold.start = coor;
                }
                '^' => {
                    manifold.splitters.insert(coor);
                }
                _ => {}
            }
        }
        manifold.max_row = row_idx as isize;
    }

    manifold
}

fn count_splits(input: &str) -> (usize, usize) {
    let manifold = parse_manifold(input);

    let mut classical_splits = 0;
    let mut multiversal_splits = 0;
    let mut beams = FxHashMap::<Pair, usize>::default();
    beams.insert(manifold.start, 1);
    while !beams.is_empty() {
        let mut next_beams = FxHashMap::default();

        for (beam, count) in beams {
            match manifold.move_beam(&beam) {
                BeamResult::Down(next_beam) => {
                    next_beams
                        .entry(next_beam)
                        .and_modify(|prev_count| *prev_count += count)
                        .or_insert(count);
                }
                BeamResult::Split(left_beam, right_beam) => {
                    next_beams
                        .entry(left_beam)
                        .and_modify(|prev_count| *prev_count += count)
                        .or_insert(count);
                    next_beams
                        .entry(right_beam)
                        .and_modify(|prev_count| *prev_count += count)
                        .or_insert(count);
                    classical_splits += 1;
                }
                BeamResult::None => {
                    multiversal_splits += count;
                }
            }
        }

        beams = next_beams;
    }

    (classical_splits, multiversal_splits)
}

#[test]
fn both_parts() {
    let (classical_splits, multiversal_splits) = count_splits(SAMPLE);
    assert_eq!(21, classical_splits);
    assert_eq!(40, multiversal_splits);

    let (classical_splits, multiversal_splits) = count_splits(INPUT);
    assert_eq!(1646, classical_splits);
    assert_eq!(32_451_134_474_991, multiversal_splits);
}
