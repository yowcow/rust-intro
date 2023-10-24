pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            max,
            value: 0,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod refcell_tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(msg.to_string())
        }
    }

    #[test]
    fn test_limit_tracker_at_quota_74_pct() {
        let m = MockMessenger::new();
        let mut lt = LimitTracker::new(&m, 100);
        lt.set_value(74);

        assert_eq!(m.sent_messages.borrow().len(), 0);

        lt.set_value(75);
        lt.set_value(90);
        lt.set_value(100);

        assert_eq!(
            m.sent_messages.borrow().to_vec(),
            vec![
                String::from("Warning: You've used up over 75% of your quota!"),
                String::from("Urgent warning: You've used up over 90% of your quota!"),
                String::from("Error: You are over your quota!"),
            ]
        );
    }
}

#[derive(Debug)]
pub enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::cell::RefCell;
use std::rc::Rc;
use List::Cons;

pub fn get_cons_list(list: List) -> Vec<i32> {
    let mut listref = &list;
    let mut ret = vec![];

    loop {
        if let Cons(v, next) = listref {
            ret.push(*v.borrow());
            listref = next;
        } else {
            break;
        }
    }

    ret
}

#[cfg(test)]
mod rc_refcell_tests {
    use super::List::{Cons, Nil};
    use super::*;

    #[test]
    fn test_cons_list() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(
            Rc::clone(&value),
            Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil))),
        ));
        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        assert_eq!(get_cons_list(b), vec![3, 15, 10]);
        assert_eq!(get_cons_list(c), vec![4, 15, 10]);
    }
}
