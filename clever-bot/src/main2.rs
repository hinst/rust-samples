use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    if true { // Fibonacci sequence
        for i in 0..20 {
            println!("{}", fib(i))
        }
    }
    if false { // Fahrenheit
        let fahrenheit_temperature = 60.0;
        println!("{} F -> {} C", fahrenheit_temperature, fahrenheit_to_celsius(fahrenheit_temperature));
    }
    if false {
        for_each();
    }
    if false {
        le_while();
    }
    if false {
        le_function();
    }
    if false {
        why();
    }
    if false {
        le_months();
    }
    if false {
        le_tuple();
    }
}

fn secret_app() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number");

    'haha_loop: loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big")
        };
    }
}

fn le_loop() {
    let mut x: f64 = 0.1 + 0.2;
    println!("{}", x)
}

fn le_tuple() {
    let tuple: (i32, i64) = (3, 6);
    let three = tuple.0;
    let six = tuple.1;
    let (three, six) = tuple;
    println!("{} {}", three, six)
}

// Three months should be enough for everyone
const LE_MONTHS: [&str; 3] = ["January", "February", "March"];

fn le_months() {
    print!("Please enter month index: ");
    let mut month_index = String::new();
    io::stdin().read_line(&mut month_index).expect("read line month index");
    let month_index: usize = month_index.trim().parse().expect("parse month index");
    let month = LE_MONTHS[month_index];
    println!("{}", month);
}

fn why() {
    print!("Please enter a text: ");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("read line");
}

fn funni(x: i32) -> i32 {
    return x + 1;
}

fn five() -> isize { 5 }

fn plus_one(x: isize) -> isize { x + 1 }

fn le_function() {
    println!("five = {}", five());
    println!("plus_one = {}", plus_one(five()));
    let x = if five() == 5 { "ok" } else { "why" };
    println!("{}", x)
}

fn le_while() {
    let mut number = 3;
    while number > 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    for index in 1..4 {
        println!("{}", index)
    }
}

fn for_each() {
    let array = [0, 40, 80, 90];
    for element in array {
        println!("{}", element)
    }
}

fn fahrenheit_to_celsius(fahrenheit_temperature: f64) -> f64 {
    (fahrenheit_temperature - 32.0) * 5.0 / 9.0
}

fn fib(n: usize) -> usize {
    if n == 0 { 0 }
    else if n == 1 { 1 }
    else { fib(n - 2) + fib(n - 1) }
}