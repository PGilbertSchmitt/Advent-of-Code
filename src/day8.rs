use std::{cell::RefCell, rc::Rc};

use fxhash::{FxHashMap, FxHashSet};

const SAMPLE: &'static str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

const INPUT: &'static str = include_str!("./inputs/day8.txt");

#[derive(Clone, Debug)]
struct Coor3D(i64, i64, i64);

impl Coor3D {
    fn abs_dist_from(&self, other: &Self) -> i64 {
        // We don't have to square root since we only care about the relative magnitude, and whether they're rooted or not doesn't change that fact
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

#[derive(Debug)]
struct Point {
    connections: FxHashSet<usize>,
}

fn parse_junctions(input: &str) -> Vec<Coor3D> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut parts = line.split(",");
            Coor3D(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn circuit_size(point: usize, flooded: &mut FxHashSet<usize>, circuits: &mut FxHashMap<usize, Rc<RefCell<Point>>>) -> usize {
    if flooded.contains(&point) {
        return 0;
    }
    flooded.insert(point);
    let point_obj = circuits.get(&point).unwrap().clone();
    let others = &point_obj.borrow().connections;
    let mut sum = 0;
    for other in others {
        sum += circuit_size(*other, flooded, circuits);
    }
    sum + 1
}

fn connect_circuits(input: &str, connections: usize) -> (usize, i64) {
    let junctions = parse_junctions(input);
    let num_points = junctions.len();
    let mut abs_pairs: Vec<(i64, usize, usize)> = Vec::with_capacity(num_points * num_points / 2);
    for first_idx in 0..num_points {
        let first = junctions.get(first_idx).unwrap();
        for second_idx in first_idx + 1..num_points {
            let second = junctions.get(second_idx).unwrap();
            abs_pairs.push((first.abs_dist_from(&second), first_idx, second_idx));
        }
    }
    abs_pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut circuit_tracker = FxHashMap::<usize, Rc<RefCell<Point>>>::default();
    let mut part1 = 0;
    for (iteration, (_, a_idx, b_idx)) in abs_pairs.iter().enumerate() {
        if iteration == connections {
            let mut flooded = FxHashSet::<usize>::default();
            let mut circuit_sizes = Vec::new();

            let points: Vec<usize> = circuit_tracker.keys().cloned().collect();
            for point in points {
                circuit_sizes.push(circuit_size(point, &mut flooded, &mut circuit_tracker));
            }
            circuit_sizes.sort_by(|a, b| b.cmp(a)); // Reverse sort

            part1 = circuit_sizes[0..3].iter().product()
        }

        let point_a = circuit_tracker.entry(*a_idx).or_insert_with(|| Rc::new(RefCell::new(Point { connections: FxHashSet::default() }))).clone();
        let point_b = circuit_tracker.entry(*b_idx).or_insert_with(|| Rc::new(RefCell::new(Point { connections: FxHashSet::default() })));
        point_a.borrow_mut().connections.insert(*b_idx);
        point_b.borrow_mut().connections.insert(*a_idx);

        // Once the circuit tracker has added each point at least once, we can start looking for the full connection
        if circuit_tracker.len() == num_points {
            let (first_point, _) = circuit_tracker.iter().next().unwrap();
            if num_points == circuit_size(*first_point, &mut FxHashSet::<usize>::default(), &mut circuit_tracker) {
                // Connected all sub-graphs
                let part2 = junctions.get(*a_idx).unwrap().0 * junctions.get(*b_idx).unwrap().0;
                return (part1, part2)
            }
        }
    };

    (0, 0)
}

#[test]
fn part1() {
    let (three_largest_sample, first_total_connection_sample) = connect_circuits(SAMPLE, 10);
    assert_eq!(40, three_largest_sample);
    assert_eq!(25272, first_total_connection_sample);

    let (three_largest_input, first_total_connection_input) = connect_circuits(INPUT, 1000);
    assert_eq!(83520, three_largest_input);
    assert_eq!(1131823407, first_total_connection_input);
}
