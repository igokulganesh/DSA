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

impl<T: Debug + Clone + Copy> DoublyList<T> {
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

    pub fn push_back(&mut self, data: T) {
        let new_node = Node::new(data);

        match self.tail.take() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
            Some(prev_tail) => {
                self.tail = Some(new_node.clone());
                prev_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(prev_tail.clone());
            }
        }
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(head) => {
                self.len -= 1;

                match head.borrow_mut().next.take() {
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                    Some(next_head) => {
                        next_head.borrow_mut().prev = None;
                        self.head = Some(next_head);
                    }
                };
                Some(head.borrow().data)
            }
        }
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
