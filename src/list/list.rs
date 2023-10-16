type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T: Copy> {
    pub data: T,
    pub next: Link<T>,
}

impl<T: Copy> Node<T> {
    pub fn new(data: T) -> Box<Node<T>> {
        Box::new(Node { data, next: None })
    }
}

pub struct List<T: Copy> {
    head: Link<T>,
    len: usize,
}

impl<T: Copy> List<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
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
        self.len += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Node::new(data);
        self.len += 1;

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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
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
