use std::fs;
use std::iter::FromIterator;

pub fn get_input() -> Result<Vec<isize>, std::num::ParseIntError> {
    let input = fs::read_to_string("input/day01.txt").expect("couldn't read input file");
    return Result::from_iter(input.lines().map(|x| x.parse()));
}

pub fn compute_a(input: Vec<isize>) -> usize {
    return input
        .iter()
        .zip(input.iter().skip(1))
        .fold(0, |acc, (a, b)| acc + (b > a) as usize);
}

pub fn compute_b(input: Vec<isize>) -> usize {
    let running_averages = input.windows(3).map(|x| x.iter().sum()).collect::<Vec<_>>();
    return compute_a(running_averages);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(get_input().unwrap());
        assert_eq!(result, 1195);
    }

    #[test]
    fn example_b() {
        let result = compute_b(get_input().unwrap());
        assert_eq!(result, 1235);
    }
}
