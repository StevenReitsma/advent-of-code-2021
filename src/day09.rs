use ndarray::Array2;
use ndarray::Axis;
use std::collections::HashSet;
use std::fs;

pub fn get_input() -> Array2<u32> {
    let mut input = fs::read_to_string("input/day09.txt").expect("couldn't read input file");
    input.retain(|c| !c.is_whitespace());
    return Array2::from_shape_vec(
        (100, 100),
        input.chars().map(|x| x.to_digit(10).unwrap()).collect(),
    )
    .unwrap();
}

pub fn get_neighbors(input: &Array2<u32>, x: usize, y: usize) -> Vec<(usize, usize)> {
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

    return neighbors;
}

pub fn get_lowest_neighbor(input: &Array2<u32>, x: usize, y: usize) -> u32 {
    return get_neighbors(input, x, y)
        .iter()
        .map(|(x, y)| input[[*x, *y]])
        .min()
        .unwrap();
}

pub fn get_lowest_points(input: &Array2<u32>) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    for ((x, y), value) in input.indexed_iter() {
        let lowest_neighbor = get_lowest_neighbor(input, x, y);
        if *value < lowest_neighbor {
            points.push((x, y));
        }
    }

    return points;
}

pub fn compute_a(input: &Array2<u32>) -> u32 {
    let mut risk_level_sum = 0;
    let lowest_points = get_lowest_points(input);

    for point in lowest_points {
        risk_level_sum += input[[point.0, point.1]] + 1;
    }

    return risk_level_sum;
}

pub fn compute_b(input: &Array2<u32>) -> u32 {
    let lowest_points = get_lowest_points(input);
    let mut basins = Vec::new();

    // From each lowest point, search BFS to find basin
    for lowest_point in lowest_points {
        let mut done = HashSet::<(usize, usize)>::new();
        done.insert(lowest_point);
        let mut todo = vec![lowest_point];
        while !todo.is_empty() {
            let current_point = todo.pop().unwrap();
            for neighbor in get_neighbors(input, current_point.0, current_point.1) {
                if !done.contains(&neighbor)
                    && input[[neighbor.0, neighbor.1]] != 9
                    && input[[neighbor.0, neighbor.1]] > input[[current_point.0, current_point.1]]
                {
                    done.insert(neighbor);
                    todo.push(neighbor);
                }
            }
        }

        basins.push(done.len());
    }

    basins.sort();
    basins.reverse();
    return basins.iter().take(3).fold(1, |acc, x| acc * *x as u32);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute_a(&get_input());
        assert_eq!(result, 465);
    }

    #[test]
    fn example_b() {
        let result = compute_b(&get_input());
        assert_eq!(result, 1269555);
    }
}
