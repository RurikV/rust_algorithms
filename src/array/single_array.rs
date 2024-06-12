
use std::ops::{Deref, DerefMut};
use super::dynamic_array::DynamicArray;

pub struct SingleArray<T> {
    array: *mut T,
    capacity: usize,
    length: usize,
}

impl<T> SingleArray<T>
where
    T: Clone + Default,
{
    pub fn new() -> Self {
        Self {
            array: std::ptr::null_mut(),
            capacity: 0,
            length: 0,
        }
    }

    fn resize(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity + 1
        };
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

impl<T> Drop for SingleArray<T> {
    fn drop(&mut self) {
        if !self.array.is_null() {
            unsafe {
                let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.array as *mut u8, layout);
            }
        }
    }
}

impl<T> Deref for SingleArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.array, self.length) }
    }
}

impl<T> DerefMut for SingleArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.array, self.length) }
    }
}

impl<T> DynamicArray<T> for SingleArray<T>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        if index > self.length {
            panic!("Index out of bounds");
        }
        if self.length == self.capacity {
            self.resize();
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
}

