use std::{collections::VecDeque, u32, u64};
use itertools::Itertools;

use fxhash::{FxHashMap, FxHashSet};

const SAMPLE: &str = "
###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

const INPUT: &str = include_str!("./inputs/day24.txt");

#[derive(Debug)]
enum Space {
    Wall,
    Empty,
    Interface(u32),
}

type Grid = Vec<Vec<Space>>;
type Coord = (usize /*X*/, usize /*Y*/);
type Interface = (u32 /*Value*/, Coord);
type Interfaces = Vec<Interface>;

fn parse_grid(input: &str) -> (Grid, Interfaces) {
    let mut interfaces = Vec::new();
    let grid = input
        .trim()
        .split("\n")
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == '#' {
                        Space::Wall
                    } else if ch == '.' {
                        Space::Empty
                    } else {
                        let interface = ch.to_digit(10).unwrap();
                        interfaces.push((interface, (x, y)));
                        Space::Interface(interface)
                    }
                })
                .collect()
        })
        .collect();

    (grid, interfaces)
}

fn neighbors(&(x, y): &Coord) -> [Coord; 4] {
    // Since walls cover the edges, this never underflow
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn find_connections(grid: &Grid, start_interface: &Interface, connections: &mut FxHashMap<(u32, u32), u64>) {
    let &(start, start_coord) = start_interface;
    let mut visited = FxHashSet::<Coord>::default();
    let mut queue = VecDeque::<(u64, Coord)>::new();

    visited.insert(start_coord);
    for n in neighbors(&start_coord) {
        queue.push_back((1, n));
    }

    while let Some((steps, cur)) = queue.pop_front() {
        if !visited.contains(&cur) {
            let (x, y) = cur;
            visited.insert((x, y));
            // These unwraps are safe because the edges are impassable walls
            let space = grid.get(y).unwrap().get(x).unwrap();
            match space {
                Space::Wall => {} // no-op
                Space::Empty => {
                    for n in neighbors(&cur) {
                        queue.push_back((steps + 1, n));
                    }
                }
                Space::Interface(to) => {
                    let to = *to;
                    if !connections.contains_key(&(start, to)) && !connections.contains_key(&(to, start)) {
                        connections.insert((start, to), steps);
                        connections.insert((to, start), steps);
                    }
                    for n in neighbors(&cur) {
                        queue.push_back((steps + 1, n));
                    }
                }
            }
        }
    }
}

// Traveling salesman is a lot easier when there are only 7 nodes to worry about
fn find_shortest_path(connections: &FxHashMap<(u32, u32), u64>, interfaces: &Interfaces) -> (u64, u64) {
    let (start, rest): (Vec<u32>, Vec<u32>) = interfaces.iter().map(|(x, _)| *x).partition(|a| *a == 0);
    let start = *start.first().unwrap();
    let mut shortest_path = u64::MAX;
    let mut shortest_path_with_reset = u64::MAX;
    for path in rest.iter().permutations(rest.len()) {
        let mut path_len = 0;
        let mut from = start;
        for next in path {
            path_len += connections.get(&(from, *next)).unwrap();
            from = *next;
        }
        shortest_path = shortest_path.min(path_len);

        path_len += connections.get(&(from, start)).unwrap();
        shortest_path_with_reset = shortest_path_with_reset.min(path_len);
    }

    (shortest_path, shortest_path_with_reset)
}

#[test]
fn samples() {
    let (grid, interfaces) = parse_grid(SAMPLE);
    let mut connections = FxHashMap::default();
    for interface in &interfaces[1..] {
        find_connections(&grid, interface, &mut connections);
    }
    assert_eq!((14, 20), find_shortest_path(&connections, &interfaces));
}

#[test]
fn solutions() {
    let (grid, interfaces) = parse_grid(INPUT);
    let mut connections = FxHashMap::default();
    for interface in &interfaces[1..] {
        find_connections(&grid, interface, &mut connections);
    }
    assert_eq!((498, 804), find_shortest_path(&connections, &interfaces));
}
