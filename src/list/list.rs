pub struct Node<T: Copy> {
    pub data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Copy> Node<T> {
    pub fn new(val: T) -> Box<Node<T>> {
        Box::new(Self {
            data: val,
            next: None,
        })
    }

    pub fn set_next(&mut self, next: Option<Box<Node<T>>>) {
        self.next = next;
    }
}

pub struct List<T: Copy> {
    head: Option<Box<Node<T>>>,
}

impl<T: Copy> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, val: T) {
        match self.head.take() {
            None => {
                self.head = Some(Node::new(val));
            }
            Some(head) => {
                let mut new_head = Node::new(val);
                new_head.set_next(Some(head));
                self.head = Some(new_head);
            }
        }
    }

    pub fn len(&self) -> usize {
        let mut head = &self.head;
        let mut len = 0;

        while let Some(node) = head {
            len += 1;
            head = &node.next;
        }

        len
    }
}

impl<T: Copy> From<Vec<T>> for List<T> {
    fn from(value: Vec<T>) -> Self {
        let mut list = Self::new();
        for i in value {
            list.push_front(i);
        }
        list
    }
}

impl<T: Copy> From<List<T>> for Vec<T> {
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
