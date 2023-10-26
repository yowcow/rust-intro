#[cfg(test)]
mod placeholder_tests {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add<Point> for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    impl Add for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Millimeters) -> Millimeters {
            Millimeters(self.0 + other.0)
        }
    }

    #[test]
    fn test_add_points() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let result = p1.add(p2);

        assert_eq!(result, Point { x: 3, y: 3 });
    }

    #[test]
    fn test_add_meters_to_millimeters() {
        let mm = Millimeters(500);
        let m = Meters(1);
        let result = mm.add(m);

        assert_eq!(result, Millimeters(1500));
    }

    #[test]
    fn test_add_millimeters_to_millimeters() {
        let mm1 = Millimeters(500);
        let mm2 = Millimeters(1000);
        let result = mm1.add(mm2);

        assert_eq!(result, Millimeters(1500));
    }
}

#[cfg(test)]
mod disambiguation_tests {
    pub trait Pilot {
        fn fly(&self) -> &str;
    }

    pub trait Wizard {
        fn fly(&self) -> &str;
    }

    pub struct Human;

    impl Pilot for Human {
        fn fly(&self) -> &str {
            "This is your captain speaking."
        }
    }

    impl Wizard for Human {
        fn fly(&self) -> &str {
            "Up!"
        }
    }

    impl Human {
        pub fn fly(&self) -> &str {
            "*waving arms furiously*"
        }
    }

    pub trait Animal {
        fn baby_name() -> String;
    }

    pub struct Dog;

    impl Dog {
        pub fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    #[test]
    fn test_fly() {
        let person = Human;

        assert_eq!(person.fly(), "*waving arms furiously*");
        assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
        assert_eq!(Wizard::fly(&person), "Up!");
        assert_eq!(Human::fly(&person), "*waving arms furiously*");
    }

    #[test]
    fn test_baby_name() {
        assert_eq!(Dog::baby_name(), "Spot");
        assert_eq!(<Dog as Animal>::baby_name(), "puppy");
    }
}

#[cfg(test)]
mod supertrait_tests {
    use std::fmt;

    pub trait OutlinePrint: fmt::Display {
        fn outline_print(&self) -> Vec<String> {
            let output = self.to_string();
            let len = output.len();

            let mut result = vec![];

            result.push(format!("{}", "*".repeat(len + 4)));
            result.push(format!("*{}*", " ".repeat(len + 2)));
            result.push(format!("* {} *", output));
            result.push(format!("*{}*", " ".repeat(len + 2)));
            result.push(format!("{}", "*".repeat(len + 4)));

            result
        }
    }

    pub struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    #[test]
    fn test_outline() {
        let p = Point { x: 1, y: 2 };

        assert_eq!(
            p.outline_print(),
            vec![
                "**********",
                "*        *",
                "* (1, 2) *",
                "*        *",
                "**********",
            ]
        )
    }
}

#[cfg(test)]
mod newtype_tests {
    use std::{fmt, ops};

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    impl ops::Deref for Wrapper {
        type Target = Vec<String>;

        fn deref(&self) -> &Vec<String> {
            &self.0
        }
    }

    #[test]
    fn test_wrapper_fmt() {
        let w = Wrapper(vec![String::from("Hello"), String::from("world")]);

        assert_eq!(format!("{}", w), "[Hello, world]")
    }

    #[test]
    fn test_deref_wrapper() {
        let w = Wrapper(vec![String::from("Hello"), String::from("world")]);
        let rw = &w;

        assert_eq!(rw.join(", "), "Hello, world");
    }
}
