use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum LeakList {
    Cons(i32, RefCell<Rc<LeakList>>),
    Nil,
}

impl LeakList {
    pub fn tail(&self) -> Option<&RefCell<Rc<LeakList>>> {
        match self {
            LeakList::Cons(_, item) => Some(item),
            LeakList::Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LeakList::{Cons, Nil};
    use super::*;

    #[test]
    fn test_leak() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        {
            assert_eq!(Rc::strong_count(&a), 1);
            let item = a.tail().unwrap().borrow();
            assert_eq!(*item.as_ref(), Nil);
        }

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        {
            assert_eq!(Rc::strong_count(&a), 2);
            assert_eq!(Rc::strong_count(&b), 1);
        }

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        {
            assert_eq!(Rc::strong_count(&a), 2); // 'b' references 'a'
            assert_eq!(Rc::strong_count(&b), 2); // now 'a' also references 'b'
        }
    }
}
