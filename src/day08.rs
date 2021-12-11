use stats::mean;
use stats::median;
use std::fs;

type Entry = (Vec<String>, Vec<String>);

pub fn get_input() -> Vec<Entry> {
    let input = fs::read_to_string("input/day08.txt").expect("couldn't read input file");
    return input
        .lines()
        .map(|x| {
            let (a, b) = x.split_once(" | ").unwrap();
            return (
                a.split(" ").map(|x| x.to_string()).collect(),
                b.split(" ").map(|x| x.to_string()).collect(),
            );
        })
        .collect();
}

pub fn compute_a(input: &Vec<Entry>) -> usize {
    return input
        .iter()
        .map(|(_, b)| {
            b.iter()
                .filter(|el| el.len() == 2 || el.len() == 3 || el.len() == 4 || el.len() == 7)
                .count()
        })
        .sum();
}

pub fn same_as_one(a: &Vec<String>, digit: &String) -> i32 {
    return a
        .iter()
        .filter(|el| el.len() == 2)
        .nth(0).unwrap().chars()
        .fold(0, |acc, d| if digit.contains(d) { acc + 1 } else { acc });
}

pub fn same_as_four(a: &Vec<String>, digit: &String) -> i32 {
    return a
        .iter()
        .filter(|el| el.len() == 4)
        .nth(0).unwrap().chars()
        .fold(0, |acc, d| if digit.contains(d) { acc + 1 } else { acc });
}

pub fn compute_b(input: &Vec<Entry>) -> usize {
    let mut total = 0;
    for (a, b) in input {
        let mut num = 0;
        let mut i = 1000;
        for digit in b {
            let n = match (digit.len(), same_as_one(a, digit), same_as_four(a, digit)) {
                (2, _, _) => 1,
                (3, _, _) => 7,
                (4, _, _) => 4,
                (5, 1, 2) => 2,
                (5, 1, 3) => 5,
                (5, 2, 3) => 3,
                (6, 1, 3) => 6,
                (6, 2, 3) => 0,
                (6, 2, 4) => 9,
                (7, _, _) => 8,
                _ => panic!(),
            };

            num += n * i;
            i /= 10;
        }

        total += num;
    }

    return total;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(&get_input());
        assert_eq!(result, 264);
    }

    #[test]
    fn example_b() {
        let result = compute_b(&get_input());
        assert_eq!(result, 1063760);
    }
}
