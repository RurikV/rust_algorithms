//use std::ops::{Deref, DerefMut};
use super::dynamic_array::DynamicArray;
use super::single_array::SingleArray;
use super::vector_array::VectorArray;

pub struct MatrixArray<T> {
    size: usize,
    vector: usize,
    array: SingleArray<VectorArray<T>>,
}

impl<T> MatrixArray<T>
where
    T: Clone + Default,
{
    fn new(vector: usize) -> Self {
        Self {
            size: 0,
            vector,
            array: SingleArray::new(),
        }
    }

    pub fn with_default() -> Self {
        Self::new(10)
    }
}

impl<T> DynamicArray<T> for MatrixArray<T>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        println!("Adding item at index: {}", index);
        if index > self.size {
            panic!("Index out of bounds");
        }
        let block_index = index / self.vector;
        let position = index % self.vector;
        println!("Block index: {}, Position: {}", block_index, position);

        while block_index >= self.array.size() {
            println!("Adding new block at index: {}", block_index);
            self.array
                .add(VectorArray::new(self.vector), self.array.size());
        }

        if self.array[block_index].size() < position {
            self.array[block_index].add(item, 0); 
        }
        else {
            self.array[block_index].add(item, position);
        }
        self.size += 1;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        println!("Removing item at index: {}", index);
        if index >= self.size {
            return None;
        }
        let mut block_index = index / self.vector;
        let position = index % self.vector;
        println!("Block index: {}, Position: {}", block_index, position);

        if block_index >= self.array.size() {
            return None;
        }
        let block_size = self.array[block_index].size();
        if position >= block_size {
            if block_index == 0 {
                return None;
            }
            block_index -= 1;
        }
        let removed_item = self.array[block_index].remove(position);
        self.size -= 1;

        // Collect items to be shifted
        let mut items_to_shift = Vec::new();
        for i in block_index + 1..self.array.size() {
            if let Some(item) = self.array[i].remove(0) {
                items_to_shift.push((i - 1, item));
            } else {
                break;
            }
        }

        // Shift collected items to the previous blocks
        for (block_index, item) in items_to_shift {
            let new_position = self.array[block_index].size();
            self.array[block_index].add(item, new_position);
        }

        // Remove last block if it's empty
        if self.array.size() > 0 && self.array[self.array.size() - 1].size() == 0 {
            println!("Removing empty block at index: {}", self.array.size() - 1);
            self.array.remove(self.array.size() - 1);
        }

        removed_item
    }

    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, index: usize) -> &T {
        println!("Getting item at index: {}", index);
        if index >= self.size {
            panic!("Index out of bounds");
        }
        let block_index = index / self.vector;
        let position = index % self.vector;
        println!("Block index: {}, Position: {}", block_index, position);
        &self.array[block_index][position]
    }

    fn reset(&mut self) {
        println!("Resetting array");
        while self.array.size() > 0 {
            self.array.remove(0);
        }
        self.size = 0;
    }
}
