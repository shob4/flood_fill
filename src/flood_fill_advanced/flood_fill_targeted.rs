use crate::matrix::Matrix;
use crate::path_tree::Node;
use std::collections::VecDeque;

/*
 * finds the best path to the target coordinate from the starting coordinate by setting each coordinate to a number based on the
 * distance from the target, then attempting to traverse the fastest path, updating coordinate's
 * distances from the target as it goes
 */
fn flood_fill_targeted(
    width: usize,
    height: usize,
    sc: usize,
    sr: usize,
    tc: usize,
    tr: usize,
    acceptable_difference: i32,
) -> Matrix<i32> {
    let mut result_image: Matrix<i32> = Matrix {
        data: VecDeque::with_capacity(width * height),
        num_columns: width,
    };
    for col in 0..width {
        for row in 0..height {
            let index = result_image.num_columns * col + row;
            result_image.data[index] = find_distance(col, tc, row, tr);
        }
    }
    let mut not_visited: VecDeque<(usize, usize)> = VecDeque::with_capacity(width * height);
    // TODO figure out how to make this the tree
    let mut visited: VecDeque<(usize, usize)> = VecDeque::with_capacity(width * height);
    let directions: [(usize, usize); 3] = [(1, 0), (0, 1), (1, 1)];
    not_visited.push_front((sc, sr));
    while !not_visited.is_empty() {
        let next_coordinate: (usize, usize) = match not_visited.pop_front() {
            Some(i) => i,
            None => panic!("Empty not_visited in loop"),
        };
        let (new_sc, new_sr) = next_coordinate;
        // TODO needs a way to update distance information. Should I be redoing the whole thing
        // with Nodes?
        // redo from current position to there? how? maybe add 1?
        // visited should be in tree form
        let color: i32 = function_to_determine_new_distance();
        result_image.data[result_image.num_columns * new_sc + new_sr] = color;
        visited.push_front(next_coordinate);
        for (dc, dr) in directions {
            let add_dc: usize = match new_sc.checked_add(dc) {
                Some(i) => i,
                None => new_sc,
            };
            let add_dr: usize = match new_sr.checked_add(dr) {
                Some(i) => i,
                None => new_sr,
            };
            let sub_dc: usize = match new_sc.checked_sub(dc) {
                Some(i) => i,
                None => new_sc,
            };
            let sub_dr: usize = match new_sr.checked_sub(dr) {
                Some(i) => i,
                None => new_sr,
            };
        }
    }
    result_image
}

fn find_distance(col1: usize, col2: usize, row1: usize, row2: usize) -> i32 {
    let col: i32 = col2 as i32 - col1 as i32;
    let row: i32 = row2 as i32 - row1 as i32;
    let col_sqr: i32 = col * col;
    let row_sqr: i32 = row * row;
    let to_be_rooted: i32 = col_sqr + row_sqr;
    let rooted: i32 = to_be_rooted.isqrt();
    rooted
}

fn function_to_determine_new_distance() -> i32 {
    0
}
