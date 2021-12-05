use std::collections::HashMap;
use std::fs;

type Point = (i32, i32);
type Line = (Point, Point);

pub fn get_lines() -> Vec<Line> {
    let input = fs::read_to_string("input/day05.txt").expect("couldn't read input file");
    return input
        .lines()
        .map(|x| {
            let points = x.split(" -> ").collect::<Vec<&str>>();
            let a = points[0].split(",").collect::<Vec<&str>>();
            let b = points[1].split(",").collect::<Vec<&str>>();

            return (
                (a[0].parse().unwrap(), a[1].parse().unwrap()),
                (b[0].parse().unwrap(), b[1].parse().unwrap()),
            );
        })
        .collect();
}

pub fn is_straight(line: Line) -> bool {
    return line.0 .0 == line.1 .0 || line.0 .1 == line.1 .1;
}

pub fn points_in_line(line: Line) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    let mut x = line.0 .0;
    let mut y = line.0 .1;

    let direction = (
        (line.1 .0 - line.0 .0).clamp(-1, 1),
        (line.1 .1 - line.0 .1).clamp(-1, 1),
    );

    loop {
        points.push((x, y));

        if x == line.1 .0 && y == line.1 .1 {
            return points;
        }

        x += direction.0;
        y += direction.1;
    }
}

pub fn num_hits(map: HashMap<Point, u32>, hit_threshold: u32) -> usize {
    return map.iter().filter(|(k, v)| **v >= hit_threshold).count();
}

pub fn compute_a(lines: Vec<Line>, use_diagonal: bool) -> usize {
    let mut map: HashMap<Point, u32> = HashMap::new();

    for line in lines {
        if use_diagonal || is_straight(line) {
            let points_in_line = points_in_line(line);

            for point in points_in_line {
                *map.entry(point).or_insert(0) += 1;
            }
        }
    }

    return num_hits(map, 2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(get_lines(), false);
        assert_eq!(result, 3990);
    }

    #[test]
    fn example_b() {
        let result = compute_a(get_lines(), true);
        assert_eq!(result, 21305);
    }
}
