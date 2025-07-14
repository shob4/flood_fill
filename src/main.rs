use std::collections::VecDeque;

struct Matrix<T> {
    data: VecDeque<T>,
    num_columns: usize,
}

impl<T> Matrix<T> {
    fn get(&self, col: usize, row: usize) -> Result<&T, String> {
        let index: usize = self.num_columns * row + col;
        match self.data.get(index) {
            Some(x) => Ok(x),
            None => Err("cannot get: {index} is out of {self.data.len}".to_string()),
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
    let start_color: &i32 = image.get(sr as usize, sc as usize).unwrap();
    let mut not_visited: VecDeque<(usize, usize)> = VecDeque::with_capacity(length);
    let mut visited: Vec<(usize, usize)> = Vec::new();
    not_visited.push_back((sr as usize, sc as usize));
    while !not_visited.is_empty() {
        // visit new node
        let new_tuple: (usize, usize) = match not_visited.pop_front() {
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

/*
 * This implementation is a mess
 * isize is needed for subtracting from usize readably, but it's a mess everywhere else
 */
fn flood_fill_diagonal_base_mess(
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
    let directions: [(usize, usize); 2] = [(1, 0), (0, 1)];
    not_visited.push_back((sr as usize, sc as usize));
    while !visited.is_empty() {
        // going through all adjacent nodes, including diagonals
        for (dx, dy) in directions {
            // visit new node
            let new_tuple: (usize, usize) = match not_visited.pop_front() {
                None => panic!("empty not_visited"),
                Some(i) => i,
            };
            let (new_sr, new_sc) = new_tuple;
            visited.push(new_tuple);
            // setting new node to correct color
            result_image.data[result_image.num_columns * new_sc + new_sr] = color;
            if !visited.iter().any(|&i| i == (new_sr + dx, new_sc + dy)) {
                match image.get(new_sr + dx, new_sc + dy) {
                    Err(error) => panic!("{error}"),
                    Ok(k) => {
                        if k == start_color {
                            not_visited.push_back((new_sr + dx, new_sc + dy));
                        }
                    }
                };
            }
            if !visited.iter().any(|&i| i == (new_sr - dx, new_sr - dy)) {
                match image.get(new_sr - dx, new_sc - dy) {
                    Err(error) => panic!("{error}"),
                    Ok(k) => {
                        if k == start_color {
                            not_visited.push_back((new_sr - dx, new_sc - dy));
                        }
                    }
                };
            }
        }
    }
    result_image
}
