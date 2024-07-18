use rand::Rng;
use std::collections::{HashSet, HashMap};

fn generate_word(rng: &mut impl Rng) -> String {
    let length = rng.gen_range(3..10);
    (0..length)
        .map(|_| rng.gen_range(b'a'..=b'z') as char)
        .collect()
}

fn generate_article(id: usize, rng: &mut impl Rng) -> String {
    let word_count = rng.gen_range(50..200);
    let words: Vec<String> = (0..word_count).map(|_| generate_word(rng)).collect();
    format!("{}\t{}", id, words.join(" "))
}

fn generate_dataset(article_count: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut train_data = String::new();

    for id in 0..article_count {
        let article = generate_article(id, &mut rng);
        train_data.push_str(&article);
        train_data.push('\n');
    }

    train_data
}

struct BloomFilter {
    size: usize,
    bit_array: Vec<bool>,
    hash_functions: Vec<Box<dyn Fn(&str) -> usize>>,
}

impl BloomFilter {
    fn new(size: usize, num_hash_functions: usize) -> Self {
        let bit_array = vec![false; size];
        let hash_functions = (0..num_hash_functions)
            .map(|i| {
                Box::new(move |s: &str| {
                    let mut hash = 5381u64;
                    for b in s.bytes() {
                        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(b as u64);
                    }
                    (hash.wrapping_add(i as u64) % size as u64) as usize
                }) as Box<dyn Fn(&str) -> usize>
            })
            .collect();

        BloomFilter {
            size,
            bit_array,
            hash_functions,
        }
    }

    fn insert(&mut self, item: &str) {
        for hash_fn in &self.hash_functions {
            let index = hash_fn(item);
            self.bit_array[index] = true;
        }
    }

    fn might_contain(&self, item: &str) -> bool {
        self.hash_functions
            .iter()
            .all(|hash_fn| self.bit_array[hash_fn(item)])
    }
}

fn main() {
    let train_data = generate_dataset(1000);
    
    println!("First 5 entries of train data:");
    for line in train_data.lines().take(5) {
        println!("{}", line);
    }
    
    // Create Bloom filter
    let mut bloom_filter = BloomFilter::new(10000, 5);
    
    // Populate Bloom filter with data from train_data
    for line in train_data.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            bloom_filter.insert(parts[1]);
        }
    }
    
    // Test Bloom filter
    let mut correct_predictions = 0;
    let mut total_predictions = 0;
    let mut false_positives = 0;
    let mut true_negatives = 0;
    
    let mut test_set = HashSet::new();
    
    for line in train_data.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            test_set.insert(parts[1].to_string());
        }
    }
    
    // Generate some random strings for negative tests
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let random_string: String = (0..50).map(|_| rng.gen_range(b'a'..=b'z') as char).collect();
        if !test_set.contains(&random_string) {
            total_predictions += 1;
            if bloom_filter.might_contain(&random_string) {
                false_positives += 1;
            } else {
                true_negatives += 1;
                correct_predictions += 1;
            }
        }
    }
    
    // Test positive cases
    for item in test_set {
        total_predictions += 1;
        if bloom_filter.might_contain(&item) {
            correct_predictions += 1;
        }
    }
    
    let accuracy = (correct_predictions as f64 / total_predictions as f64) * 100.0;
    let false_positive_rate = (false_positives as f64 / (false_positives + true_negatives) as f64) * 100.0;
    
    println!("\nBloom filter accuracy: {:.2}%", accuracy);
    println!("False positive rate: {:.2}%", false_positive_rate);
    println!("Total predictions: {}", total_predictions);
    println!("Correct predictions: {}", correct_predictions);
    println!("False positives: {}", false_positives);
    println!("True negatives: {}", true_negatives);
}