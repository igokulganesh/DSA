pub struct Stack<T: Copy> {
    arr: Vec<T>,
}

impl<T: Copy> Stack<T> {
    pub fn new() -> Self {
        Self { arr: Vec::new() }
    }

    pub fn push(&mut self, val: T) {
        self.arr.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.arr.pop()
    }

    pub fn size(&mut self) -> usize {
        self.arr.len()
    }
}
