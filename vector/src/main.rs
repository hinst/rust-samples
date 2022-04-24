use std::collections::HashMap;

fn main() {
    let vector = vec![24, 4, 2022, 16, 50, 4, 50];
    let median = get_median(&vector);
    println!("median {:?}", median);
    let mode = get_mode(&vector);
    println!("mode {:?}", mode);
}

#[derive(Debug)]
struct IndexedIsize {
    index: usize,
    value: isize,
}

fn get_median(vector: &Vec<isize>) -> Option<IndexedIsize> {
    let mut sorted_vector = vector.clone();
    sorted_vector.sort();
    let index = sorted_vector.len() / 2;
    let median = sorted_vector.get(index);
    match median {
        Some(value) => Some(IndexedIsize{index, value: *value}),
        None => None
    }
}

#[derive(Debug)]
struct CountedIsize {
    count: usize,
    value: isize,
}

/** Mode is a value which appears in the array more often then all other values */
fn get_mode(vector: &Vec<isize>) -> Option<CountedIsize> {
    let mut occurrences: HashMap<isize, usize> = HashMap::new();
    for value in vector {
        let count = occurrences.entry(*value).or_insert(0);
        *count += 1;
    };
    let mut max_value: Option<CountedIsize> = None;
    for (value, count) in occurrences {
        match &max_value {
            None => max_value = Some(CountedIsize{ count, value }),
            Some(current_max_value) => {
                if current_max_value.count < count {
                    max_value = Some(CountedIsize { count, value });
                }
            }
        }
    };
    return max_value;
}
