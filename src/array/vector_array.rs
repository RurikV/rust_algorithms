use std::ops::{Deref, DerefMut};
use super::dynamic_array::DynamicArray;

#[derive(Clone)]
pub struct VectorArray<T> {
    array: *mut T,
    capacity: usize,
    length: usize,
    vector: usize,
}

impl<T> Default for VectorArray<T> {
    fn default() -> Self {
        Self {
            array: std::ptr::null_mut(),
            capacity: 0,
            length: 0,
            vector: 10, // Default vector size
        }
    }
}

impl<T> VectorArray<T>
where
    T: Clone + Default,
{
    pub fn with_default() -> Self {
        Self::new(10)
    }

    pub fn new(vector: usize) -> Self {
        Self {
            array: std::ptr::null_mut(),
            capacity: 0,
            length: 0,
            vector,
        }
    }

    pub fn resize(&mut self, new_capacity: usize) {
        let new_array = unsafe {
            let layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut T;
            if !self.array.is_null() {
                std::ptr::copy_nonoverlapping(self.array, ptr, self.length);
                let old_layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.array as *mut u8, old_layout);
            }
            ptr
        };
        self.array = new_array;
        self.capacity = new_capacity;
    }
}

impl<T> Drop for VectorArray<T> {
    fn drop(&mut self) {
        if !self.array.is_null() {
            unsafe {
                let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.array as *mut u8, layout);
            }
        }
    }
}

impl<T> Deref for VectorArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.array, self.length) }
    }
}

impl<T> DerefMut for VectorArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.array, self.length) }
    }
}

impl<T> DynamicArray<T> for VectorArray<T>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        if index > self.length {
            // let new_capacity = self.capacity + self.vector;
            // self.resize(new_capacity);
            panic!("Index out of bounds");
        }
        if self.length == self.capacity {
            let new_capacity = self.capacity + self.vector;
            self.resize(new_capacity);
        }
        unsafe {
            let src = self.array.add(index);
            let dst = src.add(1);
            std::ptr::copy(src, dst, self.length - index);
            std::ptr::write(src, item);
        }
        self.length += 1;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }
        unsafe {
            let removed_item = std::ptr::read(self.array.add(index));
            let src = self.array.add(index + 1);
            let dst = self.array.add(index);
            std::ptr::copy(src, dst, self.length - index - 1);
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

    fn reset(&mut self) {
        self.length = 0;
    }
}
