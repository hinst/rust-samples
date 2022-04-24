#[derive(Debug)]
enum RectangleVersion {
    V1
}

#[derive(Debug)]
struct Rectangle {
    version: RectangleVersion,
    width: f32,
    height: f32,
}

const DEFAULT_RECTANGLE: Rectangle = Rectangle {
    version: RectangleVersion::V1,
    width: 0.0,
    height: 0.0,
};

impl Rectangle {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }

    fn square(width: f32) -> Rectangle {
        return Rectangle {
            width,
            height: width,
            ..DEFAULT_RECTANGLE
        };
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 80.0,
        height: 25.0,
        ..DEFAULT_RECTANGLE
    };
    dbg!(rectangle.width);
    println!("{:?} area {:?}", rectangle, rectangle.area());

    println!(
        "{}",
        Rectangle { width: 80.0, height: 25.0, ..DEFAULT_RECTANGLE }
            .can_hold(&Rectangle { width: 40.0, height: 25.0, ..DEFAULT_RECTANGLE })
    );

    println!("{:?}", Rectangle::square(1.0));

    let a: &str = "a";
}