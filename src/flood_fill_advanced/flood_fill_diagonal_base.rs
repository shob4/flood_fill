use crate::matrix::Matrix;
use std::collections::VecDeque;

fn flood_fill_diagonal_base(
    image: Matrix<i32>,
    length: usize,
    width: usize,
    sr: i32,
    sc: i32,
    color: i32,
) -> Matrix<i32> {
    let mut result_image: Matrix<i32> = Matrix {
        data: VecDeque::with_capacity(length),
        num_columns: width,
    };
    for index in 0..length {
        result_image.data[index] = image.data[index];
    }
    let start_color: &i32 = image.get(sr as usize, sc as usize).unwrap();
    let mut not_visited: VecDeque<(usize, usize)> = VecDeque::with_capacity(length);
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let directions: [(usize, usize); 3] = [(1, 0), (0, 1), (1, 1)];
    not_visited.push_back((sr as usize, sc as usize));
    while !visited.is_empty() {
        // visit new node
        let new_tuple: (usize, usize) = match not_visited.pop_front() {
            None => panic!("empty not_visited"),
            Some(i) => i,
        };
        let (new_sr, new_sc) = new_tuple;
        // going through all adjacent nodes, including diagonals
        for (dx, dy) in directions {
            visited.push(new_tuple);
            // setting new node to correct color
            result_image.data[result_image.num_columns * new_sr + new_sc] = color;
            let add_dx: usize = match new_sr.checked_add(dx) {
                Some(result) => result,
                None => new_sc,
            };
            let add_dy: usize = match new_sc.checked_add(dy) {
                Some(result) => result,
                None => new_sr,
            };
            let sub_dx: usize = match new_sr.checked_sub(dx) {
                Some(result) => result,
                None => new_sc,
            };
            let sub_dy: usize = match new_sr.checked_sub(dy) {
                Some(result) => result,
                None => new_sr,
            };
            if !visited.iter().any(|&i| i == (add_dx, add_dy)) {
                match image.get(add_dx, add_dy) {
                    Err(error) => panic!("{error}"),
                    Ok(k) => {
                        if k == start_color {
                            not_visited.push_back((add_dx, add_dy));
                        }
                    }
                };
            }
            if !visited.iter().any(|&i| i == (sub_dx, sub_dy)) {
                match image.get(sub_dx, sub_dy) {
                    Err(error) => panic!("{error}"),
                    Ok(k) => {
                        if k == start_color {
                            not_visited.push_back((sub_dx, sub_dy));
                        }
                    }
                };
            }
        }
    }
    result_image
}
