use rand::Rng;

pub struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    pub fn can_hold(&self, rectangle: &Rectangle) -> bool {
        return rectangle.width < self.width && rectangle.height < self.height;
    }

    pub fn get_area(&self) -> usize {
        return self.width * self.height
    }

    pub fn create_random(size_limit: usize) -> Rectangle {
        let mut random = rand::thread_rng();
        return Rectangle {
            width: random.gen_range(0..size_limit),
            height: random.gen_range(0..size_limit),
        };
    }

    pub fn get_double(&self) -> Result<Rectangle, String> {
        let new_width = self.width.checked_add(self.width);
        let new_height = self.height.checked_add(self.height);
        match new_width {
            Some(new_width) => {
                match new_height {
                    Some(new_height) => {
                        return Ok(Rectangle{ width: new_width, height: new_height })
                    },
                    None => {
                        return Err(format!("Height overflowed from {}", self.height))
                    }
                }
            },
            None => {
                return Err(format!("Width overflowed from {}", self.width))
            }
        }
    }
}

impl ToString for Rectangle {
    fn to_string(&self) -> String {
        let mut text = String::from("[");
        text.push_str(self.width.to_string().as_str());
        text.push(' ');
        text.push_str(self.height.to_string().as_str());
        text.push(']');
        return text;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_can_hold() {
        let large_rectangle = Rectangle {
            width: 8,
            height: 7
        };
        let small_rectangle = Rectangle {
            width: 5,
            height: 1
        };
        assert!(large_rectangle.can_hold(&small_rectangle));
    }

    #[test]
    fn rectangle_cannot_hold() {
        let small_rectangle = Rectangle {
            width: 3,
            height: 5
        };
        let large_rectangle = Rectangle {
            width: 9,
            height: 10,
        };
        assert!(!small_rectangle.can_hold(&large_rectangle));
    }

    #[test]
    fn rectangle_get_area() {
        let rectangle = Rectangle {
            width: 4,
            height: 3
        };
        assert_eq!(4 * 3, rectangle.get_area())
    }

    #[test]
    fn rectangle_to_string() {
        let rectangle = Rectangle {
            width: 15,
            height: 4
        };
        let string = rectangle.to_string();
        assert!(string.contains("15"), "Expecting 15 in \"{}\"", string);
        assert!(string.contains("4"), "Expecting 4 in \"{}\"", string);
    }

    #[test]
    #[should_panic(expected = "cannot sample empty range")]
    fn rectangle_create_random() {
        Rectangle::create_random(0);
    }

    #[test]
    #[ignore]
    fn rectangle_get_double() -> Result<(), String> {
        let rectangle = Rectangle {
            width: 128,
            height: 128,
        };
        let double_rectangle = rectangle.get_double();
        match double_rectangle {
            Ok(_) => return Ok(()),
            Err(message) => return Err(message)
        }
    }

    #[test]
    fn rectangle_get_double_too_large() {
        let rectangle = Rectangle {
            width: 18446744073709551613,
            height: 18446744073709551614,
        };
        let double_rectangle = rectangle.get_double();
        assert!(double_rectangle.is_err())
    }
}
