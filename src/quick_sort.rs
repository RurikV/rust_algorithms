use rand::Rng;
use std::time::Instant;

fn quick_sort(arr: &mut [i32]) {
    fn quick_sort_rec(arr: &mut [i32], low: isize, high: isize) {
        if low < high {
            let p = partition(arr, low, high);
            quick_sort_rec(arr, low, p - 1);
            quick_sort_rec(arr, p + 1, high);
        }
    }

    fn partition(arr: &mut [i32], low: isize, high: isize) -> isize {
        let pivot = arr[high as usize];
        let mut i = low - 1;
        for j in low..high {
            if arr[j as usize] <= pivot {
                i += 1;
                arr.swap(i as usize, j as usize);
            }
        }
        arr.swap((i + 1) as usize, high as usize);
        i + 1
    }

    let len = arr.len();
    if len > 0 {
        quick_sort_rec(arr, 0, (len - 1) as isize);
    }
}

fn merge_sort(arr: &mut [i32]) {
    fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize) {
        let mut left_sub = arr[left..mid + 1].to_vec();
        let mut right_sub = arr[mid + 1..right + 1].to_vec();

        left_sub.push(i32::MAX);
        right_sub.push(i32::MAX);

        let (mut i, mut j) = (0, 0);

        for k in left..=right {
            if left_sub[i] <= right_sub[j] {
                arr[k] = left_sub[i];
                i += 1;
            } else {
                arr[k] = right_sub[j];
                j += 1;
            }
        }
    }

    fn merge_sort_rec(arr: &mut [i32], left: usize, right: usize) {
        if left < right {
            let mid = (left + right) / 2;
            merge_sort_rec(arr, left, mid);
            merge_sort_rec(arr, mid + 1, right);
            merge(arr, left, mid, right);
        }
    }

    let len = arr.len();
    if len > 0 {
        merge_sort_rec(arr, 0, len - 1);
    }
}

fn generate_random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..10000)).collect()
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
        ("QuickSort", quick_sort),
        ("MergeSort", merge_sort),
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
