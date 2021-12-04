use ndarray::Array2;
use ndarray::Axis;
use std::fs;

pub fn get_numbers() -> Vec<isize> {
    let input = fs::read_to_string("input/day04.txt").expect("couldn't read input file");
    return input
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();
}

pub fn get_boards() -> Vec<Array2<isize>> {
    let input = fs::read_to_string("input/day04.txt").expect("couldn't read input file");
    return input
        .lines()
        .skip(2)
        .fold("".to_string(), |acc, e| acc + "\n" + e)
        .split("\n\n")
        .map(|s| {
            Array2::from_shape_vec(
                (5, 5),
                s.replace("\n", " ")
                    .split(" ")
                    .filter(|x| *x != "")
                    .map(|x| x.parse().unwrap())
                    .collect(),
            )
            .unwrap()
        })
        .collect();
}

pub fn scratch_number(board: &Array2<isize>, number: isize) -> Array2<isize> {
    let vec = board.clone().into_raw_vec();

    return Array2::from_shape_vec(
        (5, 5),
        vec.iter()
            .map(|x| if *x == number { -1 } else { *x })
            .collect(),
    )
    .unwrap();
}

pub fn won(board: &Array2<isize>) -> bool {
    // If any row or column sums to -5
    return board.axis_iter(Axis(0)).map(|x| x.sum()).any(|x| x == -5)
        || board.axis_iter(Axis(1)).map(|x| x.sum()).any(|x| x == -5);
}

pub fn compute_a(numbers: Vec<isize>, mut boards: Vec<Array2<isize>>) -> isize {
    for num in numbers {
        for i in 0..boards.len() {
            boards[i] = scratch_number(&boards[i], num);
            if won(&boards[i]) {
                return num
                    * &boards[i]
                        .iter()
                        .fold(0, |acc, e| acc + if *e == -1 { 0 } else { *e });
            }
        }
    }

    return 0;
}

pub fn compute_b(numbers: Vec<isize>, mut boards: Vec<Array2<isize>>) -> isize {
    let mut last_score: isize = 0;

    for num in numbers {
        for i in 0..boards.len() {
            if &boards[i].sum() == &0 {
                continue;
            }
            boards[i] = scratch_number(&boards[i], num);
            if won(&boards[i]) {
                last_score = num
                    * &boards[i]
                        .iter()
                        .fold(0, |acc, e| acc + if *e == -1 { 0 } else { *e });
                boards[i] = &boards[i] * 0;
            }
        }
    }

    return last_score;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(get_numbers(), get_boards());
        assert_eq!(result, 50008);
    }

    #[test]
    fn example_b() {
        let result = compute_b(get_numbers(), get_boards());
        assert_eq!(result, 17408);
    }
}
