use std::ops::Add;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

pub struct Human;

pub trait Pilot {
	fn fly(&self);
}

pub trait Wizard {
	fn fly(&self);
}

impl Pilot for Human {
	fn fly(&self) {
		println!("This is your captain speaking.");
	}
}

impl Wizard for Human {
	fn fly(&self) {
		println!("Up!");
	}
}

impl Human {
	pub fn fly(&self) {
		println!("*waving arms furiously*");
	}
}

pub struct Wrapper(pub Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
