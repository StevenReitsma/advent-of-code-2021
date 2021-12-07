use std::fs;
use stats::median;

pub fn get_input() -> Vec<i32> {
    let input = fs::read_to_string("input/day07.txt").expect("couldn't read input file");
    return input.split(",").map(|s| s.parse().unwrap()).collect();
}

pub fn compute(input: &Vec<i32>) -> i32 {
    let median = median(input.iter().map(|x| *x)).unwrap() as i32;
    return input.iter().map(|x| (x - median).abs()).sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result_naive = compute(&get_input());
        assert_eq!(result_naive, 355386);
    }

    #[test]
    fn example_b() {
        let result = compute(&get_input());
        assert_eq!(result, 1613415325);
    }
}
