use std::collections::BinaryHeap;
use super::dynamic_array::DynamicArray;

pub struct PriorityQueue<T> {
    heap: BinaryHeap<T>,
}

impl<T> PriorityQueue<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
}

impl<T> DynamicArray<T> for PriorityQueue<T>
where
    T: Ord + Clone + Default,
{
    fn add(&mut self, item: T, _index: usize) {
        self.heap.push(item);
    }

    fn remove(&mut self, _index: usize) -> Option<T> {
        self.heap.pop()
    }

    fn size(&self) -> usize {
        self.heap.len()
    }

    fn get(&self, _index: usize) -> &T {
        self.heap.peek().expect("Heap is empty")
    }

    fn reset(&mut self) {
        self.heap.clear();
    }
}
