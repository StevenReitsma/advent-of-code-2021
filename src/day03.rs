use std::fs;

pub fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input/day03.txt").expect("couldn't read input file");
    return input.lines().map(|s| s.to_string()).collect();
}

pub fn create_needle(input: &Vec<String>) -> String {
    let mut gamma: String = "".to_string();

    for c in 0..12 {
        let num_ones = input.iter().fold(0, |acc, e| {
            acc + e.chars().nth(c).unwrap().to_digit(10).unwrap()
        }) as usize;
        if num_ones as f32 >= (input.len() as f32 / 2.) {
            gamma.push_str("1");
        } else {
            gamma.push_str("0");
        }
    }

    return gamma;
}

pub fn invert_gamma(mut gamma: String) -> String {
    gamma = gamma.replace("0", "x");
    gamma = gamma.replace("1", "0");
    gamma = gamma.replace("x", "1");

    return gamma;
}

pub fn traverse_binary_tree(input: &Vec<String>, invert: bool) -> usize {
    let mut mut_input: Vec<String> = input.to_vec();
    let mut needle;

    for i in 0..12 {
        needle = create_needle(&mut_input);
        if invert {
            needle = invert_gamma(needle);
        }

        mut_input = mut_input
            .iter()
            .filter(|e| e.chars().nth(i).unwrap() == needle.chars().nth(i).unwrap())
            .map(|e| e.to_string())
            .collect();

        if mut_input.len() == 1 {
            return usize::from_str_radix(&mut_input[0], 2).unwrap();
        }
    }

    return 0;
}

pub fn compute_a(input: Vec<String>) -> usize {
    let gamma = create_needle(&input);
    let gamma_value = usize::from_str_radix(&gamma, 2).unwrap();

    let epsilon = invert_gamma(gamma);
    let epsilon_value = usize::from_str_radix(&epsilon, 2).unwrap();

    return gamma_value * epsilon_value;
}

pub fn compute_b(input: Vec<String>) -> usize {
    let oxygen = traverse_binary_tree(&input, false);
    let co2 = traverse_binary_tree(&input, true);

    return oxygen * co2;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(get_input());
        assert_eq!(result, 845186);
    }

    #[test]
    fn example_b() {
        let result = compute_b(get_input());
        assert_eq!(result, 4636702);
    }
}
