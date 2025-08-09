use std::collections::HashMap;

pub struct Node {
    distance: i32,
    children: HashMap<i32, Vec<i32>>,
    parent: i32,
    coordinate: (usize, usize),
}

impl Node {
    pub fn new(dist: i32, par: Option<i32>, coord: (usize, usize)) -> Node {
        Node {
            distance: dist,
            children: HashMap::new(),
            parent: match par {
                Some(x) => x,
                None => 0,
            },
            coordinate: coord,
        }
    }
}
