const SAMPLE: &'static str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

const INPUT: &'static str = include_str!("./inputs/day4.txt");

struct Coor(isize, isize);
struct PosCoor(usize, usize);
type Grid = Vec<Vec<bool>>;

fn parse_grid(input: &str) -> Grid {
    input
        .trim()
        .split("\n")
        .map(|row| row.chars().map(|ch| ch == '@').collect())
        .collect()
}

fn neighbors(&Coor(x, y): &Coor) -> Vec<Coor> {
    vec![
        Coor(x - 1, y),
        Coor(x + 1, y),
        Coor(x, y - 1),
        Coor(x, y + 1),
        Coor(x - 1, y - 1),
        Coor(x + 1, y - 1),
        Coor(x - 1, y + 1),
        Coor(x + 1, y + 1),
    ]
}

fn is_roll(grid: &Grid, &Coor(x, y): &Coor, &PosCoor(max_x, max_y): &PosCoor) -> bool {
    let x = usize::try_from(x);
    let y = usize::try_from(y);
    if let Ok(x) = x
        && let Ok(y) = y
    {
        if x > max_x || y > max_y {
            return false;
        }
        let row_opt = grid.get(y);
        let col_opt = row_opt.map(|row| row.get(x).unwrap_or(&false));
        return *col_opt.unwrap_or(&false);
    }

    false
}

fn count_liftables(input: &str) -> usize {
    let grid = parse_grid(input);
    let (_, removed) = clean_pass(grid);
    removed
}

fn clean_pass(grid: Grid) -> (Grid, usize) {
    let mut count = 0;
    let max = PosCoor(grid.get(0).unwrap().len(), grid.len());

    let new_grid: Grid = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, val)| {
                    if *val
                        && neighbors(&Coor(x as isize, y as isize))
                            .iter()
                            .filter(|neighbor| is_roll(&grid, neighbor, &max))
                            .count()
                            < 4
                    {
                        count += 1;
                        return false;
                    }
                    return *val;
                })
                .collect()
        })
        .collect();

    (new_grid, count)
}

fn remove_liftables(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let mut total_removed = 0;
    loop {
        let (new_grid, removed) = clean_pass(grid);
        grid = new_grid;
        if removed == 0 {
            break;
        }
        total_removed += removed;
    }

    total_removed
}

#[test]
fn part1() {
    assert_eq!(13, count_liftables(SAMPLE));
    assert_eq!(1344, count_liftables(INPUT));
}

#[test]
fn part2() {
    assert_eq!(43, remove_liftables(SAMPLE));
    assert_eq!(8112, remove_liftables(INPUT));
}
