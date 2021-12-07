use std::fs;

pub fn get_input() -> Vec<isize> {
    let input = fs::read_to_string("input/day06.txt").expect("couldn't read input file");
    return input.split(",").map(|s| s.parse().unwrap()).collect();
}

pub fn compute_naive(input: &Vec<isize>, days: usize) -> usize {
    let mut fish = input.clone();

    for day in 0..days {
        fish = fish.iter().map(|x| x - 1).collect();

        let fish_before = fish.len();

        // Remove parents
        fish = fish.iter().filter(|el| **el >= 0).map(|x| *x).collect();

        let num_parents = fish_before - fish.len();

        // Add parent back (6) and add its child (8)
        for i in 0..num_parents {
            fish.push(8);
            fish.push(6);
        }

        println!("Processing day {}", day);
    }

    return fish.len();
}

pub fn compute_smort(input: &Vec<isize>, days: usize) -> usize {
    let mut ages: Vec<usize> = Vec::new();

    for day in 0..7 {
        ages.push(input.iter().filter(|x| **x == day).count());
    }

    let mut current_day_phase = 0;
    let mut delayed_insert_in_two_days = 0;
    let mut delayed_insert_in_one_day = 0;
    for day in 0..days {
        let num_parents = ages[current_day_phase];

        ages[current_day_phase] += delayed_insert_in_one_day;
        delayed_insert_in_one_day = delayed_insert_in_two_days;
        delayed_insert_in_two_days = num_parents;

        current_day_phase = (current_day_phase + 1) % 7;
    }

    return ages.iter().sum::<usize>() + delayed_insert_in_one_day + delayed_insert_in_two_days;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result_naive = compute_naive(&get_input(), 80);
        let result_smort = compute_smort(&get_input(), 80);
        assert_eq!(result_naive, 355386);
        assert_eq!(result_smort, 355386);
    }

    #[test]
    fn example_b() {
        let result = compute_smort(&get_input(), 256);
        assert_eq!(result, 1613415325809);
    }
}
