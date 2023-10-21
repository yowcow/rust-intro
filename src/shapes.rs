pub trait Shape {
    fn area(&self) -> f32;
    fn can_hold(&self, other: &Self) -> bool;
}

//TODO
//#[cfg(test)]
//mod shape_tests {
//    use super::*;
//    use crate::rectangle::Rectangle;
//    use crate::triangle::Triangle;
//
//    #[test]
//    fn test_sum_areas() {
//        let rect = Rectangle::square(4);
//        let trig = Triangle { base: 2, height: 2 };
//
//        let mut v: Vec<Box<dyn Shape>> = Vec::new();
//        v.push(Box::new(rect));
//        v.push(Box::new(trig));
//    }
//}
