fn main() {
    let mut text = String::from("HelloWoof");
    let firstWord = first_word(&text);
    println!("{}", firstWord);
    let user1 = User {
        email: String::from("a@a"),
        username: String::from("umad"),
        active: true,
        sign_in_count: 69
    };
}

type float = f32;

fn extend_text(text: &mut String) {
    text.push_str(" sir");
}

fn dangle() -> String {
    let s = String::from("Hello");
    return s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    return &s[..];
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(u8, u8, u8);
struct Point(float, float, float);

const black: Color = Color(0, 0, 0);
const origin: Point = Point(0.0, 0.0, 0.0);