use std::time::Instant;

fn brute_force_search(text: &str, pattern: &str) -> Option<usize> {
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    for i in 0..=text_chars.len() - pattern_chars.len() {
        if text_chars[i..i + pattern_chars.len()] == pattern_chars {
            return Some(i);
        }
    }
    None
}

fn prefix_shift_search(text: &str, pattern: &str) -> Option<usize> {
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    let mut i = 0;
    while i <= text_chars.len() - pattern_chars.len() {
        let mut j = pattern_chars.len() - 1;
        while j > 0 && pattern_chars[j] == text_chars[i + j] {
            j -= 1;
        }
        if j == 0 {
            return Some(i);
        }
        i += j;
    }
    None
}

fn suffix_shift_search(text: &str, pattern: &str) -> Option<usize> {
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    let mut skip = vec![pattern_chars.len(); 256];
    for (i, &c) in pattern_chars.iter().enumerate() {
        skip[c as usize] = pattern_chars.len() - i - 1;
    }

    let mut i = pattern_chars.len() - 1;
    while i < text_chars.len() {
        let mut j = pattern_chars.len() - 1;
        while j > 0 && pattern_chars[j] == text_chars[i] {
            i -= 1;
            j -= 1;
        }
        if j == 0 {
            return Some(i);
        }
        i += std::cmp::max(skip[text_chars[i] as usize], pattern_chars.len() - j);
    }
    None
}

use std::cmp;

fn boyer_moore_search(text: &str, pattern: &str) -> Option<usize> {
    let text = text.as_bytes();
    let pattern = pattern.as_bytes();
    
    if pattern.is_empty() { return Some(0); }
    if text.len() < pattern.len() { return None; }

    // Bad Character Heuristic
    let mut bad_char = [pattern.len(); 256];
    for (i, &c) in pattern.iter().enumerate() {
        bad_char[c as usize] = pattern.len() - 1 - i;
    }

    // Good Suffix Heuristic
    let mut good_suffix = vec![0; pattern.len() + 1];
    let mut last_prefix_position = pattern.len();

    // Case 2: suffix matches prefix
    for i in (0..pattern.len()).rev() {
        if is_prefix(pattern, i + 1) {
            last_prefix_position = i + 1;
        }
        good_suffix[i] = last_prefix_position - i + pattern.len() - 1;
    }

    // Case 1: suffix matches somewhere else
    for i in 0..pattern.len() - 1 {
        let len = suffix_length(pattern, i);
        if pattern[i - len] != pattern[pattern.len() - 1 - len] {
            good_suffix[pattern.len() - 1 - len] = pattern.len() - 1 - i + len;
        }
    }

    let mut i = pattern.len() - 1;
    unsafe {
        while i < text.len() {
            let mut j = pattern.len() - 1;
            while j > 0 && *pattern.get_unchecked(j) == *text.get_unchecked(i) {
                i -= 1;
                j -= 1;
            }
            if j == 0 && *pattern.get_unchecked(0) == *text.get_unchecked(i) {
                return Some(i);
            } else {
                let bad_char_shift = bad_char[*text.get_unchecked(i) as usize].saturating_sub(pattern.len() - 1 - j);
                let good_suffix_shift = *good_suffix.get_unchecked(j);
                i += cmp::max(bad_char_shift, good_suffix_shift);
            }
        }
    }
    None
}

fn is_prefix(pattern: &[u8], p: usize) -> bool {
    for i in p..pattern.len() {
        if pattern[i] != pattern[i - p] {
            return false;
        }
    }
    true
}

fn suffix_length(pattern: &[u8], p: usize) -> usize {
    let mut len = 0;
    let mut i = p;
    let mut j = pattern.len() - 1;
    while i > 0 && pattern[i - 1] == pattern[j] {
        len += 1;
        i -= 1;
        j -= 1;
    }
    len
}

fn main() {
    let test_cases = [
        ("Hello, World!", "World"),
        ("aaaaaaaaaaaaaaaaaaab", "aaaab"),
        ("abcdefghijklmnopqrstuvwxyz", "xyz"),
        ("Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "consectetur"),
        ("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.", "exercitation ullamco"),
    ];

    let iterations = [10_000, 100_000, 1_000_000];

    for &iter in &iterations {
        println!("\nResults for {} iterations:", iter);
        println!("| Test Case |  Brute Force | Prefix Shift | Suffix Shift |  Boyer-Moore |");
        println!("|-----------|--------------|--------------|--------------|--------------|");

        for (i, (text, pattern)) in test_cases.iter().enumerate() {
            print!("|  Case {}   | ", i + 1);
            test_algorithms(text, pattern, iter);
            println!();
        }
    }
}

fn test_algorithms(text: &str, pattern: &str, iterations: u32) {
    let algorithms: [(&str, fn(&str, &str) -> Option<usize>); 4] = [
        ("Brute Force", brute_force_search),
        ("Prefix Shift", prefix_shift_search),
        ("Suffix Shift", suffix_shift_search),
        ("Boyer-Moore", boyer_moore_search),
    ];

    for (_, algorithm) in algorithms.iter() {
        let start = Instant::now();
        let mut _result = None;
        for _ in 0..iterations {
            _result = algorithm(text, pattern);
        }
        let duration = start.elapsed() / iterations;
        print!("       {:?} | ", duration);
    }
}
