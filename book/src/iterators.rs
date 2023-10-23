pub struct Fibonacci {
    curr: u32,
    next: u32,
    max_depth: u32,
    cur_depth: u32,
}

impl Fibonacci {
    pub fn new(max_depth: u32) -> Self {
        Self {
            curr: 1,
            next: 1,
            max_depth,
            cur_depth: 0,
        }
    }

    pub fn sum(self) -> u32 {
        self.fold(0, |sum, a| sum + a)
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_depth >= self.max_depth {
            return None;
        }

        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        self.cur_depth += 1;
        Some(current)
    }
}

#[derive(Debug, PartialEq)]
pub struct Shoe {
    size: u32,
    style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looping() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let mut ret: Vec<i32> = vec![];

        for val in v1.iter() {
            ret.push(*val);
        }

        assert_eq!(ret, vec![1, 2, 3]);
    }

    #[test]
    fn test_next() {
        let mut v1_iter = 1..4;

        assert_eq!(v1_iter.next(), Some(1));
        assert_eq!(v1_iter.next(), Some(2));
        assert_eq!(v1_iter.next(), Some(3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn test_custom_iterator() {
        let mut fib = Fibonacci::new(5);

        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), None);
    }

    #[test]
    fn test_custom_iterator_sum() {
        assert_eq!(Fibonacci::new(0).sum(), 0);
        assert_eq!(Fibonacci::new(5).sum(), 12);
    }

    #[test]
    fn test_map_on_iterator() {
        let iter = 0..4;
        let v: Vec<_> = iter.map(|x| x + 1).collect();

        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_filter_on_vector() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ],
        )
    }
}
