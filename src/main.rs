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
    println!("wow");
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
    not_visited.push_back((sr as usize, sc as usize));
    while !not_visited.is_empty() {
        // visit new node
        let new_tuple: (usize, usize) = match not_visited.pop_front() {
            None => panic!("empty not_visited, line 53"),
            Some(i) => i,
        };
        let (new_sr, new_sc) = new_tuple;
        // setting new node to correct color
        result_image.data[result_image.num_columns * new_sc + new_sr] = color;
        // checking if adjacent nodes have been visited or have the correct color
        let new_color = image.get(new_sr + 1, new_sc);
        let new_color = match new_color {
            Err(error) => panic!("{error}"),
            Ok(i) => i,
        };
        if new_color == start_color {
            not_visited.push_back((new_sr + 1, new_sc));
        }
        if image.get(new_sr, new_sc + 1).unwrap() == start_color {
            not_visited.push_back((new_sr, new_sc + 1));
        }
        if image.get(new_sr - 1, new_sc).unwrap() == start_color {
            not_visited.push_back((new_sr - 1, new_sc));
        }
        if image.get(new_sr, new_sc - 1).unwrap() == start_color {
            not_visited.push_back((new_sr, new_sc - 1));
        }
    }

    result_image
}
