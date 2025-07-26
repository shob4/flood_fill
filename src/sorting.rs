use rand::Rng;

pub fn partition(input: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let mut rng = rand::rng();
    let partition = input[rng.random_range(low..high)];
    let mut i: usize = low - 1;
    for j in low..high {
        if input[j] < partition {
            i += 1;
            (input[i], input[j]) = (input[j], input[i]);
        }
    }
    (input[i + 1], input[high]) = (input[high], input[i + 1]);
    return i + 1;
}

pub fn quick_sort(input: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pi: usize = partition(input, low, high);
        quick_sort(input, low, pi - 1);
        quick_sort(input, pi + 1, high);
    }
}
