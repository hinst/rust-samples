use std::thread;
use std::time::Duration;

fn main() {
    println!("STARTING...");
    iterate_counter();
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        return Cacher { calculation, value: None }
    }

    fn get_value(&mut self, arg: u32) -> u32{
        match self.value {
            Some(value) => value,
            None => {
                let value = (self.calculation)(arg);
                self.value = Some(value);
                return value;
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cacher = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.get_value(intensity));
        println!("Next, do {} situps!", cacher.get_value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!("Today, run for {} minutes", cacher.get_value(intensity));
        }
    }
}

fn iterate() {
    let vector: Vec<i32> = vec![1, 2, 3];
    let iterator = vector.iter();
    let sum: i32 = iterator.sum();
    println!("sum: {}", sum);
}

fn iterate2() {
    let vector: Vec<i32> = vec![1, 2, 3];
    let vector2: Vec<i32> = vector.iter().map(|x| x + 1).collect();
    println!("{:?}", vector2);
}

struct Counter {
    current: usize,
    limit: usize,
}

impl Counter {
    fn new(limit: usize) -> Counter {
        return Counter { current: 0, limit };
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.limit {
            let current = self.current;
            self.current += 1;
            return Some(current);
        } else {
            return None;
        }
    }
}

fn iterate_counter() {
    let counter = Counter::new(5);
    for item in counter {
        println!("counter {}", item);
    }

    let counter2 = Counter::new(5);
    let counter3 = Counter::new(6).skip(1);
    let counters: Vec<_> = counter2.zip(counter3).collect();
    println!("counters {:?}", counters);
}