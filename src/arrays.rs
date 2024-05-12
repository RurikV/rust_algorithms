use std::time::Instant;

trait DynamicArray<T> {
    fn add(&mut self, item: T, index: usize);
    fn remove(&mut self, index: usize) -> T;
}

struct SingleArray<T> {
    data: Vec<T>,
}

impl<T> SingleArray<T>
where
    T: Clone + Default,
{
    fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl<T> DynamicArray<T> for SingleArray<T>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        let mut new_data = Vec::with_capacity(self.data.len() + 1);
        new_data.extend(self.data.iter().cloned().take(index));
        new_data.push(item);
        new_data.extend(self.data.iter().cloned().skip(index));
        self.data = new_data;
    }

    fn remove(&mut self, index: usize) -> T {
        let mut new_data = Vec::with_capacity(self.data.len() - 1);
        let removed_item = self.data.remove(index);
        new_data.extend(self.data.iter().cloned());
        self.data = new_data;
        removed_item
    }
}

struct VectorArray<T> {
    data: Vec<T>,
}

impl<T> VectorArray<T>
where
    T: Clone + Default,
{
    fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl<T> DynamicArray<T> for VectorArray<T>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        if self.data.len() == self.data.capacity() {
            self.data.reserve(self.data.len()); // Double the capacity
        }
        self.data.insert(index, item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.data.remove(index)
    }
}


struct FactorArray<T> {
    data: Vec<T>,
    growth_factor: usize,
}

impl<T> FactorArray<T>
where
    T: Clone + Default,
{
    fn new(growth_factor: usize) -> Self {
        Self {
            data: Vec::with_capacity(growth_factor),
            growth_factor,
        }
    }
}

impl<T> DynamicArray<T> for FactorArray<T>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        if self.data.capacity() == self.data.len() {
            self.data.reserve(self.growth_factor * self.data.capacity());
        }
        self.data.insert(index, item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.data.remove(index)
    }
}

struct MatrixArray<T> {
    blocks: Vec<Vec<T>>,
    block_size: usize,
}

impl<T> MatrixArray<T>
where
    T: Clone + Default,
{
    fn new(block_size: usize) -> Self {
        Self {
            blocks: Vec::new(),
            block_size,
        }
    }
}

impl<T> DynamicArray<T> for MatrixArray<T>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        // This is a simplified version and doesn't handle all edge cases
        let block_index = index / self.block_size;
        let position = index % self.block_size;

        if block_index >= self.blocks.len() {
            self.blocks.push(Vec::with_capacity(self.block_size));
        }

        self.blocks[block_index].insert(position, item);
    }

    fn remove(&mut self, index: usize) -> T {
        let block_index = index / self.block_size;
        let position = index % self.block_size;
        self.blocks[block_index].remove(position)
    }
}
struct ArrayList<T> {
    data: Vec<T>,
}

impl<T> DynamicArray<T> for ArrayList<T>
where
    T: Clone + Default,
{
    fn add(&mut self, item: T, index: usize) {
        self.data.insert(index, item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.data.remove(index)
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
    let mut single_array: SingleArray<i32> = SingleArray::new();
    let mut vector_array: VectorArray<i32> = VectorArray::new();
    let mut factor_array: FactorArray<i32> = FactorArray::new( 2 );
    let mut matrix_array: MatrixArray<i32> = MatrixArray::new( 10 );
    let mut array_list: ArrayList<i32> = ArrayList { data: Vec::new() };

    println!("Testing SingleArray:");
    measure_performance(&mut single_array, 10000);

    println!("\nTesting VectorArray:");
    measure_performance(&mut vector_array, 10000);

    println!("\nTesting FactorArray:");
    measure_performance(&mut factor_array, 10000);

    println!("\nTesting MatrixArray:");
    measure_performance(&mut matrix_array, 10000);

    println!("\nTesting ArrayList:");
    measure_performance(&mut array_list, 10000);
}
