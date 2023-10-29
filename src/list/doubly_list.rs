use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

type Link<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
    pub data: T,
    pub prev: Option<Link<T>>,
    pub next: Option<Link<T>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Link<T> {
        Rc::new(RefCell::new(Node {
            data,
            prev: None,
            next: None,
        }))
    }
}

pub struct DoublyList<T> {
    pub head: Option<Link<T>>,
    pub tail: Option<Link<T>>,
    pub len: usize,
}

impl<T: Debug> DoublyList<T> {
    pub fn new() -> Self {
        DoublyList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Node::new(data);

        match self.head.take() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
            Some(head) => {
                head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(head.clone());
                self.head = Some(new_node);
            }
        };
        self.len += 1;
    }
}

impl<T: Copy + Debug> From<DoublyList<T>> for Vec<T> {
    fn from(list: DoublyList<T>) -> Self {
        let mut vector = Vec::new();

        let mut head = list.head.clone();

        while let Some(val) = head {
            vector.push(val.borrow().data);
            head = val.borrow_mut().next.clone();
        }

        vector
    }
}
