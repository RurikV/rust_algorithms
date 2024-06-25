use rand::Rng;
use std::time::Instant;

fn counting_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    let range = (max - min + 1) as usize;

    let mut count = vec![0; range];
    let mut output = vec![0; arr.len()];

    for &num in arr.iter() {
        count[(num - min) as usize] += 1;
    }

    for i in 1..range {
        count[i] += count[i - 1];
    }

    for &num in arr.iter().rev() {
        output[count[(num - min) as usize] - 1] = num;
        count[(num - min) as usize] -= 1;
    }

    arr.copy_from_slice(&output);
}

fn radix_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    let max = *arr.iter().max().unwrap();

    let mut exp = 1;
    while max / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_by_digit(arr: &mut [i32], exp: i32) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];

    for &num in arr.iter() {
        let index = (num / exp) % 10;
        count[index as usize] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for &num in arr.iter().rev() {
        let index = (num / exp) % 10;
        output[count[index as usize] - 1] = num;
        count[index as usize] -= 1;
    }

    arr.copy_from_slice(&output);
}

fn bucket_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    let max = *arr.iter().max().unwrap() as usize;
    let min = *arr.iter().min().unwrap() as usize;
    let bucket_count = 10;
    let bucket_range = (max - min) / bucket_count + 1;

    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); bucket_count];

    for &num in arr.iter() {
        let bucket_index = (num as usize - min) / bucket_range;
        buckets[bucket_index].push(num);
    }

    for bucket in buckets.iter_mut() {
        bucket.sort_unstable();
    }

    let mut index = 0;
    for bucket in buckets {
        for &num in bucket.iter() {
            arr[index] = num;
            index += 1;
        }
    }
}

fn generate_random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..999)).collect()
}

fn measure_sorting_time<F>(sort_fn: F, arr: &mut [i32]) -> String
where
    F: Fn(&mut [i32]),
{
    let start = Instant::now();
    sort_fn(arr);
    let duration = start.elapsed();
    if duration.as_secs() > 0 {
        format!("{} s", duration.as_secs())
    } else if duration.as_millis() > 0 {
        format!("{} ms", duration.as_millis())
    } else if duration.as_micros() > 0 {
        format!("{} μs", duration.as_micros())
    } else {
        format!("{} ns", duration.as_nanos())
    }
}

fn main() {
    let sizes = vec![100, 1000, 10_000, 100_000, 1_000_000];
    let sort_functions: Vec<(&str, fn(&mut [i32]))> = vec![
        ("CountingSort", counting_sort),
        ("RadixSort", radix_sort),
        ("BucketSort", bucket_sort),
    ];

    // Print table header
    print!("{:<25}", "Algorithm");
    for size in &sizes {
        print!("{:<17}", format!("{} elements", size));
    }
    println!();

    // Print sorting times
    for (name, sort_fn) in &sort_functions {
        print!("{:<25}", name);
        for &size in &sizes {
            let mut arr = generate_random_array(size);
            let time = measure_sorting_time(*sort_fn, &mut arr);
            print!("{:<17}", time);
        }
        println!();
    }
}
