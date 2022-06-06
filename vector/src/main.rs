use std::collections::HashMap;

fn main() {
    let vector = vec![24, 4, 2022, 16, 50, 4, 50];
    let median = get_median(&vector);
    println!("median {:?}", median);
    let mode = get_mode(&vector);
    println!("mode {:?}", mode);
}

#[derive(Debug)]
struct IndexedValue<T> {
    index: usize,
    value: T,
}

fn get_median<T: Ord + Copy>(vector: &Vec<T>) -> Option<IndexedValue<T>> {
    let mut sorted_vector = vector.clone();
    sorted_vector.sort();
    let index = sorted_vector.len() / 2;
    let median = sorted_vector.get(index);
    match median {
        Some(value) => Some(IndexedValue{index, value: *value}),
        None => None
    }
}

#[derive(Debug)]
struct CountedValue<T> {
    count: usize,
    value: T,
}

/** Mode is a value which appears in the array more often then all other values */
fn get_mode<T: Ord + Copy + std::hash::Hash>(vector: &Vec<T>) -> Option<CountedValue<T>> {
    let mut occurrences: HashMap<T, usize> = HashMap::new();
    for value in vector {
        let count = occurrences.entry(*value).or_insert(0);
        *count += 1;
    };
    let mut max_value: Option<CountedValue<T>> = None;
    for (value, count) in occurrences {
        match &max_value {
            None => max_value = Some(CountedValue{ count, value }),
            Some(current_max_value) => {
                if current_max_value.count < count {
                    max_value = Some(CountedValue { count, value });
                }
            }
        }
    };
    return max_value;
}
