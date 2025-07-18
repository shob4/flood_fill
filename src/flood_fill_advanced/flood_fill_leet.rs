use std::collections::VecDeque;

fn flood_fill_leet(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let mut result_image: Vec<Vec<i32>> = Vec::new();
    for i in 0..image.len() {
        for j in 0..image[i].len() {
            result_image[i][j] = image[i][j];
        }
    }
    let start_color = match image.get(sr as usize).and_then(|r| r.get(sc as usize)) {
        Some(i) => i,
        None => return result_image,
    };
    let mut visited: VecDeque<(usize, usize)> = VecDeque::new();
    let mut not_visited: VecDeque<(usize, usize)> = VecDeque::new();
    not_visited.push_back((sr as usize, sc as usize));
    let directions: [(usize, usize); 2] = [(0, 1), (1, 0)];
    while !not_visited.is_empty() {
        let new_tuple = match not_visited.pop_front() {
            Some(i) => i,
            None => panic!("no tuple to pop"),
        };
        let (new_sr, new_sc) = new_tuple;
        visited.push_front((new_sr, new_sc));
        for (dx, dy) in directions {
            result_image[new_sr][new_sc] = color;
            let add_dx: usize = match new_sr.checked_add(dx) {
                Some(i) => i,
                None => new_sr,
            };
            let add_dy: usize = match new_sc.checked_add(dy) {
                Some(i) => i,
                None => new_sc,
            };
            let sub_dx: usize = match new_sr.checked_sub(dx) {
                Some(i) => i,
                None => new_sr,
            };
            let sub_dy: usize = match new_sc.checked_sub(dy) {
                Some(i) => i,
                None => new_sc,
            };
            if !visited.iter().any(|&i| i == (add_dx, add_dy)) {
                match image.get(add_dx).and_then(|r| r.get(add_dy)) {
                    Some(i) => {
                        if i == start_color {
                            not_visited.push_back((add_dx, add_dy));
                        }
                    }
                    None => (),
                };
            }
            if !visited.iter().any(|&i| i == (sub_dx, sub_dy)) {
                match image.get(sub_dx).and_then(|r| r.get(sub_dy)) {
                    Some(i) => {
                        if i == start_color {
                            not_visited.push_back((sub_dx, sub_dy));
                        }
                    }
                    None => (),
                };
            }
        }
    }
    result_image
}
