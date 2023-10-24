use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_child_to_node() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        {
            assert_eq!(Rc::strong_count(&leaf), 1);
            assert_eq!(Rc::weak_count(&leaf), 0);

            let p = leaf.parent.borrow();
            match p.upgrade() {
                None => assert!(true),
                _ => panic!("expected parent to be None but got some"),
            }
        }

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            {
                assert_eq!(Rc::strong_count(&leaf), 2);
                assert_eq!(Rc::weak_count(&leaf), 0);
                assert_eq!(Rc::strong_count(&branch), 1);
                assert_eq!(Rc::weak_count(&branch), 1);

                let p = leaf.parent.borrow();
                match p.upgrade() {
                    Some(node) => assert_eq!(node.value, 5),
                    _ => panic!("expected Some but got None"),
                }
            };
        }

        {
            assert_eq!(Rc::strong_count(&leaf), 1);
            assert_eq!(Rc::weak_count(&leaf), 0);

            let p = leaf.parent.borrow();
            match p.upgrade() {
                None => assert!(true),
                _ => panic!("expected parent to be None but got some"),
            }
        }
    }
}
