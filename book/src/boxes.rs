pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::Cons;

pub fn get_cons_list(list: List) -> Vec<i32> {
    let mut listref = &list;
    let mut ret = vec![];

    loop {
        if let Cons(val, next) = listref {
            ret.push(*val);
            listref = next;
        } else {
            break;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use List::{Cons, Nil};

    #[test]
    fn test_cons_list() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        let result = get_cons_list(list);

        assert_eq!(result, vec![1, 2, 3]);
    }
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod mybox_tests {
    use super::MyBox;
    use std::ops::Deref;

    #[test]
    fn deref_mybox() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(x, 5);
        assert_eq!(*y, 5);
        assert_eq!(*(y.deref()), 5);
    }
}
