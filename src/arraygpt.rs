use std::time::Instant;
use std::collections::VecDeque;
use std::collections::LinkedList;
use rand::seq::SliceRandom;

trait IArray<T> {
    fn size(&self) -> usize;
    fn add(&mut self, item: T);
    fn get(&self, index: usize) -> T;
    fn add_at(&mut self, item: T, index: usize);
    fn remove(&mut self, index: usize) -> T;
    fn reset(&mut self);
}

struct SingleArray<T> {
    array: Vec<T>,
}

impl<T> SingleArray<T> {
    fn new() -> Self {
        Self { array: Vec::new() }
    }
}

impl<T: Clone> IArray<T> for SingleArray<T> {
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

    fn reset(&mut self) {
        self.array.clear();
    }
}

struct VectorArray<T> {
    array: Vec<T>,
    vector: usize,
}

impl<T> VectorArray<T> {
    fn new(vector: usize) -> Self {
        Self {
            array: Vec::new(),
            vector,
        }
    }

    fn resize(&mut self) {
        let new_size = self.array.len() + self.vector;
        self.array.reserve(new_size);
    }
}

impl<T: Clone> IArray<T> for VectorArray<T> {
    fn size(&self) -> usize {
        self.array.len()
    }

    fn add(&mut self, item: T) {
        if self.size() == self.array.capacity() {
            self.resize();
        }
        self.array.push(item);
    }

    fn get(&self, index: usize) -> T {
        self.array[index].clone()
    }

    fn add_at(&mut self, item: T, index: usize) {
        if self.size() == self.array.capacity() {
            self.resize();
        }
        self.array.insert(index, item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.array.remove(index)
    }

    fn reset(&mut self) {
        self.array.clear();
    }
}

struct FactorArray<T> {
    array: Vec<T>,
    allocated: usize,
    factor: usize,
    init_length: usize,
}

impl<T> FactorArray<T> {
    fn new(factor: usize, init_length: usize) -> Self {
        Self {
            array: Vec::with_capacity(init_length),
            allocated: init_length,
            factor,
            init_length,
        }
    }

    fn resize(&mut self) {
        self.allocated += self.allocated * self.factor / 100;
        self.array.reserve(self.allocated);
    }
}

impl<T: Clone> IArray<T> for FactorArray<T> {
    fn size(&self) -> usize {
        self.array.len()
    }

    fn add(&mut self, item: T) {
        if self.size() == self.allocated {
            self.resize();
        }
        self.array.push(item);
    }

    fn get(&self, index: usize) -> T {
        self.array[index].clone()
    }

    fn add_at(&mut self, item: T, index: usize) {
        if self.size() == self.allocated {
            self.resize();
        }
        self.array.insert(index, item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.array.remove(index)
    }

    fn reset(&mut self) {
        self.array.clear();
        self.allocated = self.init_length;
    }
}


fn check_valid(array: &mut dyn IArray<i32>) -> bool {
    let s0 = array.size();
    array.add(1);
    let s1 = array.size();
    array.add(3);
    let s2 = array.size();
    array.add_at(2, 1);
    let s3 = array.size();
    if s0 == 0 && s1 == 1 && s2 == 2 && s3 == 3 {
        println!("Size - OK");
    } else {
        println!("Size - Fail");
        return false;
    }
    if array.get(0) == 1 && array.get(1) == 2 && array.get(2) == 3 {
        println!("Add/Get - OK");
    } else {
        println!("Add/Get - Fail");
        return false;
    }
    if array.remove(1) == 2 && array.get(0) == 1 && array.get(1) == 3 && array.size() == 2
        && array.remove(1) == 3 && array.get(0) == 1 && array.size() == 1
        && array.remove(0) == 1 && array.size() == 0
    {
        println!("Remove - OK");
    } else {
        println!("Remove - Fail");
        return false;
    }
    array.reset();
    for i in 0..1000 {
        array.add(i);
    }
    for j in 0..1000 {
        if array.get(j) != j {
            println!("Back insert - Fail");
            return false;
        }
    }
    println!("Back insert - OK");
    array.reset();
    for i in 0..1000 {
        array.add_at(i, 0);
    }
    for j in 0..1000 {
        if array.get(j) != 999 - j {
            println!("Front insert - Fail");
            return false;
        }
    }
    println!("Front insert - OK");
    true
}

fn check_performance(array: &mut dyn IArray<i32>, n: usize) {
    let mut rng = rand::thread_rng();
    let mut start = Instant::now();
    array.reset();
    for i in 0..n {
        array.add_at(i as i32, 0);
    }
    println!(
        "{} inserts into first position {}ms.",
        n,
        start.elapsed().as_millis()
    );

    let mut ixes: Vec<usize> = (0..n).collect();
    ixes.shuffle(&mut rng);

    start = Instant::now();
    for &ix in &ixes {
        array.add_at(ix as i32, ix);
    }
    println!(
        "{} inserts into random position {}ms.",
        n,
        start.elapsed().as_millis()
    );

    start = Instant::now();
    for i in 0..n {
        array.add(i as i32);
    }
    println!(
        "{} inserts into last position {}ms.",
        n,
        start.elapsed().as_millis()
    );

    start = Instant::now();
    for _ in 0..n {
        array.get(0);
    }
    println!(
        "{} reads from first position {}ms.",
        n,
        start.elapsed().as_millis()
    );

    ixes = (0..n).collect();
    ixes.shuffle(&mut rng);

    start = Instant::now();
    for &ix in &ixes {
        array.get(ix);
    }
    println!(
        "{} reads from random position {}ms.",
        n,
        start.elapsed().as_millis()
    );

    start = Instant::now();
    for _ in 0..n {
        array.get(n - 1);
    }
    println!(
        "{} reads from last position {}ms.",
        n,
        start.elapsed().as_millis()
    );

    array.reset();
    for i in 0..n {
        array.add_at(i as i32, i);
    }
    start = Instant::now();
    for _ in 0..n {
        array.remove(0);
    }
    println!(
        "{} removes from first position {}ms.",
        n,
        start.elapsed().as_millis()
    );

    array.reset();
    ixes.clear();
    ixes.extend((0..n).map(|i| rand::random::<usize>() % (n - i)));
    for i in 0..n {
        array.add_at(i as i32, i);
    }
    start = Instant::now();
    for &ix in &ixes {
        array.remove(ix);
    }
    println!(
        "{} removes from random position {}ms.",
        n,
        start.elapsed().as_millis()
    );

    array.reset();
    for i in 0..n {
        array.add_at(i as i32, i);
    }
    start = Instant::now();
    for _ in 0..n {
        array.remove(array.size() - 1);
    }
    println!(
        "{} removes from last position {}ms.",
        n,
        start.elapsed().as_millis()
    );
    array.reset();
}

fn main() {
    let tests = 100_000;

    let mut single_array = SingleArray::new();
    println!("SingleArray");
    if check_valid(&mut single_array) {
        check_performance(&mut single_array, tests);
    }

    let mut vector_array = VectorArray::new(10);
    println!("VectorArray");
    if check_valid(&mut vector_array) {
        check_performance(&mut vector_array, tests);
    }

    let mut factor_array = FactorArray::new(50, 10);
    println!("FactorArray");
    if check_valid(&mut factor_array) {
        check_performance(&mut factor_array, tests);
    }
}
