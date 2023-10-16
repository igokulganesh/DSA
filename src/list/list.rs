use std::fmt::{Debug, Display};

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T: Copy + Display + Debug> {
    pub data: T,
    pub next: Link<T>,
}

impl<T: Copy + Display + Debug> Node<T> {
    pub fn new(data: T) -> Box<Node<T>> {
        Box::new(Node { data, next: None })
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
        let mut new_node = Node::new(data);

        match self.head.take() {
            None => self.head = Some(new_node),
            Some(prev_head) => {
                new_node.next = Some(prev_head);
                self.head = Some(new_node);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Node::new(data);

        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(new_node);
                return;
            }
            current = &mut node.next;
        }

        // If the list is empty, add the new node as the head
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn len(&mut self) -> usize {
        let mut len: usize = 0;

        let mut current = &mut self.head;

        while let Some(node) = current {
            len += 1;
            if node.next.is_none() {
                break;
            }
            current = &mut node.next;
            println!("len: {}", len);
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

        let mut head = &list.head;

        while let Some(val) = head {
            vector.push(val.data);
            head = &val.next;
        }

        vector
    }
}
