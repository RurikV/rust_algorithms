use std::ops::{Deref, DerefMut};
use std::time::Instant;

trait DynamicArray<T> {
    fn add(&mut self, item: T, index: usize);
    fn remove(&mut self, index: usize) -> T;
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
            self.capacity * 2
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
}
struct VectorArray<T, const N: usize> {
    data: [T; N],
    length: usize,
}

impl<T, const N: usize> VectorArray<T, N>
where
    T: Clone + Default + Copy,
{
    fn new() -> Self {
        Self {
            data: [T::default(); N],
            length: 0,
        }
    }
}

impl<T, const N: usize> DynamicArray<T> for VectorArray<T, N>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        if index > self.length {
            return;
        }
        if self.length == N {
            panic!("Array is full");
        }
        for i in (index + 1..=self.length).rev() {
            self.data[i] = self.data[i - 1].clone();
        }
        self.data[index] = item;
        self.length += 1;
    }

    fn remove(&mut self, index: usize) -> T {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        let removed_item = self.data[index].clone();
        for i in index..self.length - 1 {
            self.data[i] = self.data[i + 1].clone();
        }
        self.length -= 1;
        removed_item
    }
}

struct FactorArray<T, const N: usize> {
    data: [T; N],
    length: usize,
}

impl<T, const N: usize> FactorArray<T, N>
where
    T: Clone + Default + Copy,
{
    fn new() -> Self {
        Self {
            data: [T::default(); N],
            length: 0,
        }
    }
}

impl<T, const N: usize> DynamicArray<T> for FactorArray<T, N>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        if index > self.length {
            return;
        }
        if self.length == N {
            panic!("Array is full");
        }
        for i in (index + 1..=self.length).rev() {
            self.data[i] = self.data[i - 1].clone();
        }
        self.data[index] = item;
        self.length += 1;
    }

    fn remove(&mut self, index: usize) -> T {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        let removed_item = self.data[index].clone();
        for i in index..self.length - 1 {
            self.data[i] = self.data[i + 1].clone();
        }
        self.length -= 1;
        removed_item
    }
}

struct MatrixArray<T, const N: usize, const M: usize> {
    blocks: [[T; N]; M],
    lengths: [usize; M],
}

impl<T, const N: usize, const M: usize> MatrixArray<T, N, M>
where
    T: Clone + Default + Copy, 
{
    fn new() -> Self {
        Self {
            blocks: [[T::default(); N]; M],
            lengths: [0; M],
        }
    }
}

impl<T, const N: usize, const M: usize> DynamicArray<T> for MatrixArray<T, N, M>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        let block_index = index / N;
        let position = index % N;

        if block_index >= M {
            return;
        }

        if self.lengths[block_index] == N {
            panic!("Block is full");
        }

        for i in (position + 1..=self.lengths[block_index]).rev() {
            self.blocks[block_index][i] = self.blocks[block_index][i - 1].clone();
        }
        self.blocks[block_index][position] = item;
        self.lengths[block_index] += 1;
    }

    fn remove(&mut self, index: usize) -> T {
        let block_index = index / N;
        let position = index % N;

        if block_index >= M || position >= self.lengths[block_index] {
            panic!("Index out of bounds");
        }

        let removed_item = self.blocks[block_index][position].clone();
        for i in position..self.lengths[block_index] - 1 {
            self.blocks[block_index][i] = self.blocks[block_index][i + 1].clone();
        }
        self.lengths[block_index] -= 1;
        removed_item
    }
}

fn measure_performance<T>(array: &mut dyn DynamicArray<T>, size: usize)
where
    T: Clone + Default,
{
    let start_add = Instant::now();
    for i in 0..size {
        array.add(T::default(), i);
    }
    let duration_add = start_add.elapsed();

    let start_remove = Instant::now();
    for i in (0..size).rev() {
        array.remove(i);
    }
    let duration_remove = start_remove.elapsed();

    println!("Addition Time: {:?}", duration_add);
    println!("Removal Time: {:?}", duration_remove);
}

fn main() {
    const SIZE: usize = 10_000;
    const BLOCK_SIZE: usize = 1_000;
    let mut single_array: SingleArray<i32> = SingleArray::new();
    let mut vector_array: VectorArray<i32, SIZE> = VectorArray::new();
    let mut factor_array: FactorArray<i32, SIZE> = FactorArray::new();
    let mut matrix_array: MatrixArray<i32, 10, BLOCK_SIZE> = MatrixArray::new();

    println!("Testing SingleArray:");
    measure_performance(&mut single_array, SIZE);

    println!("\nTesting VectorArray:");
    measure_performance(&mut vector_array, SIZE);

    println!("\nTesting FactorArray:");
    measure_performance(&mut factor_array, SIZE);

    println!("\nTesting MatrixArray:");
    measure_performance(&mut matrix_array, SIZE);
}