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

fn boyer_moore_search(text: &str, pattern: &str) -> Option<usize> {
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
        i += std::cmp::max(skip[text_chars[i] as usize], 1);
    }
    None
}

fn test_algorithms(text: &str, pattern: &str, iterations: u32) {
    let algorithms: [(&str, fn(&str, &str) -> Option<usize>); 4] = [
        ("Brute Force", brute_force_search),
        ("Prefix Shift", prefix_shift_search),
        ("Suffix Shift", suffix_shift_search),
        ("Boyer-Moore", boyer_moore_search),
    ];

    for (name, algorithm) in algorithms.iter() {
        let start = Instant::now();
        let mut result = None;
        for _ in 0..iterations {
            result = algorithm(text, pattern);
        }
        let duration = start.elapsed() / iterations;
        println!("{}: {:?}, Result: {:?}", name, duration, result);
    }
}

fn main() {
    let test_cases = [
        ("Hello, World!", "World"),
        ("aaaaaaaaaaaaaaaaaaab", "aaaab"),
        ("abcdefghijklmnopqrstuvwxyz", "xyz"),
        ("Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "consectetur"),
    ];

    for (i, (text, pattern)) in test_cases.iter().enumerate() {
        println!("Test case {}:", i + 1);
        test_algorithms(text, pattern, 10000);
        println!();
    }
}