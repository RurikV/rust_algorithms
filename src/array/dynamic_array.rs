pub trait DynamicArray<T> {
    fn add(&mut self, item: T, index: usize);
    fn remove(&mut self, index: usize) -> Option<T>;
    fn size(&self) -> usize;
    fn get(&self, index: usize) -> &T;
    fn reset(&mut self) {
        while self.size() > 0 {
            self.remove(0);
        }
    }
}

