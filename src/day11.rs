use ndarray::Array2;
use ndarray::Axis;
use std::collections::HashSet;
use std::fs;

pub fn get_input() -> Array2<u32> {
    let input = fs::read_to_string("input/day11.txt").expect("couldn't read input file");
    return Array2::from_shape_vec(
        (10, 10),
        input
            .replace("\n", "")
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect(),
    )
    .unwrap();
}

pub fn get_diagonal_neighbors(input: &Array2<u32>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if x > 0 {
        neighbors.push((x - 1, y));
    }

    if x + 1 < input.len_of(Axis(0)) {
        neighbors.push((x + 1, y));
    }

    if y > 0 {
        neighbors.push((x, y - 1));
    }

    if y + 1 < input.len_of(Axis(1)) {
        neighbors.push((x, y + 1));
    }

    // Diagonals
    if x > 0 && y > 0 {
        neighbors.push((x - 1, y - 1));
    }

    if x > 0 && y + 1 < input.len_of(Axis(1)) {
        neighbors.push((x - 1, y + 1));
    }

    if x + 1 < input.len_of(Axis(0)) && y > 0 {
        neighbors.push((x + 1, y - 1));
    }

    if x + 1 < input.len_of(Axis(0)) && y + 1 < input.len_of(Axis(1)) {
        neighbors.push((x + 1, y + 1));
    }

    return neighbors;
}

pub fn compute_a(input: &Array2<u32>, sync: bool) -> usize {
    let mut octopuses = input.clone();
    let mut total_flashes = 0;

    let range = match sync {
        true => 0..9999,
        false => 0..100,
    };

    for step in range {
        octopuses = octopuses + 1;

        let mut flashed = HashSet::<(usize, usize)>::new();
        while octopuses.iter().any(|x| *x > 9) {
            for (x, y) in itertools::iproduct!(0..10, 0..10) {
                if octopuses[[x, y]] <= 9 || flashed.contains(&(x, y)) {
                    continue;
                }

                // Flash!
                flashed.insert((x, y));
                octopuses[[x, y]] = 0;
                for (nx, ny) in get_diagonal_neighbors(&octopuses, x, y) {
                    octopuses[[nx, ny]] += 1;
                }
            }
        }

        // Set flashed back to 0
        flashed.iter().for_each(|x| octopuses[[x.0, x.1]] = 0);
        total_flashes += flashed.len();

        if flashed.len() == 100 && sync {
            return step + 1;
        }
    }

    return total_flashes;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(&get_input(), false);
        assert_eq!(result, 1681);
    }

    #[test]
    fn example_b() {
        let result = compute_a(&get_input(), true);
        assert_eq!(result, 276);
    }
}
