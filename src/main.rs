use std::collections::VecDeque;

struct Matrix<T> {
    data: VecDeque<T>,
    num_columns: usize,
}

impl<T> Matrix<T> {
    fn get(&self, col: usize, row: usize) -> Option<&T> {
        if col >= self.num_columns {
            return None;
        }
        let index: usize = self.num_columns * row + col;
        self.data.get(index)
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
    not_visited.push_back((sr as usize, sc as usize));
    while !not_visited.is_empty() {
        let new_tuple: Option<(usize, usize)> = not_visited.pop_front();
        let (new_sr, new_sc) = new_tuple.unwrap();
        result_image.data[result_image.num_columns * new_sc + new_sr] = color;
        // TODO maybe check if it exists
        if image.get(new_sr + 1, new_sc).unwrap() == start_color {
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
