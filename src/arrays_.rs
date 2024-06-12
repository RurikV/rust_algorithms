use std::ops::{Deref, DerefMut};
use std::time::Instant;
use rand::Rng; 

// iarray.rs
pub trait IArray<T> {
    fn reset(&mut self);
    fn size(&self) -> usize;
    fn add(&mut self, item: T);
    fn get(&self, index: usize) -> T;
    fn add_at(&mut self, item: T, index: usize);
    fn remove(&mut self, index: usize) -> T;
}

// single_array.rs
// use crate::iarray::IArray;

pub struct SingleArray<T> {
    array: Vec<T>,
}

impl<T> SingleArray<T> {
    pub fn new() -> Self {
        SingleArray { array: Vec::new() }
    }
}

impl<T> IArray<T> for SingleArray<T>
where
    T: Clone,
{
    fn reset(&mut self) {
        self.array.clear();
    }

    fn size(&self) -> usize {
        self.array.len()
    }

    fn add(&mut self, item: T) {
        self.array.push(item);
    }

    fn get(&self, index: usize) -> T {
        self.array[index].clone()
    }

    fn add_at(&mut self, item: T, index: usize) {
        self.array.insert(index, item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.array.remove(index)
    }
}

// vector_array.rs
// use crate::iarray::IArray;

#[derive(Clone)]
pub struct VectorArray<T> {
    array: Vec<T>,
    vector: usize,
}

impl<T> VectorArray<T> {
    pub fn new(vector: usize) -> Self {
        VectorArray {
            array: Vec::new(),
            vector,
        }
    }
}

impl<T> IArray<T> for VectorArray<T>
where
    T: Clone,
{
    fn reset(&mut self) {
        self.array.clear();
    }

    fn size(&self) -> usize {
        self.array.len()
    }

    fn add(&mut self, item: T) {
        self.add_at(item, self.size());
    }

    fn get(&self, index: usize) -> T {
        self.array[index].clone()
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        &mut self.array[index]
    }

    fn add_at(&mut self, item: T, index: usize) {
        if self.size() == self.array.capacity() {
            let new_capacity = self.array.capacity() + self.vector;
            self.array.reserve(new_capacity - self.array.capacity());
        }
        self.array.insert(index, item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.array.remove(index)
    }
}

// factor_array.rs
// use crate::iarray::IArray;

pub struct FactorArray<T> {
    array: Vec<T>,
    factor: usize,
    init_length: usize,
}

impl<T> FactorArray<T> {
    pub fn new(factor: usize, init_length: usize) -> Self {
        FactorArray {
            array: Vec::with_capacity(init_length),
            factor,
            init_length,
        }
    }
}

impl<T> IArray<T> for FactorArray<T>
where
    T: Clone,
{
    fn reset(&mut self) {
        self.array.clear();
        self.array.reserve(self.init_length);
    }

    fn size(&self) -> usize {
        self.array.len()
    }

    fn add(&mut self, item: T) {
        self.add_at(item, self.size());
    }

    fn get(&self, index: usize) -> T {
        self.array[index].clone()
    }

    fn add_at(&mut self, item: T, index: usize) {
        if self.size() == self.array.capacity() {
            let new_capacity = self.array.capacity() + self.array.capacity() * self.factor / 100;
            self.array.reserve(new_capacity - self.array.capacity());
        }
        self.array.insert(index, item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.array.remove(index)
    }
}

// matrix_array.rs
// use crate::iarray::IArray;
// use crate::vector_array::VectorArray;

pub struct MatrixArray<T> {
    array: VectorArray<VectorArray<T>>,
    size: usize,
    vector: usize,
}

impl<T> MatrixArray<T> {
    pub fn new(vector: usize) -> Self {
        MatrixArray {
            array: VectorArray::new(10),
            size: 0,
            vector,
        }
    }
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        &mut self.array[index]
    }
}

impl<T> IArray<T> for MatrixArray<T>
where
    T: Clone,
{
    fn reset(&mut self) {
        for i in 0..self.array.size() {
            self.array.get_mut(i).reset();
        }
        self.array.reset();
        self.size = 0;
    }

    fn size(&self) -> usize {
        self.size
    }

    fn add(&mut self, item: T) {
        self.add_at(item, self.size);
    }

    fn get(&self, index: usize) -> T {
        self.array.get(index / self.vector).get(index % self.vector)
    }

    fn add_at(&mut self, item: T, index: usize) {
        if self.size == self.array.size() * self.vector {
            self.array.add(VectorArray::new(self.vector));
        }
        let bin_to_insert = index / self.vector;
        for i in (bin_to_insert + 1..self.array.size()).rev() {
            let last_item = self.array.get(i - 1).get(self.array.get(i - 1).size() - 1);
            self.array.get_mut(i).add_at(last_item, 0);
            self.array.get_mut(i - 1).remove(self.array.get(i - 1).size() - 1);
        }
        self.array.get_mut(bin_to_insert).add_at(item, index % self.vector);
        self.size += 1;
    }

    fn remove(&mut self, index: usize) -> T {
        let bin_to_remove = index / self.vector;
        let removed = self.array.get_mut(bin_to_remove).remove(index % self.vector);
        for i in bin_to_remove + 1..self.array.size() {
            let first_item = self.array.get(i).get(0);
            self.array.get_mut(i - 1).add(first_item);
            self.array.get_mut(i).remove(0);
        }
        if self.array.get(self.array.size() - 1).size() == 0 {
            self.array.remove(self.array.size() - 1);
        }
        self.size -= 1;
        removed
    }
}

fn measure_performance<T>(array: &mut dyn IArray<T>, size: usize, name: &str, first_row: bool)
where
    T: Clone + Default,
{
    let mut rng = rand::thread_rng();

    // Inserts into first position
    println!("-----Inserts into first position");
    array.reset();
    let start_add_start = Instant::now();
    for _i in 0..size {
        array.add_at(T::default(), 0);
    }
    let duration_add_start = start_add_start.elapsed();

    // Inserts into random position
    println!("-----Inserts into random position");
    array.reset();
    let mut ixes: Vec<usize> = (0..size).collect();
    for i in 0..size {
        ixes[i] = rng.gen_range(0..=i);
    }
    let start_add_random = Instant::now();
    for &ix in &ixes {
        array.add_at(T::default(), ix);
    }
    let duration_add_random = start_add_random.elapsed();

    // Inserts into last position
    println!("-----Inserts into last position");
    array.reset();
    let start_add_end = Instant::now();
    for _i in 0..size {
        array.add_at(T::default(), array.size());
    }
    let duration_add_end = start_add_end.elapsed();

    // Reads from first position
    println!("-----Reads from first position");
    let start_read_start = Instant::now();
    for _i in 0..size {
        let _ = array.get(0);
    }
    let duration_read_start = start_read_start.elapsed();

    // Reads from random position
    println!("-----Reads from random position");
    let start_read_random = Instant::now();
    for _ in 0..size {
        let random_index = rng.gen_range(0..array.size());
        let _ = array.get(random_index);
    }
    let duration_read_random = start_read_random.elapsed();

    // Reads from last position
    println!("-----Reads from last position");
    let start_read_end = Instant::now();
    for _i in 0..size {
        let _ = array.get(array.size() - 1);
    }
    let duration_read_end = start_read_end.elapsed();

    // Removes from first position
    println!("-----Removes from first position");
    let start_remove_start = Instant::now();
    for _i in 0..size {
        array.remove(0);
    }
    let duration_remove_start = start_remove_start.elapsed();

    // Removes from random position
    println!("-----Removes from random position");
    for _i in 0..size {
        array.add_at(T::default(), 0);
    }
    let start_remove_random = Instant::now();
    for _i in 0..size {
        let random_index = rng.gen_range(0..array.size());
        array.remove(random_index);
    }
    let duration_remove_random = start_remove_random.elapsed();

    // Removes from last position
    println!("-----Removes from last position");
    for _i in 0..size {
        array.add_at(T::default(), 0);
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
    const SIZE: usize = 4;
    let mut single_array: SingleArray<i32> = SingleArray::new();
    let mut vector_array: VectorArray<i32> = VectorArray::new(10);
    let mut factor_array: FactorArray<i32> = FactorArray::new(50, 10);
    // let mut matrix_array: MatrixArray<i32> = MatrixArray::with_default();
    // let mut array_list: ArrayList<i32> = ArrayList::with_default();

    // measure_performance(&mut matrix_array, SIZE, "MatrixArray", true);
    measure_performance(&mut single_array, SIZE, "SingleArray", true);
    measure_performance(&mut vector_array, SIZE, "VectorArray", false);
    measure_performance(&mut factor_array, SIZE, "FactorArray", false);
    // measure_performance(&mut array_list, SIZE, "ArrayList", false);
}

