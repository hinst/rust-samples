fn main() {
    let mut vector = vec![24, 4, 2022, 16, 50, 4, 50];
    println!("{:?}", vector);
    vector.sort();
    println!("{:?}", vector);
    let median_index = vector.len() / 2;
    let median = vector.get(median_index);
    println!("Median {:?} at index {}", median, median_index);
}
