fn build_kmp_automaton(pattern: &str) -> Vec<Vec<usize>> {
    let m = pattern.len();
    let mut automaton = vec![vec![0; 256]; m + 1];
    let mut lps = vec![0; m + 1];
    
    // Making the LPS table (Longest Proper Prefix which is also Suffix)
    let mut len = 0;
    let mut i = 1;
    while i < m {
        if pattern.as_bytes()[i] == pattern.as_bytes()[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    
    // Making the automaton
    for j in 0..256 {
        automaton[0][j] = 0;
    }
    automaton[0][pattern.as_bytes()[0] as usize] = 1;
    
    for i in 1..=m {
        for j in 0..256 {
            if i < m && j == pattern.as_bytes()[i] as usize {
                automaton[i][j] = i + 1;
            } else {
                automaton[i][j] = automaton[lps[i - 1]][j];
            }
        }
    }
    
    automaton
}
fn kmp_search_automaton(text: &str, pattern: &str) -> Option<usize> {
    let automaton = build_kmp_automaton(pattern);
    let mut state = 0;

    for (i, &c) in text.as_bytes().iter().enumerate() {
        state = automaton[state][c as usize];
        if state == pattern.len() {
            return Some(i - pattern.len() + 1);
        }
    }

    None
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
use std::fmt::Write;

fn test_algorithms(text: &str, pattern: &str, iterations: u32) -> (String, String) {
    let algorithms: [(&str, fn(&str, &str) -> Option<usize>); 2] = [
        ("KMP Automaton", kmp_search_automaton),
        ("KMP", kmp_search),
    ];

    let mut results = ("".to_string(), "".to_string());

    for (i, (_, algorithm)) in algorithms.iter().enumerate() {
        let start = Instant::now();
        let mut result = None;
        for _ in 0..iterations {
            result = algorithm(text, pattern);
        }
        let duration = start.elapsed() / iterations;
        let _ = write!(
            if i == 0 { &mut results.0 } else { &mut results.1 },
            "{:7} ({:9})",
            format!("{:?}", duration),
            if result.is_some() { "Found" } else { "Not found" }
        );
    }

    results
}

fn main() {
    let test_cases = [
        ("Hello, World!", "World"),
        ("aaaaaaaaaaaaaaaaaaab", "aaaab"),
        ("abcdefghijklmnopqrstuvwxyz", "xyz"),
        ("Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "consectetur"),
        ("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.", "exercitation ullamco"),
    ];

    for iterations in [1_000, 10_000, 100_000] {
        println!("\nResults for {} iterations:", iterations);
        println!("| {:^9} | {:^21} | {:^21} |", "Test Case", "KMP Automaton", "KMP");
        println!("|{:-^11}|{:-^23}|{:-^23}|", "", "", "");

        for (i, (text, pattern)) in test_cases.iter().enumerate() {
            let (automaton_result, kmp_result) = test_algorithms(text, pattern, iterations);
            println!("| Case {:<4} | {} | {} |", i + 1, automaton_result, kmp_result);
        }
    }
}