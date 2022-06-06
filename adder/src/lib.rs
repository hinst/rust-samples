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
}
