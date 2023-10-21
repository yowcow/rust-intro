use crate::shapes::Shape;

pub struct Triangle {
    base: u32,
    height: u32,
}

impl Shape for Triangle {
    fn area(&self) -> f32 {
        self.base as f32 * self.height as f32 / 2.0
    }

    fn can_hold(&self, other: &Triangle) -> bool {
        self.base > other.base && self.height > other.height
    }
}

#[cfg(test)]
mod triangle_tests {
    use super::{Shape, Triangle};

    #[test]
    fn area() {
        let tr = Triangle { base: 3, height: 3 };

        assert_eq!(4.5, tr.area());
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Triangle { base: 2, height: 2 };
        let smaller = Triangle { base: 1, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Triangle { base: 2, height: 2 };
        let smaller = Triangle { base: 1, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}
