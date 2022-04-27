pub struct Stack<T> {
    max_size: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(max: usize) -> Self {
        Stack {
            items: Vec::with_capacity(max),
            max_size: max,
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn push(&mut self, item: T) -> bool {
        if self.items.len() < self.max_size {
            self.items.push(item);
            true
        } else {
            false
        }
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn length(&self) -> usize {
        self.items.len()
    }
    pub fn top(&self) -> Option<&T> {
        self.items.last()
    }
}
