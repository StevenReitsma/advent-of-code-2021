use std::fs;
use std::iter::FromIterator;

pub fn get_input() -> Result<Vec<isize>, std::num::ParseIntError> {
    let input = fs::read_to_string("input/day1.txt").expect("couldn't read input file");
    return Result::from_iter(input.lines().map(|x| x.parse()));
}

pub fn compute(input: Vec<isize>) -> usize {
    return input
        .iter()
        .zip(input[1..].iter())
        .fold(0, |acc, (a, b)| acc + (b > a) as usize);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute(get_input().unwrap());
        assert_eq!(result, 1195);
    }

    // #[test]
    // fn example_b() {
    //     let result = compute(get_input().unwrap());
    //     assert_eq!(result, 276650720);
    // }
}
