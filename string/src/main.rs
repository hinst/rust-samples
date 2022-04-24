mod pig_latin;

use pig_latin::get_pig_latin_sentence;

fn main() {
    println!("{}", get_pig_latin_sentence("A horse, an apple! My kingdom for a horse!"));
}