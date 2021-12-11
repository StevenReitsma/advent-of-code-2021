use std::fs;

pub fn get_input() -> Vec<String> {
    let mut input = fs::read_to_string("input/day10.txt").expect("couldn't read input file");
    return input.lines().map(|x| x.to_string()).collect();
}

pub fn compute_a(input: Vec<String>) -> u32 {
    let mut error_score = 0;
    for line in input {
        let mut stack = Vec::new();

        for char in line.chars() {
            match char {
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                '(' => stack.push(')'),
                a => {
                    let exp = stack.pop().unwrap();
                    if a != exp {
                        error_score += match a {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("no"),
                        };
                    }
                }
            }
        }
    }

    return error_score;
}

// Too lazy to rewrite `a` into reusable functions
pub fn compute_b(input: Vec<String>) -> u64 {
    let mut scores: Vec<u64> = Vec::new();
    for line in input {
        let mut stack = Vec::new();
        let mut invalid = false;

        for char in line.chars() {
            match char {
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                '(' => stack.push(')'),
                a => {
                    let exp = stack.pop().unwrap();
                    if a != exp {
                        // Line is not valid, discard
                        invalid = true;
                        break;
                    }
                }
            }
        }

        if !invalid {
            stack.reverse();
            scores.push(stack.iter().fold(0 as u64, |acc, c| acc * 5 + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("no"),
            }));
        }
    }

    scores.sort();
    return scores[scores.len() / 2];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(get_input());
        assert_eq!(result, 296535);
    }

    #[test]
    fn example_b() {
        let result = compute_b(get_input());
        assert_eq!(result, 4245130838);
    }
}
