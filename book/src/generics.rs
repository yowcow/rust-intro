use std::cmp::PartialOrd;

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if *item > *largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod generics_tests {
    use super::largest;

    #[test]
    fn largest_i32() {
        let list: Vec<i32> = vec![34, 50, 25, 100, 65];
        let val = largest(&list);

        assert_eq!(*val, 100)
    }

    #[test]
    fn largest_char() {
        let list: Vec<char> = vec!['y', 'm', 'a', 'q'];
        let val = largest(&list);

        assert_eq!(*val, 'y')
    }
}

#[allow(dead_code)]
pub struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

#[allow(dead_code)]
impl Point<i32> {
    fn distance_from_origin(&self) -> f32 {
        ((self.x as f32).powi(2) + (self.y as f32).powi(2)).sqrt()
    }
}

#[allow(dead_code)]
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod generics_struct_tests {
    use super::Point;

    #[test]
    fn generics_struct() {
        let p1 = Point { x: 5, y: 5 };
        let p2: Point<f32> = Point { x: 5.0, y: 5.0 };

        assert_eq!(p1.x, 5);
        assert_eq!(p1.y, 5);
        assert_eq!(p1.distance_from_origin(), 50_f32.sqrt());

        assert_eq!(*p2.x(), 5.0);
        assert_eq!(p2.y(), &5.0);
        assert_eq!(p2.distance_from_origin(), 50_f32.sqrt());
    }
}

#[cfg(test)]
mod generics_enum_tests {
    #[derive(Debug)]
    enum Opt<T> {
        None,
        Some(T),
    }

    #[test]
    fn generics_enum() {
        let v: Vec<Opt<bool>> = vec![Opt::None, Opt::Some(true), Opt::Some(false)];

        let opt0 = v.get(0).unwrap();
        match opt0 {
            Opt::None => assert!(true),
            other => panic!("item 0 should be None but got {:?}", other),
        }

        let opt1 = v.get(1).unwrap();
        match opt1 {
            Opt::Some(some) => assert_eq!(*some, true),
            other => panic!("item 1 should be Some(true) but got {:?}", other),
        }

        let opt2 = v.get(2).unwrap();
        match opt2 {
            Opt::Some(some) => assert_eq!(*some, false),
            other => panic!("item 2 should be Some(false) but got {:?}", other),
        }
    }
}

#[cfg(test)]
mod multiple_trait_bounds_tests {
    use std::fmt;
    use std::ops::Add;

    pub fn do_work<T: fmt::Display + Add>(item1: T, item2: T) -> String
    where
        <T as Add>::Output: fmt::Display,
    {
        format!("{}", item1.add(item2))
    }

    struct Item(String);

    impl fmt::Display for Item {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Item({})", self.0)
        }
    }

    impl Add for Item {
        type Output = Item;

        fn add(self, rhs: Self) -> Self::Output {
            let mut s = self.0.to_string();
            s.push_str("+");
            s.push_str(&rhs.0);
            Item(s)
        }
    }

    #[test]
    fn test_do_work() {
        let item1 = Item(String::from("Hello"));
        let item2 = Item(String::from("world"));
        let result = do_work(item1, item2);

        assert_eq!(result, "Item(Hello+world)");
    }
}
