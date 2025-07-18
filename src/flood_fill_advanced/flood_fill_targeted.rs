use crate::matrix::Matrix;
use std::collections::VecDeque;

/*
 * finds the best path to the target coordinate from the starting coordinate by setting each coordinate to a number based on the
 * distance from the target, then attempting to traverse the fastest path, updating coordinate's
 * distances from the target as it goes
 */
fn flood_fill_targeted(
    length: usize,
    width: usize,
    sr: usize,
    sc: usize,
    tr: usize,
    tc: usize,
    acceptable_difference: i32,
) -> Matrix<i32> {
    let mut result_image: Matrix<i32> = Matrix {
        data: VecDeque::with_capacity(length),
        num_columns: width,
    };
    let mut distance: i32 = 1;
    let directions: [(usize, usize); 3] = [(1, 0), (0, 1), (1, 1)];
}

fn find_distance(col1: usize, col2: usize, row1: usize, row2: usize) {
    let col: isize = col2 as isize - col1 as isize;
    let row: isize = row2 as isize - row1 as isize;
}
