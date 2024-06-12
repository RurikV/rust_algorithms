use std::ops::{Deref, DerefMut};
use super::dynamic_array::DynamicArray;

pub struct FactorArray<T> {
    array: *mut T,
    capacity: usize,
    length: usize,
    factor: usize,
}

impl<T> FactorArray<T>
where
    T: Clone + Default,
{
    fn new(factor: usize, init_length: usize) -> Self {
        let layout = std::alloc::Layout::array::<T>(init_length).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
        Self {
            array: ptr,
            capacity: init_length,
            length: 0,
            factor,
        }
    }

    pub fn with_default() -> Self {
        Self::new(50, 10)
    }

    fn size(&self) -> usize {
        self.length
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity + self.capacity * self.factor / 100;
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

impl<T> Drop for FactorArray<T> {
    fn drop(&mut self) {
        if !self.array.is_null() {
            unsafe {
                let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.array as *mut u8, layout);
            }
        }
    }
}

impl<T> Deref for FactorArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.array, self.length) }
    }
}

impl<T> DerefMut for FactorArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.array, self.length) }
    }
}

impl<T> DynamicArray<T> for FactorArray<T>
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
