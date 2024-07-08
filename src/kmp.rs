fn kmp_automaton(pattern: &str) -> Vec<Vec<usize>> {
    let m = pattern.len();
    let alphabet: Vec<char> = pattern.chars().collect::<std::collections::HashSet<_>>().into_iter().collect();
    let mut automaton = vec![vec![0; alphabet.len()]; m + 1];

    for i in 0..=m {
        for (j, &c) in alphabet.iter().enumerate() {
            if i < m && pattern.chars().nth(i) == Some(c) {
                automaton[i][j] = i + 1;
            } else {
                automaton[i][j] = if i > 0 { automaton[kmp_prefix_function(pattern)[i - 1]][j] } else { 0 };
            }
        }
    }

    automaton
}

fn kmp_search_automaton(text: &str, pattern: &str) -> Option<usize> {
    let automaton = kmp_automaton(pattern);
    let alphabet: Vec<char> = pattern.chars().collect::<std::collections::HashSet<_>>().into_iter().collect();
    let mut state = 0;

    for (i, c) in text.char_indices() {
        if let Some(j) = alphabet.iter().position(|&x| x == c) {
            state = automaton[state][j];
        } else {
            state = 0;
        }
        if state == pattern.len() {
            return Some(i - pattern.len() + 1);
        }
    }

    None
}

fn kmp_prefix_function_slow(pattern: &str) -> Vec<usize> {
    let m = pattern.len();
    let mut pi = vec![0; m];
    
    for i in 1..m {
        for j in 0..=i {
            if pattern[..j] == pattern[i-j+1..=i] {
                pi[i] = j;
            }
        }
    }
    
    pi
}

fn kmp_prefix_function(pattern: &str) -> Vec<usize> {
    let m = pattern.len();
    let mut pi = vec![0; m];
    let mut k = 0;
    
    for i in 1..m {
        while k > 0 && pattern.chars().nth(k) != pattern.chars().nth(i) {
            k = pi[k - 1];
        }
        if pattern.chars().nth(k) == pattern.chars().nth(i) {
            k += 1;
        }
        pi[i] = k;
    }
    
    pi
}

fn kmp_search(text: &str, pattern: &str) -> Option<usize> {
    let pi = kmp_prefix_function(pattern);
    let mut j = 0;
    
    for (i, c) in text.char_indices() {
        while j > 0 && pattern.chars().nth(j) != Some(c) {
            j = pi[j - 1];
        }
        if pattern.chars().nth(j) == Some(c) {
            j += 1;
        }
        if j == pattern.len() {
            return Some(i - pattern.len() + 1);
        }
    }
    
    None
}

use std::time::Instant;

fn test_algorithms(text: &str, pattern: &str, iterations: u32) {
    let algorithms: [(&str, fn(&str, &str) -> Option<usize>); 2] = [
        ("KMP Automaton", kmp_search_automaton),
        ("KMP", kmp_search),
    ];

    for (_name, algorithm) in algorithms.iter() {
        let start = Instant::now();
        let mut result = None;
        for _ in 0..iterations {
            result = algorithm(text, pattern);
        }
        let duration = start.elapsed() / iterations;
        print!("{:?} ({}) | ", duration, result.map_or("Not found", |_v| "Found"));
    }
    println!();
}

fn main() {
    let test_cases = [
        ("Hello, World!", "World"),
        ("aaaaaaaaaaaaaaaaaaab", "aaaab"),
        ("abcdefghijklmnopqrstuvwxyz", "xyz"),
        ("Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "consectetur"),
        ("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.", "exercitation ullamco"),
    ];

    for iterations in [1000, 10_000, 100_000] {
        println!("\nResults for {} iterations:", iterations);
        println!("| Test Case | KMP Automaton |      KMP    |");
        println!("|-----------|---------------|-------------|");
        for (i, (text, pattern)) in test_cases.iter().enumerate() {
            print!("| Case {} | ", i + 1);
            test_algorithms(text, pattern, iterations);
        }
    }
}