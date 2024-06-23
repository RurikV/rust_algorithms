use rand::Rng;
use std::time::Instant;

fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}

fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();

    fn heapify(arr: &mut [i32], n: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && arr[left] > arr[largest] {
            largest = left;
        }

        if right < n && arr[right] > arr[largest] {
            largest = right;
        }

        if largest != i {
            arr.swap(i, largest);
            heapify(arr, n, largest);
        }
    }

    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
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
        ("SelectionSort", selection_sort),
        ("HeapSort", heap_sort),
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
            if size == 1_000_000 && (name.contains("Insertion") || name.contains("Selection")) {
                print!("{:<17}", ">");
            } else {
                let mut arr = generate_random_array(size);
                let time = measure_sorting_time(*sort_fn, &mut arr);
                print!("{:<17}", time);
            }
        }
        println!();
    }
}
