pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn new() -> Self {
        AverageCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        let cur_count = self.list.len() as f64;
        let cur_sum = self.average * cur_count;

        self.list.push(value);
        self.average = (cur_sum + value as f64) / (cur_count + 1.0);
    }

    pub fn remove(&mut self) -> Option<i32> {
        let cur_count = self.list.len() as f64;
        let cur_sum = self.average * cur_count;

        match self.list.pop() {
            None => None,
            Some(value) => {
                self.average = if cur_count > 1.0 {
                    (cur_sum - (value as f64)) / (cur_count - 1.0)
                } else {
                    0.0
                };
                Some(value)
            }
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_collection() {
        let mut col = AverageCollection::new();
        col.add(1);
        assert_eq!(col.average(), 1.0);

        col.add(2);
        assert_eq!(col.average(), 1.5);

        col.add(3);
        assert_eq!(col.average(), 2.0);

        {
            let val = col.remove().unwrap();
            assert_eq!(val, 3);
            assert_eq!(col.average(), 1.5);
        }

        {
            let val = col.remove().unwrap();
            assert_eq!(val, 2);
            assert_eq!(col.average(), 1.0);
        }

        {
            let val = col.remove().unwrap();
            assert_eq!(val, 1);
            assert_eq!(col.average(), 0.0);
        }

        {
            match col.remove() {
                None => assert!(true),
                Some(val) => panic!("expected None but got Some({})", val),
            }
        }
    }
}

pub trait Thing {
    fn name(&self) -> &str;
}

pub trait Shape: Thing {
    fn area(&self) -> f64;
}

pub struct Triangle {
    base: u32,
    height: u32,
}

impl Triangle {
    pub fn new(base: u32, height: u32) -> Triangle {
        Triangle { base, height }
    }

    pub fn can_hold(&self, other: &Self) -> bool {
        self.base > other.base && self.height > other.height
    }
}

impl Thing for Triangle {
    fn name(&self) -> &str {
        "I'm a triangle!"
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        (self.base * self.height) as f64 / 2.0
    }
}

pub struct Square {
    side: u32,
}

impl Square {
    pub fn new(side: u32) -> Square {
        Square { side }
    }

    pub fn can_hold(&self, other: &Self) -> bool {
        self.side > other.side
    }
}

impl Thing for Square {
    fn name(&self) -> &str {
        "I'm a square!"
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        (self.side * self.side) as f64
    }
}

#[cfg(test)]
mod shape_tests {
    use super::*;

    #[test]
    fn test_triangle() {
        let item1 = Triangle::new(5, 3);
        let item2 = Triangle::new(5, 2);
        let item3 = Triangle::new(4, 3);
        let item4 = Triangle::new(4, 2);

        assert_eq!(item1.area(), 7.5);
        assert_eq!(item2.area(), 5.0);
        assert_eq!(item3.area(), 6.0);
        assert_eq!(item4.area(), 4.0);

        assert_eq!(item1.can_hold(&item2), false);
        assert_eq!(item1.can_hold(&item3), false);
        assert_eq!(item1.can_hold(&item4), true);
    }

    #[test]
    fn test_square() {
        let item1 = Square::new(5);
        let item2 = Square::new(4);

        assert_eq!(item1.area(), 25.0);
        assert_eq!(item2.area(), 16.0);

        assert_eq!(item1.can_hold(&item1), false);
        assert_eq!(item1.can_hold(&item2), true);
    }

    #[test]
    fn test_shape_vector() {
        let shapes: Vec<Box<dyn Shape>> =
            vec![Box::new(Triangle::new(5, 3)), Box::new(Square::new(4))];

        let sum: f64 = shapes.iter().map(|s| s.area()).sum();

        assert_eq!(sum, 7.5 + 16.0);

        let mut names = vec![];
        shapes.iter().for_each(|s| names.push(s.name()));

        assert_eq!(names, vec!["I'm a triangle!", "I'm a square!"]);
    }
}

pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post {
    pub fn new() -> Post {
        return Post {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        };
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[cfg(test)]
mod blog_tests {
    use super::*;

    #[test]
    fn test_post() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());

        post.reject();
        assert_eq!("", post.content());
    }
}
