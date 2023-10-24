use std::rc::Rc;

pub enum List {
    Cons(i32, Rc<List>),
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
    use super::List::{Cons, Nil};
    use super::*;

    #[test]
    fn test_rc() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

        assert_eq!(Rc::strong_count(&a), 1);

        let b = Cons(3, Rc::clone(&a));

        assert_eq!(Rc::strong_count(&a), 2);

        let c = Cons(4, Rc::clone(&a));

        assert_eq!(Rc::strong_count(&a), 3);

        assert_eq!(get_cons_list(b), vec![3, 5, 10]);

        assert_eq!(Rc::strong_count(&a), 2);

        assert_eq!(get_cons_list(c), vec![4, 5, 10]);

        assert_eq!(Rc::strong_count(&a), 1);
    }
}
