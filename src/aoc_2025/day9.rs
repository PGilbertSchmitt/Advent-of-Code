use std::isize;

use logos::Logos;

const SAMPLE: &'static str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

const INPUT: &'static str = include_str!("./inputs/day9.txt");

#[derive(Debug)]
struct Tile(isize, isize);

#[derive(Logos, Debug)]
#[logos(skip "\n")]
enum Token {
    #[regex("[0-9]+,[0-9]+", |lex| {
        let mut values = lex.slice().split(",");
        Tile(values.next().unwrap().parse().unwrap(), values.next().unwrap().parse().unwrap())
    })]
    TileToken(Tile),
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    let tokens = Token::lexer(input);
    tokens
        .map(|t| {
            let Token::TileToken(tile) = t.unwrap();
            tile
        })
        .collect()
}

fn largest_rect(input: &str) -> isize {
    let tiles = parse_tiles(input);

    let mut largest_area = 0;
    for (idx, Tile(ax, ay)) in tiles.iter().enumerate() {
        for Tile(bx, by) in &tiles[idx + 1..] {
            largest_area = largest_area.max(((ax - bx).abs() + 1) * ((ay - by).abs() + 1));
        }
    }

    largest_area
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn is_clockwise(&self, prev: Self) -> bool {
        match self {
            Self::Up => prev == Self::Left,
            Self::Down => prev == Self::Right,
            Self::Left => prev == Self::Down,
            Self::Right => prev == Self::Up,
        }
    }

    fn transform(&self, clockwise: bool) -> Self {
        match self {
            Self::Up => {
                if clockwise {
                    Self::Right
                } else {
                    Self::Left
                }
            }
            Self::Down => {
                if clockwise {
                    Self::Left
                } else {
                    Self::Right
                }
            }
            Self::Left => {
                if clockwise {
                    Self::Up
                } else {
                    Self::Down
                }
            }
            Self::Right => {
                if clockwise {
                    Self::Down
                } else {
                    Self::Up
                }
            }
        }
    }
}

#[derive(Debug)]
struct Line<'a> {
    start: &'a Tile,
    end: &'a Tile,
    dir: Dir,
    outside: Dir,
}

impl<'a> Line<'a> {
    fn new(t1: &'a Tile, t2: &'a Tile) -> Self {
        let mut s = Self {
            start: t1,
            end: t2,
            // For now, these are dummy values
            dir: Dir::Down,
            outside: Dir::Down,
        };

        // These calculations consider pixels like a digital image where (0, 0) is the top left corner
        s.dir = if t1.0 == t2.0 {
            if t1.1 < t2.1 { Dir::Down } else { Dir::Up }
        } else {
            if t1.0 < t2.0 { Dir::Right } else { Dir::Left }
        };

        // The `outside` attribute is still nonsense for now, it will be calculated later
        s
    }

    fn min_scalar(&self) -> isize {
        match self.dir {
            Dir::Right => self.start.0,
            Dir::Down => self.start.1,
            Dir::Left => self.end.0,
            Dir::Up => self.end.1,
        }
    }

    fn max_scalar(&self) -> isize {
        match self.dir {
            Dir::Right => self.end.0,
            Dir::Down => self.end.1,
            Dir::Left => self.start.0,
            Dir::Up => self.start.1,
        }
    }

    fn is_vertical(&self) -> bool {
        match self.dir {
            Dir::Up | Dir::Down => true,
            _ => false,
        }
    }

    fn set_outside(&mut self, prev_line_dir: Dir, prev_line_outside: Dir) {
        self.outside = prev_line_outside.transform(self.dir.is_clockwise(prev_line_dir));
    }
}

fn to_lines<'a>(points: &'a Vec<Tile>) -> Vec<Line<'a>> {
    let mut lines = Vec::with_capacity(points.len());
    let first_tile = points.first().unwrap();
    let mut prev_tile = points.first().unwrap();
    for next_tile in &points[1..] {
        lines.push(Line::new(prev_tile, next_tile));
        prev_tile = next_tile;
    }
    lines.push(Line::new(prev_tile, first_tile));
    lines
}

#[derive(Debug)]
struct Rect {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Rect {
    fn new(&Tile(x1, y1): &Tile, &Tile(x2, y2): &Tile) -> Self {
        let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn area(&self) -> isize {
        (self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)
    }

    fn contains_lines(&self, verticals: &Vec<&Line>, horizontals: &Vec<&Line>) -> bool {
        for l in verticals {
            if self.contains_vertical_line(l) {
                return true;
            }
        }
        for l in horizontals {
            if self.contains_horizontal_line(l) {
                return true;
            }
        }
        false
    }

    fn contains_vertical_line(&self, line: &Line) -> bool {
        // First we reject any line segment that doesn't intersect with the x-axis of the rect
        // (but importantly, we allow inclusive intersections here, we do the outside check last)
        if line.start.0 < self.min_x || line.start.0 > self.max_x {
            return false;
        }

        // Now we check if the line segment's range intersects the y-axis of the rect exlusively
        if !exclusive_range_overlap(self.min_y, self.max_y, line.min_scalar(), line.max_scalar()) {
            return false;
        }

        // All we have left are lines which intersect the box but potentially touch the left or right edges,
        // so we do an "outside" check. If "outside" is to the right, then the line intersects as long as it's
        // not on the right edge. If "outside" is to the left, then the line intersects as long as it's not
        // on the left edge.
        (line.outside == Dir::Right && line.start.0 != self.max_x)
            || (line.outside == Dir::Left && line.start.0 != self.min_x)
        // Now figure out the rest...
    }

    // All the same as the `contains_vertical_line` jazz, but swapping the x and y axes
    fn contains_horizontal_line(&self, line: &Line) -> bool {
        if line.start.1 < self.min_y || line.start.1 > self.max_y {
            return false;
        }

        if !exclusive_range_overlap(self.min_x, self.max_x, line.min_scalar(), line.max_scalar()) {
            return false;
        }

        (line.outside == Dir::Down && line.start.1 != self.max_y)
            || (line.outside == Dir::Up && line.start.1 != self.min_y)
    }
}

fn exclusive_range_overlap(start_1: isize, end_1: isize, start_2: isize, end_2: isize) -> bool {
    start_1 < end_2 && start_2 < end_1
}

// I've made a few guarantees about the input.
// - Lines always switch between horizontal and vertical, which is nice
fn largest_internal_rect(input: &str) -> isize {
    let tiles = parse_tiles(input);
    let mut lines = to_lines(&tiles);
    let mut leftmost_line_x = isize::MAX;
    let mut leftmost_line_idx = 0; // Looking for the leftmost vertical line
    for (idx, line) in lines.iter().enumerate() {
        if line.is_vertical() {
            if line.start.0 < leftmost_line_x {
                leftmost_line_x = line.start.0;
                leftmost_line_idx = idx;
            }
        }
    }

    // The first step is to find one line that is guaranteed to be  the furthest in one dimension,
    // so that we can tell which side is outside
    let leftmost_line = lines.get_mut(leftmost_line_idx).unwrap();
    match leftmost_line.dir {
        Dir::Up => {
            leftmost_line.outside = Dir::Left;
        }
        Dir::Down => {
            leftmost_line.outside = Dir::Right;
        }
        _ => unreachable!(),
    }

    let mut prev_line_dir = leftmost_line.dir;
    let mut prev_line_outside = leftmost_line.outside;

    // All lines after the known line
    for next_line in &mut lines[leftmost_line_idx + 1..] {
        next_line.set_outside(prev_line_dir, prev_line_outside);
        prev_line_dir = next_line.dir;
        prev_line_outside = next_line.outside;
    }
    // All lines up to the known line
    for next_line in &mut lines[..leftmost_line_idx] {
        next_line.set_outside(prev_line_dir, prev_line_outside);
        prev_line_dir = next_line.dir;
        prev_line_outside = next_line.outside;
    }

    let mut horizontal_lines = Vec::new();
    let mut vertical_lines = Vec::new();
    for line in &lines {
        if line.is_vertical() {
            vertical_lines.push(line);
        } else {
            horizontal_lines.push(line);
        }
    }

    vertical_lines.sort_by(|l1, l2| l1.start.0.cmp(&l2.start.0));
    horizontal_lines.sort_by(|l1, l2| l1.start.1.cmp(&l2.start.1));

    let mut largest_area = 0;
    for (idx, t1) in tiles.iter().enumerate() {
        for t2 in &tiles[idx + 1..] {
            let rect = Rect::new(t1, t2);
            let area = rect.area();
            if area > largest_area {
                if !rect.contains_lines(&vertical_lines, &horizontal_lines) {
                    largest_area = area;
                }
            }
        }
    }

    largest_area
}

#[test]
fn part1() {
    assert_eq!(50, largest_rect(SAMPLE));
    assert_eq!(4_750_092_396, largest_rect(INPUT));
}

#[test]
fn part2() {
    assert_eq!(24, largest_internal_rect(SAMPLE));
    assert_eq!(1_468_516_555, largest_internal_rect(INPUT));
}
