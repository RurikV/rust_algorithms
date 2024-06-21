use rand::Rng;
use std::time::Instant;

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn optimized_bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn optimized_insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn binary_insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut left = 0;
        let mut right = i;
        while left < right {
            let mid = (left + right) / 2;
            if arr[mid] > key {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        for j in (left..i).rev() {
            arr[j + 1] = arr[j];
        }
        arr[left] = key;
    }
}

fn shell_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mut gap = n / 2;
    while gap > 0 {
        for i in gap..n {
            let temp = arr[i];
            let mut j = i;
            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }
            arr[j] = temp;
        }
        gap /= 2;
    }
}

fn ciura_shell_sort(arr: &mut [i32]) {
    let gaps = vec![701, 301, 132, 57, 23, 10, 4, 1];
    let n = arr.len();
    for gap in gaps {
        for i in gap..n {
            let temp = arr[i];
            let mut j = i;
            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }
            arr[j] = temp;
        }
    }
}

fn sedgewick_shell_sort(arr: &mut [i32]) {
    let gaps = vec![929, 505, 209, 109, 41, 19, 5, 1];
    let n = arr.len();
    for gap in gaps {
        for i in gap..n {
            let temp = arr[i];
            let mut j = i;
            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }
            arr[j] = temp;
        }
    }
}

fn generate_random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..10000)).collect()
}

fn measure_sorting_time<F>(sort_fn: F, arr: &mut [i32]) -> u128
where
    F: Fn(&mut [i32]),
{
    let start = Instant::now();
    sort_fn(arr);
    let duration = start.elapsed();
    duration.as_millis()
}


fn main() {
    let sizes = vec![100, 1000, 10_000, 100_000, 1_000_000];
    let sort_functions: Vec<(&str, fn(&mut [i32]))> = vec![
        ("BubbleSort", bubble_sort),
        ("Optimized BubbleSort", optimized_bubble_sort),
        ("InsertionSort", insertion_sort),
        ("Optimized InsertionSort", optimized_insertion_sort),
        ("Binary InsertionSort", binary_insertion_sort),
        ("ShellSort", shell_sort),
        ("Ciura ShellSort", ciura_shell_sort),
        ("Sedgewick ShellSort", sedgewick_shell_sort),
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
            if size == 1_000_000 && (name.contains("Bubble") || name.contains("Insertion")) {
                print!("{:<17}", ">");
            } else {
                let mut arr = generate_random_array(size);
                let time = measure_sorting_time(*sort_fn, &mut arr);
                print!("{:<17}", format!("{} ms", time));
            }
        }
        println!();
    }
}

