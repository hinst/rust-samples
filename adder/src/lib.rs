struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        return rectangle.width < self.width && rectangle.height < self.height;
    }

    fn get_area(&self) -> usize {
        return self.width * self.height
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
}
