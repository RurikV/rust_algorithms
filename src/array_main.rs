use std::time::Instant;
use rand::Rng;

mod array;
use array::dynamic_array::DynamicArray;
use array::single_array::SingleArray;
use array::vector_array::VectorArray;
use array::factor_array::FactorArray;
use array::matrix_array::MatrixArray;
use array::arraylist::ArrayList;

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
    for _ in 0..size {
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
    let mut current_size = array.size();
    for _i in 0..size {
        if current_size > 0 {
            let random_index = rng.gen_range(0..current_size);
            array.remove(random_index);
            current_size -= 1;
        }
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

    measure_performance(&mut single_array, SIZE, "SingleArray", true);
    measure_performance(&mut vector_array, SIZE, "VectorArray", false);
    measure_performance(&mut factor_array, SIZE, "FactorArray", false);
    measure_performance(&mut matrix_array, SIZE, "MatrixArray", false);
    measure_performance(&mut array_list, SIZE, "ArrayList", false);
}
