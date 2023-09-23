use std::collections::VecDeque;

pub struct Queue<T> {
    queue: VecDeque<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { queue: VecDeque::new() }
    }

    pub fn push(&mut self, val: T) {
        self.queue.push_front(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop_back()
    }
}
