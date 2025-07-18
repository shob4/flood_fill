use std::collections::VecDeque;

pub struct Matrix<T> {
    pub data: VecDeque<T>,
    pub num_columns: usize,
}

impl<T> Matrix<T> {
    pub fn get(&self, col: usize, row: usize) -> Result<&T, String> {
        let index: usize = self.num_columns * col + row;
        match self.data.get(index) {
            Some(x) => Ok(x),
            None => Err("cannot get: {index} is out of {self.data.len}".to_string()),
        }
    }

    pub fn get_option(&self, col: usize, row: usize) -> Option<&T> {
        let index: usize = self.num_columns * col + row;
        match self.data.get(index) {
            Some(x) => Some(x),
            None => None,
        }
    }

    // fn set(&self, col: usize, row: usize, value: i32) {
    //     let index: usize = self.num_columns * row + col;
    //     self.data[index] = value;
    // }
}
