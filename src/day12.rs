use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::fs;

pub fn get_input() -> HashMap<String, Vec<String>> {
    let input = fs::read_to_string("input/day12.txt").expect("couldn't read input file");
    let mut hm = HashMap::new();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once("-").unwrap();
        hm.entry(a.to_string())
            .or_insert(Vec::new())
            .push(b.to_string());
        hm.entry(b.to_string())
            .or_insert(Vec::new())
            .push(a.to_string());
    });

    return hm;
}

pub fn compute_a(
    input: &HashMap<String, Vec<String>>,
    current: &String,
    visited: &Vec<String>,
    duplicate: &String,
    part_b: bool,
) -> usize {
    if current == "end" {
        return 1;
    }

    let mut new_duplicate = duplicate.clone();

    if current.to_lowercase() == *current && visited.contains(current) {
        if part_b && duplicate == "" && current != "start" && current != "end" {
            new_duplicate = current.clone();
        } else {
            return 0;
        }
    }

    let mut visited_new = visited.clone();
    visited_new.push(current.to_string());

    let neighbors = input[current].clone();
    return neighbors
        .iter()
        .map(|n| compute_a(input, n, &visited_new, &new_duplicate, part_b))
        .sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(
            &get_input(),
            &"start".to_string(),
            &Vec::new(),
            &"".to_string(),
            false,
        );
        assert_eq!(result, 4304);
    }

    #[test]
    fn example_b() {
        let result = compute_a(
            &get_input(),
            &"start".to_string(),
            &Vec::new(),
            &"".to_string(),
            true,
        );
        assert_eq!(result, 118242);
    }
}
