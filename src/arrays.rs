use std::ops::{Deref, DerefMut};
use std::time::Instant;
use rand::Rng; 

trait DynamicArray<T> {
    fn add(&mut self, item: T, index: usize);
    fn remove(&mut self, index: usize) -> T;
    fn size(&self) -> usize;
    fn get(&self, index: usize) -> &T;
    fn reset(&mut self) {
        while self.size() > 0 {
            self.remove(0);
        }
    }
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

    fn get(&self, index: usize) -> &T {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        unsafe { &*self.array.add(index) }
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

    fn get(&self, index: usize) -> &T {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        unsafe { &*self.array.add(index) }
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

    fn get(&self, index: usize) -> &T {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        unsafe { &*self.array.add(index) }
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

    fn get(&self, index: usize) -> &T {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        unsafe { &*self.array.add(index) }
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

impl<T> DynamicArray<T> for MatrixArray<T>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        if self.size == self.array.size() * self.vector {
            self.array.add(VectorArray::with_default(), self.array.size());
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
 
    fn get(&self, index: usize) -> &T {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        let block_index = index / self.vector;
        let position = index % self.vector;
        &self.array[block_index][position]
    }
}

fn measure_performance<T>(array: &mut dyn DynamicArray<T>, size: usize, name: &str, first_row: bool)
where
    T: Clone + Default,
{
    let mut rng = rand::thread_rng();

    // Inserts into first position
    array.reset();
    let start_add_start = Instant::now();
    for _i in 0..size {
        array.add(T::default(), 0);
    }
    let duration_add_start = start_add_start.elapsed();

    // Inserts into random position
    array.reset();
    let mut ixes: Vec<usize> = (0..size).collect();
    for i in 0..size {
        ixes[i] = rng.gen_range(0..=i);
    }
    let start_add_random = Instant::now();
    for &ix in &ixes {
        array.add(T::default(), ix);
    }
    let duration_add_random = start_add_random.elapsed();

    // Inserts into last position
    array.reset();
    let start_add_end = Instant::now();
    for _i in 0..size {
        array.add(T::default(), array.size());
    }
    let duration_add_end = start_add_end.elapsed();


    // Reads from first position
    let start_read_start = Instant::now();
    for _i in 0..size {
        let _ = array.get(0);
    }
    let duration_read_start = start_read_start.elapsed();

    // Reads from random position
    let start_read_random = Instant::now();
    for _i in 0..size {
        let random_index = rng.gen_range(0..array.size());
        let _ = array.get(random_index);
    }
    let duration_read_random = start_read_random.elapsed();

    // Reads from last position
    let start_read_end = Instant::now();
    for _i in 0..size {
        let _ = array.get(array.size() - 1);
    }
    let duration_read_end = start_read_end.elapsed();

    // Removes from first position
    let start_remove_start = Instant::now();
    for _i in 0..size {
        array.remove(0);
    }
    let duration_remove_start = start_remove_start.elapsed();

    // Removes from random position
    for _i in 0..size {
        array.add(T::default(), 0);
    }
    let start_remove_random = Instant::now();
    for _i in 0..size {
        let random_index = rng.gen_range(0..array.size());
        array.remove(random_index);
    }
    let duration_remove_random = start_remove_random.elapsed();

    // Removes from last position
    for _i in 0..size {
        array.add(T::default(), 0);
    }
    let start_remove_end = Instant::now();
    for _i in 0..size {
        array.remove(array.size() - 1);
    }
    let duration_remove_end = start_remove_end.elapsed();

    if first_row {
        println!("| {:15} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} |",
                 "Collection",
                 "Insertion",
                 "Insertion",
                 "Insertion",
                 "Reading",
                 "Reading",
                 "Reading",
                 "Deletion",
                 "Deletion",
                 "Deletion");
        println!("| {:15} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} | {:12} |",
                 "",
                 "start",
                 "random",
                 "end",
                 "start",
                 "random",
                 "end",
                 "start",
                 "random",
                 "end");
        println!("|-----------------|--------------|--------------|--------------|--------------|--------------|--------------|--------------|--------------|--------------|");
    }

    println!("| {:15} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} |",
             name,
             format!("{:?}", duration_add_start),
             format!("{:?}", duration_add_random),
             format!("{:?}", duration_add_end),
             format!("{:?}", duration_read_start),
             format!("{:?}", duration_read_random),
             format!("{:?}", duration_read_end),
             format!("{:?}", duration_remove_start),
             format!("{:?}", duration_remove_random),
             format!("{:?}", duration_remove_end));
}
fn main() {
    const SIZE: usize = 100_000;
    let mut single_array: SingleArray<i32> = SingleArray::new();
    let mut vector_array: VectorArray<i32> = VectorArray::with_default();
    let mut factor_array: FactorArray<i32> = FactorArray::with_default();
    let mut matrix_array: MatrixArray<i32> = MatrixArray::with_default();
    let mut array_list: ArrayList<i32> = ArrayList::with_default();

    measure_performance(&mut matrix_array, SIZE, "MatrixArray", false);
    measure_performance(&mut single_array, SIZE, "SingleArray", true);
    measure_performance(&mut vector_array, SIZE, "VectorArray", false);
    measure_performance(&mut factor_array, SIZE, "FactorArray", false);
    measure_performance(&mut array_list, SIZE, "ArrayList", false);
}