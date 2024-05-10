use std::time::Instant;

trait DynamicArray<T> {
    fn add(&mut self, item: T, index: usize);
    fn remove(&mut self, index: usize) -> T;
}

struct SingleArray<T> {
    data: Vec<T>,
}

impl<T> DynamicArray<T> for SingleArray<T>
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

struct VectorArray<T> {
    data: Vec<T>,
}

impl<T> DynamicArray<T> for VectorArray<T>
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

struct FactorArray<T> {
    data: Vec<T>,
}

impl<T> DynamicArray<T> for FactorArray<T>
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

struct MatrixArray<T> {
    data: Vec<T>,
}

impl<T> DynamicArray<T> for MatrixArray<T>
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
    let mut single_array: SingleArray<i32> = SingleArray { data: Vec::new() };
    let mut vector_array: VectorArray<i32> = VectorArray { data: Vec::new() };
    let mut factor_array: FactorArray<i32> = FactorArray { data: Vec::new() };
    let mut matrix_array: MatrixArray<i32> = MatrixArray { data: Vec::new() };

    println!("Testing SingleArray:");
    measure_performance(&mut single_array, 10000);

    println!("\nTesting VectorArray:");
    measure_performance(&mut vector_array, 10000);

    println!("\nTesting FactorArray:");
    measure_performance(&mut factor_array, 10000);

    println!("\nTesting MatrixArray:");
    measure_performance(&mut matrix_array, 10000);
}
