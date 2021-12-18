use std::collections::HashSet;
use std::fs;

pub fn get_input() -> (HashSet<(i32, i32)>, Vec<(char, i32)>) {
    let input = fs::read_to_string("input/day13.txt").expect("couldn't read input file");
    let (dots, folds) = input.split_once("\n\n").unwrap();

    return (
        dots.split("\n")
            .map(|line| line.split_once(",").unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect(),
        folds
            .split("\n")
            .map(|line| line.split_once("=").unwrap())
            .map(|(a, b)| (a.chars().last().unwrap(), b.parse().unwrap()))
            .collect(),
    );
}

pub fn fold(dots: &HashSet<(i32, i32)>, fold: (char, i32)) -> HashSet<(i32, i32)> {
    let (axis, line) = fold;

    let mut new_dots = HashSet::new();

    for (x, y) in dots {
        match axis {
            'x' => {
                if *x > line {
                    new_dots.insert((2 * line - x, *y));
                } else {
                    new_dots.insert((*x, *y));
                }
            }
            'y' => {
                if *y > line {
                    new_dots.insert((*x, 2 * line - y));
                } else {
                    new_dots.insert((*x, *y));
                }
            }
            _ => panic!(),
        }
    }

    return new_dots;
}

pub fn compute_a(input: &(HashSet<(i32, i32)>, Vec<(char, i32)>)) -> usize {
    let (dots, folds) = input;

    return fold(dots, folds[0]).len();
}

pub fn compute_b(input: &(HashSet<(i32, i32)>, Vec<(char, i32)>)) -> String {
    let (dots, folds) = input;

    let mut mdots = dots.clone();

    for f in folds {
        mdots = fold(&mdots, *f);
    }

    for y in 0..6 {
        for x in 0..39 {
            if mdots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    return "JZGUAPRB".to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(&get_input());
        assert_eq!(result, 942);
    }

    #[test]
    fn example_b() {
        let result = compute_b(&get_input());
        assert_eq!(result, "JZGUAPRB");
    }
}
