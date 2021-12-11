use stats::mean;
use stats::median;
use std::fs;

pub fn get_input() -> Vec<i32> {
    let input = fs::read_to_string("input/day07.txt").expect("couldn't read input file");
    return input.split(",").map(|s| s.parse().unwrap()).collect();
}

pub fn compute_a(input: &Vec<i32>) -> i32 {
    // median is optimal solution for 1-norm distance
    let median = median(input.iter().map(|x| *x)).unwrap() as i32;
    return input.iter().map(|x| (x - median).abs()).sum();
}

pub fn compute_b(input: &Vec<i32>) -> i32 {
    // error is n*(n+1)/2. biggest term is n^2
    // mean solves optimal solution for 2-norm distance, so approximates error above
    let mean = mean(input.iter().map(|x| *x)) as i32;
    return input
        .iter()
        .map(|x| (x - mean).abs() * ((x - mean).abs() + 1) / 2)
        .sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(&get_input());
        assert_eq!(result, 337833);
    }

    #[test]
    fn example_b() {
        let result = compute_b(&get_input());
        assert_eq!(result, 96678050);
    }
}
