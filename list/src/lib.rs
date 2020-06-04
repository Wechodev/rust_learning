use std::rc::Rc;
use std::cell::RefCell;

type NodeType<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    value: T,
    pre: Option<NodeType<T>>,
    next: Option<NodeType<T>>,
}

struct List<T> {
    head: Option<NodeType<T>>,
    tail: Option<NodeType<T>>,
}

impl <T: Clone> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }

    fn push_back(&mut self, val: T) {
        let node = Rc::new(RefCell::new(
            Node::<T> {
                value: val,
                pre: self.tail.clone(),
                next: None
            }
        ));
        match self.tail.clone() {
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node.clone());
            },

            Some(ref tail) => {
                tail.borrow_mut().next = Some(node.clone());
                self.tail = Some(node.clone());
            }
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        match self.tail.clone() {
            None => None,
            Some(tail) => {
                let pre = tail.borrow().pre.clone();
                match pre {
                    None => {
                        self.head = None;
                        self.tail = None;
                    },

                    Some(ref p) => {
                        p.borrow_mut().next = None;
                        self.tail = pre.clone();
                    }
                }

                return  Some(tail.borrow().value.clone());
            }
        }
    }
}

#[test]
fn test_list() {
    let mut l = List::new();
    l.push_back(1);
    l.push_back(2);

    assert_eq!(l.pop_back(), Some(2));
    assert_eq!(l.pop_back(), Some(1));
    assert_eq!(l.pop_back(), None);

    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    assert_eq!(l.pop_back(), Some(3));
}
