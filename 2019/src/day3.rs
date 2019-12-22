use std::fs;
use std::collections::HashSet;
// use itertools::Itertools;

fn main() {
    let raw_contents = fs::read_to_string("./priv/2019/day3.in").expect("Error reading the file.");
    let contents: Vec<&str> = raw_contents.trim().split("\n").collect(); // get rid of trailing \n

    let w1: Path = parse_path(contents[0]);
    let w2: Path = parse_path(contents[1]);

    let intersections = find_intersections(w1.clone(), w2.clone());

    let min_distance = closest_intersection(intersections);
    println!("Day3 part1: {}", min_distance);
}


#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Point(isize, isize);

impl Point {
    fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

type Path = Vec<Point>;

#[derive(Debug, PartialEq)]
enum Direction { Up, Down, Left, Right, Invalid }

fn closest_intersection(path: Path) -> isize {
    path.iter().filter(|&p| !p.is_zero()).fold(1_000_000, |distance, &Point(x, y)| {
        let d = x.abs() + y.abs();
        if d < distance { d } else { distance }
    })
}

fn find_intersections(p1: Vec<Point>, p2: Vec<Point>) -> Vec<Point> {
    let h1: HashSet<Point> = p1.into_iter().collect();
    let h2: HashSet<Point> = p2.into_iter().collect();
    h1.intersection(&h2).cloned().collect()
}

fn parse_path(path_str: &str) -> Vec<Point> {
    path_str.split(",").fold(vec![Point(0, 0)], add_steps_to_path).clone()
}

// reducer
fn add_steps_to_path(path: Path, step: &str) -> Path {
    let (drct, n) = parse_step(step);
    let &Point(last_x, last_y) = path.last().unwrap();

    let mut new_steps: Path = match drct {
        Direction::Up => (last_y + 1..=last_y + n).map(|y| Point(last_x, y)).collect(),
        Direction::Down => (last_y - n..=last_y - 1).map(|y| Point(last_x, y)).rev().collect(),
        Direction::Right => (last_x + 1..=last_x + n).map(|x| Point(x, last_y)).collect(),
        Direction::Left => (last_x - n..=last_x - 1).map(|x| Point(x, last_y)).rev().collect(),
        Direction::Invalid => vec![],
    };

    let mut new_path = path.clone();
    new_path.append(&mut new_steps);
    new_path
}

fn parse_step(step: &str) -> (Direction, isize) {
    let mut chars = step.chars();

    let direction = chars.next();
    let n = chars.as_str().parse::<isize>();

    match (direction, n) {
        (Some('U'), Ok(x)) => (Direction::Up, x),
        (Some('D'), Ok(x)) => (Direction::Down, x),
        (Some('L'), Ok(x)) => (Direction::Left, x),
        (Some('R'), Ok(x)) => (Direction::Right, x),
        // Errors
        (_, _) => (Direction::Invalid, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_intersections_pass() {
        let p1 = vec![Point(0,0), Point(0,1), Point(1,1), Point(1,2)];
        let p2 = vec![Point(0,0), Point(1,0), Point(1,1), Point(2,1)];
        assert_eq!(find_intersections(p1, p2), vec![Point(0,0), Point(1,1)]);
    }

    #[test]
    fn parse_path_pass() {
        assert_eq!(
            parse_path("U3,L4,D4"),
            vec![Point(0, 0), Point(0, 1), Point(0, 2), Point(0, 3), Point(-1, 3), Point(-2, 3),
                 Point(-3, 3), Point(-4, 3), Point(-4, 2), Point(-4, 1), Point(-4, 0),
                 Point(-4, -1)]
        )
    }

    #[test]
    fn add_steps_to_path_pass() {
        assert_eq!(
            add_steps_to_path(vec![Point(0, 0), Point(0, -1)], "L2"),
            vec![Point(0, 0), Point(0, -1), Point(-1, -1), Point(-2, -1)]
        );
    }

    #[test]
    fn parse_step_pass() {
        assert_eq!(parse_step("U2"), (Direction::Up, 2));
        assert_eq!(parse_step("D10"), (Direction::Down, 10));
        assert_eq!(parse_step("R17"), (Direction::Right, 17));
        assert_eq!(parse_step("L999"), (Direction::Left, 999));
        assert_eq!(parse_step("2"), (Direction::Invalid, 0));
    }
}
