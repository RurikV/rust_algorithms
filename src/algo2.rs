fn power_by_multiplication(base: f64, exponent: i32) -> f64 {
    if exponent == 0 {
        return 1.0;
    }

    let mut result = 1.0;
    let mut power = base;

    let mut is_even = exponent % 2 == 0;
    let mut exponent = exponent;
    while exponent > 0 {
        if !is_even {
            result *= power;
        }

        power *= power;
        exponent /= 2;
        is_even = !is_even;
    }

    result
}

fn power_by_binary_exponentiation(base: f64, exponent: i32) -> f64 {
    if exponent == 0 {
        return 1.0;
    }

    let mut result = 1.0;
    let mut power = base;
    let mut exponent = exponent;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result *= power;
        }

        power *= power;
        exponent /= 2;
    }

    result
}

fn fibonacci_by_golden_ratio(n: i32) -> f64 {
    let phi = (1.0 + (5.0 as f64).sqrt()) / 2.0;
    let result = (phi.powf(n as f64) - (-phi).powf(n as f64)) / (5.0 as f64).sqrt();
    result.round()
}

// struct Matrix {
//     data: [[f64; 2]; 2],
// }

// use std::ops::MulAssign;

// impl Matrix {
//     fn new() -> Self {
//         Self { data: [[1.0, 1.0], [1.0, 0.0]] }
//     }

//     fn power(&mut self, exponent: i32) {
//         if exponent == 0 {
//             return;
//         }

//         let mut temp = Matrix::new();
//         let mut power = self.clone();

//         let mut is_even = exponent % 2 == 0;
//         let mut exponent = exponent;

//         while exponent > 0 {
//             if !is_even {
//                 *self *= power; // Multiply by a reference
//             }

//             temp *= power; // Multiply by a reference
//             power = power.clone(); // Create a new clone as needed
//             exponent /= 2;
//             is_even = !is_even;
//         }
//     }

//     fn get_fibonacci_number(&self) -> i32 {
//         (self.data[0][1] as f64).round() as i32
//     }
// }

// impl Clone for Matrix {
//     fn clone(&self) -> Self {
//         Self { data: self.data }
//     }
// }

// impl MulAssign for Matrix {
//     fn mul_assign(&mut self, rhs: Self) {
//         let mut result = Matrix::new();
//         for i in 0..2 {
//             for j in 0..2 {
//                 for k in 0..2 {
//                     result.data[i][j] += self.data[i][k] * rhs.data[k][j];
//                 }
//             }
//         }
//         *self = result;
//     }
// }

// fn fibonacci_by_matrix_multiplication(n: i32) -> i32 {
//     let mut matrix = Matrix::new();
//     matrix.power(n);
//     matrix.get_fibonacci_number()
// }

fn count_primes_optimized(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    }

    let mut is_prime: Vec<bool> = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let limit = (n as f64).sqrt() as usize;
    for i in 2..=limit {
        if is_prime[i] {
            let mut multiple = i * i;
            while multiple <= n as usize {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    // Optimization: Check only for odd numbers
    for i in 2..=n as usize {
        if is_prime[i] && i % 2 == 0 {
            is_prime[i] = false;
        }
    }

    is_prime.iter().filter(|&p| *p).count() as i32
}

use num::integer::Roots;

fn count_primes_eratosthenes(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    }

    let mut is_prime: Vec<bool> = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=(n as f64).sqrt() as i32 {
        if is_prime[i as usize] {
            // Cast i to usize here
            let mut multiple = i * i;
            while multiple <= n {
                is_prime[multiple as usize] = false; // Cast multiple to usize here
                multiple += i;
            }
        }
    }

    is_prime.iter().filter(|&p| *p).count() as i32
}

fn count_primes_eratosthenes_optimized_memory(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    }

    let size = (n as usize + 1) / 8 + 1; // Ensure there is enough space for all bits
    let mut is_prime: Vec<u8> = vec![0xFF; size]; // Initialize with all bits set to 1

    is_prime[0] &= !0b11; // Mark 0 and 1 as not prime

    let limit = (n as f64).sqrt() as i32;
    for i in 2..=limit {
        if is_prime[(i >> 3) as usize] & (1 << (i & 7)) != 0 { // Use '&' for bit operation
            let mut multiple = i * i;
            while multiple <= n {
                let index = (multiple >> 3) as usize;
                let offset = multiple & 7; // Use '&' for bit operation
                is_prime[index] &= !(1 << offset);
                multiple += i;
            }
        }
    }

    let mut count = 0;
    for (i, &byte) in is_prime.iter().enumerate() {
        for j in 0..8 {
            if byte & (1 << j) != 0 {
                let num = i * 8 + j;
                if num as i32 <= n {
                    count += 1;
                }
            }
        }
    }

    count
}

use std::time::Instant;

fn main() {
    // Test for Power Calculation by Multiplication
    let base = 2.0;
    let exponent = 10;
    let start = Instant::now();
    let result_power_mul = power_by_multiplication(base, exponent);
    let duration_power_mul = start.elapsed();
    println!(
        "Power by Multiplication: {}, Time: {:?}",
        result_power_mul, duration_power_mul
    );

    // Test for Power Calculation by Binary Exponentiation
    let start = Instant::now();
    let result_power_bin = power_by_binary_exponentiation(base, exponent);
    let duration_power_bin = start.elapsed();
    println!(
        "Power by Binary Exponentiation: {}, Time: {:?}",
        result_power_bin, duration_power_bin
    );

    // Test for Fibonacci Calculation by Golden Ratio
    let fib_index = 10;
    let start = Instant::now();
    let result_fib_golden = fibonacci_by_golden_ratio(fib_index);
    let duration_fib_golden = start.elapsed();
    println!(
        "Fibonacci by Golden Ratio: {}, Time: {:?}",
        result_fib_golden, duration_fib_golden
    );

    // // Test for Fibonacci Calculation by Matrix Multiplication
    // let start = Instant::now();
    // let result_fib_matrix = fibonacci_by_matrix_multiplication(fib_index);
    // let duration_fib_matrix = start.elapsed();
    // println!("Fibonacci by Matrix Multiplication: {}, Time: {:?}", result_fib_matrix, duration_fib_matrix);

    // Test for Prime Counting by Optimized Method
    let prime_limit = 10000;
    let start = Instant::now();
    let result_primes_opt = count_primes_optimized(prime_limit);
    let duration_primes_opt = start.elapsed();
    println!(
        "Prime Counting Optimized: {}, Time: {:?}",
        result_primes_opt, duration_primes_opt
    );

    // Test for Prime Counting by Eratosthenes Sieve
    let start = Instant::now();
    let result_primes_erato = count_primes_eratosthenes(prime_limit);
    let duration_primes_erato = start.elapsed();
    println!(
        "Prime Counting Eratosthenes: {}, Time: {:?}",
        result_primes_erato, duration_primes_erato
    );

    // Test for Prime Counting by Eratosthenes with Optimized Memory
    let start = Instant::now();
    let result_primes_erato_mem_opt = count_primes_eratosthenes_optimized_memory(prime_limit);
    let duration_primes_erato_mem_opt = start.elapsed();
    println!(
        "Prime Counting Eratosthenes Memory Optimized: {}, Time: {:?}",
        result_primes_erato_mem_opt, duration_primes_erato_mem_opt
    );
}
