const CONSONANTS: &str = "bcdfghjklmnpqrstvwxz";

fn get_lower_case_char(c: char) -> char {
    let lower_case = c.to_lowercase().next();
    match lower_case {
        Some(c) => c,
        _ => c,
    }
}

fn get_upper_case_char(c: char) -> char {
    let upper_case = c.to_uppercase().next();
    match upper_case {
        Some(c) => c,
        _ => c,
    }
}

fn is_consonant(c: char) -> bool {
    CONSONANTS.contains(get_lower_case_char(c))
}

const VOWELS: &str = "aeiouy";

fn is_vowel(c: char) -> bool {
    VOWELS.contains(get_lower_case_char(c))
}

fn get_pig_latin_word(word: &str) -> String {
    let mut output = String::new();
    let mut first_character: Option<char> = None;
    for character in word.chars() {
        if let None = first_character {
            first_character = Some(character);
            if is_vowel(character) {
                output.push(character);
            }
        } else {
            let mut pushed_upper_case = false;
            if let Some(first_character) = first_character {
                if output.len() == 0 && first_character.is_uppercase() {
                    output.push(get_upper_case_char(character));
                    pushed_upper_case = true;
                }
            }
            if !pushed_upper_case {
                output.push(character);
            }
        }
    }
    if let Some(first_character) = first_character {
        if is_consonant(first_character) {
            output.push('-');
            output.push(first_character);
            output.push_str("ay");
        } else {
            output.push_str("-hay");
        }
    }
    return output;
}

fn flush_word(output: &mut String, word: &str) {
    let converted_word = get_pig_latin_word(&word);
    output.push_str(&converted_word);
}

pub fn get_pig_latin_sentence(source: &str) -> String {
    let mut current_word = String::new();
    let mut output = String::new();
    for character in source.chars() {
        if is_vowel(character) || is_consonant(character) {
            current_word.push(character);
        } else {
            flush_word(&mut output, &current_word);
            output.push(character);
            current_word = String::new();
        }
    };
    return output;
}