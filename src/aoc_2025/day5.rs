const SAMPLE: &'static str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

const INPUT: &'static str = include_str!("./inputs/day5.txt");

#[derive(Clone)]
struct Interval {
    pub start: usize,
    pub end: usize,
}

impl Interval {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn grow_end(&mut self, other: &Self) -> bool {
        if self.end >= other.start {
            self.end = self.end.max(other.end);
            true
        } else {
            false
        }
    }

    fn size(&self) -> usize {
        self.end - self.start + 1
    }
}

struct Inventory {
    intervals: Vec<Interval>,
    ingredients: Vec<usize>,
}

fn parse_inventory(input: &str) -> Inventory {
    let separator_idx = input.find("\n\n").unwrap();
    let (id_ranges, ingredient_ids) = input.split_at(separator_idx);
    let intervals: Vec<Interval> = id_ranges
        .trim()
        .split("\n")
        .map(|range| {
            let dash_at = range.find("-").unwrap();
            let (start, end) = range.split_at(dash_at);
            Interval::new(start.parse().unwrap(), end[1..].parse().unwrap())
        })
        .collect();

    Inventory {
        intervals: merge_intervals(intervals),
        ingredients: ingredient_ids
            .trim()
            .split("\n")
            .map(|id| id.parse().unwrap())
            .collect(),
    }
}

fn merge_intervals(mut intervals: Vec<Interval>) -> Vec<Interval> {
    // Both parts require the intervals to be pre-sorted by the interval start value
    intervals.sort_by(|a, b| a.start.cmp(&b.start));
    let mut merged_intervals: Vec<Interval> = Vec::new();
    for interval in intervals {
        match merged_intervals.last_mut() {
            Some(last) => {
                if !last.grow_end(&interval) {
                    merged_intervals.push(interval);
                }
            }
            None => {
                merged_intervals.push(interval);
            }
        }
    }
    merged_intervals
}

fn count_fresh_ingredients(input: &str) -> usize {
    let Inventory {
        ingredients,
        intervals,
    } = parse_inventory(input);

    let mut count = 0;
    for ingredient in ingredients {
        for interval in &intervals {
            if ingredient < interval.start {
                break;
            } else if ingredient <= interval.end {
                count += 1;
                break;
            }
        }
    }

    count
}

fn count_total_fresh_ingredients(input: &str) -> usize {
    let Inventory { intervals, .. } = parse_inventory(input);
    intervals.iter().map(|interval| interval.size()).sum()
}

#[test]
fn part_1() {
    assert_eq!(3, count_fresh_ingredients(SAMPLE));
    assert_eq!(868, count_fresh_ingredients(INPUT));
}

#[test]
fn part_2() {
    assert_eq!(14, count_total_fresh_ingredients(SAMPLE));
    assert_eq!(354_143_734_113_772, count_total_fresh_ingredients(INPUT));
}
