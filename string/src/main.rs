mod pig_latin;

use pig_latin::get_pig_latin_sentence;

fn main() {
    println!("{}", get_pig_latin_sentence("Hello, world apple!"));
}