use std::thread;
use std::time::Duration;

fn main() {
    println!("STARTING...");
    let x = 4;
    let equal_to_x = move |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
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