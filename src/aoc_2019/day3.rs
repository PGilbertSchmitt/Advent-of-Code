use logos::Logos;

const SAMPLE_1: &str = "
R8,U5,L5,D3
U7,R6,D4,L4";

const SAMPLE_2: &str = "
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

const SAMPLE_3: &str = "
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

const INPUT: &str = include_str!("./inputs/day3.txt");

#[derive(Logos)]
#[logos(skip r"[,]")]
enum Token {
    #[token("\n")]
    Newline,

    #[regex("[RLUD][0-9]+", |lex| {
        let slice = lex.slice();
        let value: i64 = slice[1..].parse().unwrap();
        match slice.chars().nth(0).unwrap() {
            'R' => Dir::Right(value),
            'L' => Dir::Left(value),
            'U' => Dir::Up(value),
            'D' => Dir::Down(value),
            _ => unreachable!()
        }
    })]
    Wire(Dir),
}

#[derive(Debug)]
enum Dir {
    Right(i64),
    Left(i64),
    Up(i64),
    Down(i64),
}

impl Dir {
    fn is_vertical(&self) -> bool {
        match self {
            Self::Up(_) | Self::Down(_) => true,
            _ => false,
        }
    }
}

fn parse_wires(input: &str) -> ((Vec<Line>, Vec<Line>), (Vec<Line>, Vec<Line>)) {
    let mut first_wire = Vec::new();
    let mut second_wire = Vec::new();
    let mut on_first = true;
    let mut tokens = Token::lexer(input.trim());
    while let Some(Ok(token)) = tokens.next() {
        match token {
            Token::Newline => {
                on_first = false;
            }
            Token::Wire(wire) => {
                if on_first {
                    first_wire.push(wire);
                } else {
                    second_wire.push(wire);
                }
            }
        }
    }
    (extract_lines(first_wire), extract_lines(second_wire))
}

type Coord = (i64, i64);

#[derive(Debug)]
struct Line {
    start: Coord,
    total_len: i64,

    // Which axes these are depends on the orientation
    min: i64,
    max: i64,
}

impl Line {
    // Only works if self is vertical and other is horizontal
    fn intersects(&self, other: &Line) -> Option<Coord> {
        if self.start.0 >= other.min
            && self.start.0 <= other.max
            && other.start.1 >= self.min
            && other.start.1 <= self.max
        {
            Some((self.start.0, other.start.1))
        } else {
            None
        }
    }
}

fn get_next_coord(&(x, y): &Coord, dir: Dir) -> Coord {
    match dir {
        Dir::Right(dx) => (x + dx, y),
        Dir::Left(dx) => (x - dx, y),
        Dir::Up(dy) => (x, y + dy),
        Dir::Down(dy) => (x, y - dy),
    }
}

fn extract_lines(schematics: Vec<Dir>) -> (Vec<Line>, Vec<Line>) {
    let mut vertical_lines = Vec::new();
    let mut horizontal_lines = Vec::new();
    let mut last_coord: Coord = (0, 0);
    let mut dist_so_far: i64 = 0;
    for dir in schematics {
        let is_vertical = dir.is_vertical();
        let next_coord = get_next_coord(&last_coord, dir);
        let min_x = last_coord.0.min(next_coord.0);
        let min_y = last_coord.1.min(next_coord.1);
        let max_x = last_coord.0.max(next_coord.0);
        let max_y = last_coord.1.max(next_coord.1);
        if is_vertical {
            vertical_lines.push(Line {
                start: last_coord,
                total_len: dist_so_far,
                min: min_y,
                max: max_y,
            });
            dist_so_far += (last_coord.1 - next_coord.1).abs();
        } else {
            horizontal_lines.push(Line {
                start: last_coord,
                total_len: dist_so_far,
                min: min_x,
                max: max_x,
            });
            dist_so_far += (last_coord.0 - next_coord.0).abs();
        }

        last_coord = next_coord;
    }
    (vertical_lines, horizontal_lines)
}

fn intersections(verticals: Vec<Line>, horizontals: Vec<Line>) -> (i64, i64) {
    let mut closest_intersection = i64::MAX;
    let mut intersection_with_least_delay = i64::MAX;
    for v in &verticals {
        for h in &horizontals {
            if let Some((x, y)) = v.intersects(h) {
                if x != 0 || y != 0 {
                    closest_intersection = closest_intersection.min(x.abs() + y.abs());
                    let delay =
                        v.total_len + h.total_len + (v.start.1 - y).abs() + (h.start.0 - x).abs();
                    intersection_with_least_delay = intersection_with_least_delay.min(delay);
                }
            }
        }
    }
    (closest_intersection, intersection_with_least_delay)
}

#[test]
fn samples() {
    let ((a_v, a_h), (b_v, b_h)) = parse_wires(SAMPLE_1);
    let (close_x, delay_x) = intersections(a_v, b_h);
    let (close_y, delay_y) = intersections(b_v, a_h);
    assert_eq!(6, close_x.min(close_y));
    assert_eq!(30, delay_x.min(delay_y));

    let ((a_v, a_h), (b_v, b_h)) = parse_wires(SAMPLE_2);
    let (close_x, delay_x) = intersections(a_v, b_h);
    let (close_y, delay_y) = intersections(b_v, a_h);
    assert_eq!(159, close_x.min(close_y));
    assert_eq!(610, delay_x.min(delay_y));

    let ((a_v, a_h), (b_v, b_h)) = parse_wires(SAMPLE_3);
    let (close_x, delay_x) = intersections(a_v, b_h);
    let (close_y, delay_y) = intersections(b_v, a_h);
    assert_eq!(135, close_x.min(close_y));
    assert_eq!(410, delay_x.min(delay_y));
}

#[test]
fn solutions() {
    let ((a_v, a_h), (b_v, b_h)) = parse_wires(INPUT);
    let (close_x, delay_x) = intersections(a_v, b_h);
    let (close_y, delay_y) = intersections(b_v, a_h);
    assert_eq!(865, close_x.min(close_y));
    assert_eq!(35038, delay_x.min(delay_y));
}
