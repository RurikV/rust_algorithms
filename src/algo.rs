use std::time::Instant;

// 1. Power Calculation
fn power_iterative(base: f64, exponent: i32) -> f64 {
    if exponent == 0 {
        return 1.0;
    }
    let mut result = 1.0;
    for _ in 0..exponent {
        result *= base;
    }
    result
}

// 2. Fibonacci Numbers
fn fibonacci_recursive(n: i32) -> i64 {
    if n <= 1 {
        return n as i64;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

fn fibonacci_iterative(n: i32) -> i64 {
    if n <= 1 {
        return n as i64;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

// 3. Prime Number Search
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let limit = (n as f64).sqrt() as i32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn count_primes_up_to_n(n: i32) -> i32 {
    let mut count = 0;
    for i in 2..=n {
        if is_prime(i) {
            count += 1;
        }
    }
    count
}

// Benchmarking 
fn main() {
    // Power
    let base = 2.0;
    let exponent = 30;
    let start = Instant::now();
    let result_power = power_iterative(base, exponent);
    let duration_power = start.elapsed();
    println!("power({base}, {exponent}) = {result_power}");
    println!("Time (power): {:?}", duration_power);

    // Fibonacci
    let n_fib = 35;
    let start = Instant::now();
    let result_fib_rec = fibonacci_recursive(n_fib);
    let duration_fib_rec = start.elapsed();
    println!("\nfibonacci_recursive({n_fib}) = {result_fib_rec}");
    println!("Time (fib_recursive): {:?}", duration_fib_rec);

    let start = Instant::now();
    let result_fib_iter = fibonacci_iterative(n_fib);
    let duration_fib_iter = start.elapsed();
    println!("fibonacci_iterative({n_fib}) = {result_fib_iter}");
    println!("Time (fib_iterative): {:?}", duration_fib_iter);

    // Prime numbers
    let limit = 10_000;
    let start = Instant::now();
    let prime_count = count_primes_up_to_n(limit);
    let duration_prime = start.elapsed();
    println!("\nPrimes up to {limit} = {prime_count}");
    println!("Time (primes): {:?}", duration_prime);
}
