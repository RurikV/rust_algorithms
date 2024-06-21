use full_palette::GREY;
use rand::Rng;
use std::time::Instant;
use plotters::prelude::*;
use std::error::Error;

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

fn measure_sorting_time<F>(sort_fn: F, arr: &mut [i32]) -> f64
where
    F: Fn(&mut [i32]),
{
    let start = Instant::now();
    sort_fn(arr);
    let duration = start.elapsed();
    duration.as_secs_f64()
}

fn visualize_sorting_algorithms() -> Result<(), Box<dyn Error>> {
    let sizes = vec![100, 1000, 10_000, 100_000];
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

    let root = BitMapBackend::new("sorting_algorithms.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Sorting Algorithms Performance", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            (sizes[0] as f64).log10()..(sizes[sizes.len() - 1] as f64).log10(),
            0f64..30f64,
        )?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .x_label_formatter(&|x| format!("{:.0}", 10f64.powf(*x)))
        .y_label_formatter(&|y| format!("{:.2}", y))
        .draw()?;

    let colors = vec![
        &RED, &BLUE, &GREEN, &CYAN, &MAGENTA, &YELLOW, &BLACK, &GREY,
    ];

    for ((name, sort_fn), &color) in sort_functions.iter().zip(colors.iter()) {
        let mut times = vec![];
        for &size in &sizes {
            if size == 1_000_000 && (name.contains("Bubble") || name.contains("Insertion")) {
                times.push(30.0); // Arbitrary large value for visualization
            } else {
                let mut arr = generate_random_array(size);
                let time = measure_sorting_time(sort_fn, &mut arr);
                times.push(time);
            }
        }
        chart.draw_series(LineSeries::new(
            sizes.iter().map(|&size| (size as f64).log10()).zip(times.iter().cloned()),
            &color,
        ))?
        .label(name.to_string())
        .legend(move |(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 20, y)],
                ShapeStyle {
                    color: color.to_rgba(),
                    filled: true,
                    stroke_width: 2,
                },
            )
        });
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn main() {
    if let Err(e) = visualize_sorting_algorithms() {
        eprintln!("Error generating visualization: {}", e);
    }
}
