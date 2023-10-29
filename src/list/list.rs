use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T: Copy + Display + Debug> {
    pub data: T,
    pub next: Link<T>,
}

impl<T: Copy + Display + Debug> Node<T> {
    pub fn new(data: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { data, next: None }))
    }
}

pub struct List<T: Copy + Display + Debug> {
    head: Link<T>,
}

impl<T: Copy + Display + Debug> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Node::new(data);

        match self.head.take() {
            None => self.head = Some(new_node),
            Some(prev_head) => {
                new_node.borrow_mut().next = Some(prev_head);
                self.head = Some(new_node);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Node::new(data);

        let mut current = self.head.clone();

        while let Some(node) = current {
            if node.borrow().next.is_none() {
                node.borrow_mut().next = Some(new_node);
                return;
            }
            current = node.borrow().next.clone();
        }

        // If the list is empty, add the new node as the head
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(head) => {
                if head.borrow().next.is_some() {
                    self.head = head.borrow().next.clone();
                } else {
                    self.head = None;
                }
                Some(head.borrow().data)
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let mut current = self.head.clone();
        let mut prev: Link<T> = None;

        while let Some(node) = current.clone() {
            if node.borrow().next.is_none() {
                if prev.is_some() {
                    prev.unwrap().borrow_mut().next = None;
                } else {
                    self.head = None;
                }
                return Some(node.borrow().data);
            }
            prev = current.clone();
            current = node.borrow().next.clone();
        }
        None
    }

    pub fn len(&mut self) -> usize {
        let mut len: usize = 0;

        let mut current = self.head.clone();

        while let Some(node) = current {
            len += 1;
            if node.borrow().next.is_none() {
                break;
            }
            current = node.borrow().next.clone();
        }

        len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<T: Copy + Display + Debug> From<Vec<T>> for List<T> {
    fn from(value: Vec<T>) -> Self {
        let mut list = Self::new();
        for i in value {
            list.push_front(i);
        }
        list
    }
}

impl<T: Copy + Display + Debug> From<List<T>> for Vec<T> {
    fn from(list: List<T>) -> Self {
        let mut vector = Vec::new();

        let mut head = list.head.clone();

        while let Some(val) = head {
            vector.push(val.borrow().data);
            head = val.borrow_mut().next.clone();
        }

        vector
    }
}
