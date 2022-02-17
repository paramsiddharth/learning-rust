fn main() {
    let rect_1 = Rectangle {
        height: 1 as f32,
        width: 2 as f32
    };

    println!("Permieter: {}", rect_1.perimeter());
    println!("Area: {}", rect_1.area());

    let rect_2 = Rectangle {
        height: 3 as f32,
        width: 4 as f32
    };

    println!("Rectangle 1 inside 2: {}", rect_2.can_hold(&rect_1));
    println!("Rectangle 2 inside 1: {}", rect_1.can_hold(&rect_2));
}

struct Rectangle {
    width: f32,
    height: f32
}

impl Rectangle {
    fn perimeter(&self) -> f32 {
        (2 as f32) * (self.height + self.width)
    }

    fn area(&self) -> f32 {
        self.height * self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height >= rect.height && self.width >= rect.width
    }
}