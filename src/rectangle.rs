use crate::shapes::Shape;

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        (self.width * self.height) as f32
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::{Rectangle, Shape};

    #[test]
    fn area() {
        let sq = Rectangle::square(2);

        assert_eq!(4.0, sq.area())
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::square(2);
        let smaller = Rectangle::square(1);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle::square(2);
        let smaller = Rectangle::square(1);

        assert!(!smaller.can_hold(&larger));
    }
}
