use std::collections::VecDeque;

struct Matrix<T> {
    data: VecDeque<T>,
    num_columns: usize,
}

impl<T> Matrix<T> {
    fn get(&self, col: isize, row: isize) -> Result<&T, String> {
        let index: isize = self.num_columns as isize * row + col;
        match self.data.get(index as usize) {
            Some(x) => Ok(x),
            None => Err("{index} is out of {self.data.len}".to_string()),
        }
    }

    // fn set(&self, col: usize, row: usize, value: i32) {
    //     let index: usize = self.num_columns * row + col;
    //     self.data[index] = value;
    // }
}

fn main() {
    println!("Hello, world!");
}

/*
 * a bfs-like algorithm that changes the "color" of all "pixels" in the "image" matching the target
 * color
 * move to matching within degree of one for maze solving?
 */
fn flood_fill_base(
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
    let start_color: &i32 = image.get(sr as isize, sc as isize).unwrap();
    let mut not_visited: VecDeque<(isize, isize)> = VecDeque::with_capacity(length);
    let mut visited: Vec<(isize, isize)> = Vec::new();
    not_visited.push_back((sr as isize, sc as isize));
    while !not_visited.is_empty() {
        // visit new node
        let new_tuple: (isize, isize) = match not_visited.pop_front() {
            None => panic!("empty not_visited, line 53"),
            Some(i) => i,
        };
        let (new_sr, new_sc) = new_tuple;
        visited.push(new_tuple);
        // setting new node to correct color
        result_image.data[result_image.num_columns * new_sc as usize + new_sr as usize] = color;
        // checking if adjacent nodes have been visited or have the correct color
        if !visited.iter().any(|&i| i == (new_sc + 1, new_sr)) {
            match image.get(new_sr + 1, new_sc) {
                Err(error) => panic!("{error}"),
                Ok(i) => {
                    if i == start_color {
                        not_visited.push_back((new_sr + 1, new_sc));
                    }
                }
            };
        }
        if !visited.iter().any(|&i| i == (new_sc, new_sr + 1)) {
            match image.get(new_sr, new_sc + 1) {
                Err(error) => panic!("{error}"),
                Ok(i) => {
                    if i == start_color {
                        not_visited.push_back((new_sr, new_sc + 1));
                    }
                }
            };
        }
        if !visited.iter().any(|&i| i == (new_sc - 1, new_sr)) {
            match image.get(new_sr - 1, new_sc) {
                Err(error) => panic!("{error}"),
                Ok(i) => {
                    if i == start_color {
                        not_visited.push_back((new_sr - 1, new_sc));
                    }
                }
            };
        }
        if !visited.iter().any(|&i| i == (new_sc, new_sr - 1)) {
            match image.get(new_sr, new_sc - 1) {
                Err(error) => panic!("{error}"),
                Ok(i) => {
                    if i == start_color {
                        not_visited.push_back((new_sr, new_sc - 1));
                    }
                }
            };
        }
    }

    result_image
}

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
    let start_color: &i32 = image.get(sr as isize, sc as isize).unwrap();
    let mut not_visited: VecDeque<(isize, isize)> = VecDeque::with_capacity(length);
    let mut visited: Vec<(isize, isize)> = Vec::new();
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    not_visited.push_back((sr as isize, sc as isize));
    while !visited.is_empty() {
        // going through all adjacent nodes, including diagonals
        for (dx, dy) in directions {
            // visit new node
            let new_tuple: (isize, isize) = match not_visited.pop_front() {
                None => panic!("empty not_visited, line 132"),
                Some(i) => i,
            };
            let (new_sr, new_sc) = new_tuple;
            visited.push(new_tuple);
            // setting new node to correct color
            result_image.data[result_image.num_columns * new_sc as usize + new_sr as usize] = color;
            if !visited.iter().any(|&i| i == (new_sr + dx, new_sc + dy)) {
                match image.get(new_sr + dx, new_sc + dy) {
                    Err(error) => panic!("{error}, 141"),
                    Ok(k) => {
                        if k == start_color {
                            not_visited.push_back((new_sr + dx, new_sc + dy));
                        }
                    }
                };
            }
        }
    }
    result_image
}
