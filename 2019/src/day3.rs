use crate::loader::read_priv;

#[derive(PartialEq, Clone, Debug)]
enum Axis { X, Y }

#[derive(PartialEq, Clone, Debug)]
enum Direction { Left, Right, Up, Down }

#[derive(Clone, Debug)]
struct Point(i32, i32);

#[derive(Clone, Debug)]
struct Step(Direction, i32);

#[derive(Clone, Debug)]
struct Segment {
    axis: Axis,
    at: i32,
    from: i32,
    to: i32
}

pub fn part1() -> i32 {
    let wires: Vec<Vec<Segment>> = input_wires().iter().map(segments).collect();
    intersections(&wires[0], &wires[1]).iter().map(manhatten).min().unwrap_or(0)
}

fn intersections(wire1: &Vec<Segment>, wire2: &Vec<Segment>) -> Vec<Point> {
    wire1.iter()
         .flat_map(|seg1| intersect_with(wire2, seg1))
         .filter(|opt| opt.is_some())
         .map(|opt| opt.unwrap())
         .collect()
}

fn intersect_with(wire: &Vec<Segment>, seg: &Segment) -> Vec<Option<Point>> {
    wire.iter().map(|seg2| intersection(seg, seg2)).collect()
}

fn intersection(seg1: &Segment, seg2: &Segment) -> Option<Point>{
    if seg1.axis != seg2.axis && (seg1.from..seg1.to).contains(&seg2.at) && (seg2.from..seg2.to).contains(&seg1.at) {
        let point: Point = if seg1.axis == Axis::X { Point(seg1.at, seg2.at) } else { Point(seg2.at, seg1.at) };
        Some(point)
    } else {
        None
    }
}

fn manhatten(p: &Point) -> i32 {
    p.0.abs() + p.1.abs()
}

fn segments(wire: &Vec<Step>) -> Vec<Segment> {
    let mut segments: Vec<Segment> = vec![];
    let mut current = Point(0, 0);

    for i in 0..wire.len() {
        let (end_point, segment) = to_segment(current, &wire[i]);
        segments.push(segment);
        current = end_point;
    }

    segments
}

fn to_segment(start: Point, step: &Step)  -> (Point, Segment){
    let start_point = start.clone();
    let end_point = mv(start, step);
    let (axis, at, from, to) = if step.0 == Direction::Left || step.0 == Direction::Right {
        (Axis::X, start_point.1, start_point.0, end_point.0)
    } else {
        (Axis::Y, start_point.0, start_point.1, end_point.1)
    };

    let segment = Segment {
        axis: axis,
        at: at,
        from: from,
        to: to
    };

    (end_point, segment)
}

fn mv(start: Point, step: &Step) -> Point {
    match step.0 {
        Direction::Left => Point(start.0 - step.1, start.1),
        Direction::Right => Point(start.0 + step.1, start.1),
        Direction::Up => Point(start.0, start.1 + step.1),
        Direction::Down => Point(start.0, start.1 - step.1),
    }
}

fn input_wires() -> Vec<Vec<Step>> {
    read_priv(module_path!()).trim_end().split("\n").map(parse_wire).collect()
}

fn parse_wire(ws: &str) -> Vec<Step> {
    ws.split(",").map(|w| w.to_string()).map(parse_step).collect()
}

fn parse_step(step: String) -> Step {
    let distance = &step[1..].parse::<i32>().unwrap();
    Step(to_direction(&step[0..1]), *distance)
}

fn to_direction(d: &str) -> Direction {
    match d {
        "L" => Direction::Left,
        "R" => Direction::Right,
        "U" => Direction::Up,
        _ => Direction::Down
    }
}

