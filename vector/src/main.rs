fn main() {
    let vector = vec![24, 4, 2022, 16, 50, 4, 50];
    let median = get_median(&vector);
    println!("Median {:?}", median);
}


#[derive(Debug)]
struct Median {
    index: usize,
    value: isize,
}

fn get_median(vector: &Vec<isize>) -> Option<Median> {
    let mut sorted_vector = vector.clone();
    sorted_vector.sort();
    let index = sorted_vector.len() / 2;
    let median = sorted_vector.get(index);
    match median {
        Some(value) => Some(Median{index, value: *value}),
        None => None
    }
}
