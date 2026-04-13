pub struct Container<T: Clone + Send> {
    pub items: Vec<T>,
}

impl<T: Clone + Send> Container<T> {
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
