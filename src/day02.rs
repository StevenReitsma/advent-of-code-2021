use std::error::Error;
use std::fs;
use std::iter::FromIterator;
use std::str::FromStr;

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

pub struct Command {
    Direction: Direction,
    Steps: usize,
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let splits = string.split(" ").collect::<Vec<&str>>();

        Ok(Command {
            Direction: splits[0].parse().unwrap(),
            Steps: splits[1].parse().unwrap(),
        })
    }
}

pub fn get_input() -> Result<Vec<Command>, Box<dyn Error>> {
    let input = fs::read_to_string("input/day02.txt").expect("couldn't read input file");
    return Result::from_iter(input.lines().map(|x| x.parse()));
}

pub fn compute_a(input: Vec<Command>) -> usize {
    let mut x = 0;
    let mut y = 0;

    for cmd in input {
        match cmd.Direction {
            Direction::Forward => x += cmd.Steps,
            Direction::Up => y -= cmd.Steps,
            Direction::Down => y += cmd.Steps,
        }
    }

    return x * y;
}

pub fn compute_b(input: Vec<Command>) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for cmd in input {
        match cmd.Direction {
            Direction::Forward => {
                x += cmd.Steps;
                y += aim * cmd.Steps;
            }
            Direction::Up => aim -= cmd.Steps,
            Direction::Down => aim += cmd.Steps,
        }
    }

    return x * y;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(get_input().unwrap());
        assert_eq!(result, 1855814);
    }

    #[test]
    fn example_b() {
        let result = compute_b(get_input().unwrap());
        assert_eq!(result, 1845455714);
    }
}
