use crate::matrix::Matrix;
use std::collections::VecDeque;

// IMPORTANT columns and rows should be in the right place here
/*
 * finds all pixels within the acceptable range from the start color and changes them to the
 * provided color.
 */
fn flood_fill_diagonal_range(
    image: Matrix<i32>,
    length: usize,
    width: usize,
    sr: i32,
    sc: i32,
    color: i32,
    acceptable_difference: i32,
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
    let mut visited: VecDeque<(usize, usize)> = VecDeque::new();
    let directions: [(usize, usize); 3] = [(1, 0), (0, 1), (1, 1)];
    not_visited.push_back((sc as usize, sr as usize));
    while !not_visited.is_empty() {
        let next_coordinate: (usize, usize) = match not_visited.pop_front() {
            Some(i) => i,
            None => panic!("Empty not_visited in loop"),
        };
        let (new_sc, new_sr) = next_coordinate;
        result_image.data[result_image.num_columns * new_sc + new_sr] = color;
        visited.push_front(next_coordinate);
        for (dc, dr) in directions {
            let add_dr: usize = match new_sr.checked_add(dr) {
                Some(i) => i,
                None => new_sr,
            };
            let add_dc: usize = match new_sc.checked_add(dc) {
                Some(i) => i,
                None => new_sc,
            };
            let sub_dr: usize = match new_sr.checked_sub(dr) {
                Some(i) => i,
                None => new_sr,
            };
            let sub_dc: usize = match new_sc.checked_sub(dc) {
                Some(i) => i,
                None => new_sc,
            };
            if !visited.iter().any(|&i| i == (add_dr, add_dc)) {
                match image.get_option(add_dc, add_dr) {
                    Some(i) => {
                        if *i <= start_color + acceptable_difference
                            && *i >= start_color - acceptable_difference
                        {
                            visited.push_back((add_dc, add_dr));
                        }
                    }
                    None => (),
                }
                match image.get_option(sub_dc, sub_dr) {
                    Some(i) => {
                        if *i <= start_color + acceptable_difference
                            && *i >= start_color - acceptable_difference
                        {
                            visited.push_back((sub_dc, add_dr));
                        }
                    }
                    None => (),
                }
            };
        }
    }
    result_image
}
