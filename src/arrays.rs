use std::ops::{Deref, DerefMut};
use std::time::Instant;

trait DynamicArray<T> {
    fn add(&mut self, item: T, index: usize);
    fn remove(&mut self, index: usize) -> T;
    fn size(&self) -> usize;
}

struct SingleArray<T> {
    array: *mut T,
    capacity: usize,
    length: usize,
}

impl<T> SingleArray<T>
where
    T: Clone + Default,
{
    fn new() -> Self {
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

    fn remove(&mut self, index: usize) -> T {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        unsafe {
            let removed_item = std::ptr::read(self.array.add(index));
            let src = self.array.add(index + 1);
            let dst = self.array.add(index);
            std::ptr::copy(src, dst, self.length - index - 1);
            self.length -= 1;
            removed_item
        }
    }

    
    fn size(&self) -> usize {
        self.length
    }
}

#[derive(Clone)]
struct VectorArray<T> {
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
    fn with_default() -> Self {
        Self::new(10)
    }

    fn new(vector: usize) -> Self {
        Self {
            array: std::ptr::null_mut(),
            capacity: 0,
            length: 0,
            vector,
        }
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity + self.vector;
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

    fn remove(&mut self, index: usize) -> T {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        unsafe {
            let removed_item = std::ptr::read(self.array.add(index));
            let src = self.array.add(index + 1);
            let dst = self.array.add(index);
            std::ptr::copy(src, dst, self.length - index - 1);
            self.length -= 1;
            removed_item
        }
    }
    
    fn size(&self) -> usize {
        self.length
    }
}

struct FactorArray<T> {
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

    fn with_default() -> Self {
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

    fn remove(&mut self, index: usize) -> T {
        if index >= self.size() {
            panic!("Index out of bounds");
        }
        unsafe {
            let removed_item = std::ptr::read(self.array.add(index));
            let src = self.array.add(index + 1);
            let dst = self.array.add(index);
            std::ptr::copy(src, dst, self.size() - index - 1);
            self.length -= 1;
            removed_item
        }
    }
    
    fn size(&self) -> usize {
        self.length
    }
}

struct MatrixArray<T> {
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

    fn with_default() -> Self {
        Self::new(10)
    }

}

struct ArrayList<T> {
    array: *mut T,
    capacity: usize,
    length: usize,
}

impl<T> ArrayList<T>
where
    T: Clone + Default,
{
    fn with_default() -> Self {
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

impl<T> Drop for ArrayList<T> {
    fn drop(&mut self) {
        if !self.array.is_null() {
            unsafe {
                let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.array as *mut u8, layout);
            }
        }
    }
}

impl<T> Deref for ArrayList<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.array, self.length) }
    }
}

impl<T> DerefMut for ArrayList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.array, self.length) }
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

    fn remove(&mut self, index: usize) -> T {
        if index >= self.size() {
            panic!("Index out of bounds");
        }
        unsafe {
            let removed_item = std::ptr::read(self.array.add(index));
            let src = self.array.add(index + 1);
            let dst = self.array.add(index);
            std::ptr::copy(src, dst, self.size() - index - 1);
            self.length -= 1;
            removed_item
        }
    }
    
    fn size(&self) -> usize {
        self.length
    }
}

impl<T> DynamicArray<T> for MatrixArray<T>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        if self.size == self.array.size() * self.vector {
            self.array.add(VectorArray::new(self.vector), self.array.size());
        }
        let block_index = index / self.vector;
        let position = index % self.vector;
        self.array[block_index].add(item, position);
        self.size += 1;
    }

    fn remove(&mut self, index: usize) -> T {
        let block_index = index / self.vector;
        let position = index % self.vector;
        let removed_item = self.array[block_index].remove(position);
        self.size -= 1;
        removed_item
    }
    
    fn size(&self) -> usize {
        self.size
    }
}

fn measure_performance<T>(array: &mut dyn DynamicArray<T>, size: usize)
where
    T: Clone + Default,
{
    let start_add_start = Instant::now();
    for i in 0..size {
        array.add(T::default(), i);
    }
    let duration_add_start = start_add_start.elapsed();

    let start_add_end = Instant::now();
    array.add(T::default(), size);
    let duration_add_end = start_add_end.elapsed();

    let start_remove_start = Instant::now();
    if array.size() > 0 {
        array.remove(0);
    }
    let duration_remove_start = start_remove_start.elapsed();

    let start_remove_end = Instant::now();
    if array.size() > 0 {
        array.remove(array.size() - 1);
    }
    let duration_remove_end = start_remove_end.elapsed();

    println!("| {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} |",
             "Insertion",
             "Insertion",
             "Insertion",
             "Reading",
             "Reading",
             "Reading",
             "Deletion",
             "Deletion",
             "Deletion");
    println!("| {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} |",
             "start",
             "random",
             "end",
             "start",
             "random",
             "end",
             "start",
             "random",
             "end");
    println!("|--------------|--------------|--------------|--------------|--------------|--------------|--------------|--------------|--------------|");
    println!("| {:>12} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} |",
             format!("{:?}", duration_add_start),
             format!("{:?}", duration_add_end),
             "N/A",
             "N/A",
             "N/A",
             "N/A",
             format!("{:?}", duration_remove_start),
             "N/A",
             format!("{:?}", duration_remove_end));
    println!("|--------------|--------------|--------------|--------------|--------------|--------------|--------------|--------------|--------------|");
}

fn main() {
    const SIZE: usize = 100_000;
    let mut single_array: SingleArray<i32> = SingleArray::new();
    let mut vector_array: VectorArray<i32> = VectorArray::with_default();
    let mut factor_array: FactorArray<i32> = FactorArray::with_default();
    let mut matrix_array: MatrixArray<i32> = MatrixArray::with_default();
    let mut array_list: ArrayList<i32> = ArrayList::with_default();

    println!("Testing SingleArray:");
    measure_performance(&mut single_array, SIZE);

    println!("\nTesting VectorArray:");
    measure_performance(&mut vector_array, SIZE);

    println!("\nTesting FactorArray:");
    measure_performance(&mut factor_array, SIZE);

    println!("\nTesting MatrixArray:");
    measure_performance(&mut matrix_array, SIZE);

    println!("\nTesting ArrayList:");
    measure_performance(&mut array_list, SIZE);
}