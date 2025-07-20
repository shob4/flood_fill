use crate::matrix::Matrix;
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
    let mut distance: i32 = 1;
    let directions: [(usize, usize); 3] = [(1, 0), (0, 1), (1, 1)];
    for col in 0..width {
        for row in 0..height {
            let index = result_image.num_columns * col + row;
            result_image.data[index] = find_distance(col, tc, row, tr);
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
