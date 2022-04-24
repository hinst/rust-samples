enum Coin {
    Cent1,
    Cent10,
    Cent50,
}

fn value_in_cents(coin: Coin) -> isize {
    match coin {
        Coin::Cent1 => {
            println!("penny");
            1
        },
        Coin::Cent10 => 10,
        Coin::Cent50 => 50,
        _others => 0
    }
}

fn increase_optional_isize(value: Option<isize>) -> Option<isize> {
    return match value {
        None => None,
        Some(value) => Some(value + 1)
    }
}

fn main() {
    println!("STARTING...");
    println!("{:?} {:?}", increase_optional_isize(Some(12)), increase_optional_isize(None));
}
