use std::collections::HashMap;

pub struct Node {
    distance: i32,
    children: HashMap<i32, Vec<i32>>,
    parent: i32,
    coordinate: (i32, i32),
}
