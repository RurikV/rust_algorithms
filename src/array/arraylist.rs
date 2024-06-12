//use std::ops::{Deref, DerefMut};
use super::dynamic_array::DynamicArray;

pub struct ArrayList<T> {
    array: *mut T,
    capacity: usize,
    length: usize,
}

impl<T> ArrayList<T>
where
    T: Clone + Default,
{
    pub fn with_default() -> Self {
        Self::new(10)
    }

    fn new(capacity: usize) -> Self {
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
        Self {
            array: ptr,
            capacity,
            length: 0,
        }
    }

    fn size(&self) -> usize {
        self.length
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let new_array = unsafe {
            let layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut T;
            std::ptr::copy_nonoverlapping(self.array, ptr, self.length);
            ptr
        };
        unsafe {
            let old_layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            std::alloc::dealloc(self.array as *mut u8, old_layout);
        }
        self.array = new_array;
        self.capacity = new_capacity;
    }
}

impl<T> DynamicArray<T> for ArrayList<T>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        if index > self.size() {
            panic!("Index out of bounds");
        }
        if self.size() == self.capacity {
            self.resize();
        }
        unsafe {
            let src = self.array.add(index);
            let dst = src.add(1);
            std::ptr::copy(src, dst, self.size() - index);
            std::ptr::write(src, item);
        }
        self.length += 1;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        unsafe {
            let removed_item = std::ptr::read(self.array.add(index));
            let src = self.array.add(index + 1);
            let dst = self.array.add(index);
            std::ptr::copy(src, dst, self.size() - index - 1);
            self.length -= 1;
            Some(removed_item)
        }
    }

    fn size(&self) -> usize {
        self.length
    }

    fn get(&self, index: usize) -> &T {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        unsafe { &*self.array.add(index) }
    }
}
