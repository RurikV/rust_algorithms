use rand::Rng;
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

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

struct MinHash {
    num_hashes: usize,
    hash_functions: Vec<Box<dyn Fn(&str) -> u64>>,
}

impl MinHash {
    fn new(num_hashes: usize) -> Self {
        let hash_functions = (0..num_hashes)
            .map(|i| {
                Box::new(move |s: &str| {
                    let mut hasher = DefaultHasher::new();
                    s.hash(&mut hasher);
                    hasher.finish().wrapping_add(i as u64)
                }) as Box<dyn Fn(&str) -> u64>
            })
            .collect();

        MinHash {
            num_hashes,
            hash_functions,
        }
    }

    fn compute_signature(&self, text: &str) -> Vec<u64> {
        let words: HashSet<&str> = text.split_whitespace().collect();
        let mut signature = vec![u64::MAX; self.num_hashes];

        for word in words {
            for (i, hash_fn) in self.hash_functions.iter().enumerate() {
                let hash = hash_fn(word);
                if hash < signature[i] {
                    signature[i] = hash;
                }
            }
        }

        signature
    }
}

struct SimHash {
    num_bits: usize,
}

impl SimHash {
    fn new(num_bits: usize) -> Self {
        SimHash { num_bits }
    }

    fn compute_hash(&self, text: &str) -> u64 {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut v = vec![0i32; self.num_bits];

        for word in words {
            let mut hasher = DefaultHasher::new();
            word.hash(&mut hasher);
            let hash = hasher.finish();

            for i in 0..self.num_bits {
                if (hash & (1 << i)) != 0 {
                    v[i] += 1;
                } else {
                    v[i] -= 1;
                }
            }
        }

        let mut fingerprint = 0u64;
        for i in 0..self.num_bits {
            if v[i] > 0 {
                fingerprint |= 1 << i;
            }
        }

        fingerprint
    }
}

fn jaccard_similarity(set1: &[u64], set2: &[u64]) -> f64 {
    let intersection = set1.iter().zip(set2).filter(|&(a, b)| a == b).count();
    intersection as f64 / set1.len() as f64
}

fn hamming_distance(x: u64, y: u64) -> u32 {
    (x ^ y).count_ones()
}

fn calculate_metrics(predictions: &[bool], actual: &[bool]) -> (f64, usize, usize, usize, usize) {
    let total = predictions.len();
    let mut correct = 0;
    let mut false_positives = 0;
    let mut true_negatives = 0;

    for (&pred, &act) in predictions.iter().zip(actual) {
        if pred == act {
            correct += 1;
            if !act {
                true_negatives += 1;
            }
        } else if pred && !act {
            false_positives += 1;
        }
    }

    let accuracy = (correct as f64 / total as f64) * 100.0;
    let false_positive_rate = if false_positives + true_negatives > 0 {
        (false_positives as f64 / (false_positives + true_negatives) as f64) * 100.0
    } else {
        0.0
    };

    (accuracy, total, correct, false_positives, true_negatives)
}

fn main() {
    let train_data = generate_dataset(1000);
    
    println!("First 5 entries of train data:");
    for line in train_data.lines().take(5) {
        println!("{}", line);
    }
    
    let mut bloom_filter = BloomFilter::new(10000, 5);
    let minhash = MinHash::new(100);
    let simhash = SimHash::new(64);
    
    let mut minhash_signatures = Vec::new();
    let mut simhash_hashes = Vec::new();
    
    for line in train_data.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            bloom_filter.insert(parts[1]);
            minhash_signatures.push(minhash.compute_signature(parts[1]));
            simhash_hashes.push(simhash.compute_hash(parts[1]));
        }
    }
    
    let total_tests = 10000;
    let mut rng = rand::thread_rng();
    
    let mut bloom_predictions = Vec::new();
    let mut minhash_predictions = Vec::new();
    let mut simhash_predictions = Vec::new();
    let mut actual = Vec::new();
    
    for _ in 0..total_tests {
        let index1 = rng.gen_range(0..1000);
        let index2 = rng.gen_range(0..1000);
        let text1 = train_data.lines().nth(index1).unwrap().split('\t').nth(1).unwrap();
        let text2 = train_data.lines().nth(index2).unwrap().split('\t').nth(1).unwrap();
        
        bloom_predictions.push(bloom_filter.might_contain(text1) && bloom_filter.might_contain(text2));
        minhash_predictions.push(jaccard_similarity(&minhash_signatures[index1], &minhash_signatures[index2]) > 0.5);
        simhash_predictions.push(hamming_distance(simhash_hashes[index1], simhash_hashes[index2]) < 32);
        actual.push(index1 == index2);
    }
    
    let (bloom_accuracy, bloom_total, bloom_correct, bloom_fp, bloom_tn) = calculate_metrics(&bloom_predictions, &actual);
    let (minhash_accuracy, minhash_total, minhash_correct, minhash_fp, minhash_tn) = calculate_metrics(&minhash_predictions, &actual);
    let (simhash_accuracy, simhash_total, simhash_correct, simhash_fp, simhash_tn) = calculate_metrics(&simhash_predictions, &actual);
    
    println!("\nBloom Filter:");
    println!("Accuracy: {:.2}%", bloom_accuracy);
    println!("False positive rate: {:.2}%", (bloom_fp as f64 / (bloom_fp + bloom_tn) as f64) * 100.0);
    println!("Total predictions: {}", bloom_total);
    println!("Correct predictions: {}", bloom_correct);
    println!("False positives: {}", bloom_fp);
    println!("True negatives: {}", bloom_tn);
    
    println!("\nMinHash:");
    println!("Accuracy: {:.2}%", minhash_accuracy);
    println!("False positive rate: {:.2}%", (minhash_fp as f64 / (minhash_fp + minhash_tn) as f64) * 100.0);
    println!("Total predictions: {}", minhash_total);
    println!("Correct predictions: {}", minhash_correct);
    println!("False positives: {}", minhash_fp);
    println!("True negatives: {}", minhash_tn);
    
    println!("\nSimHash:");
    println!("Accuracy: {:.2}%", simhash_accuracy);
    println!("False positive rate: {:.2}%", (simhash_fp as f64 / (simhash_fp + simhash_tn) as f64) * 100.0);
    println!("Total predictions: {}", simhash_total);
    println!("Correct predictions: {}", simhash_correct);
    println!("False positives: {}", simhash_fp);
    println!("True negatives: {}", simhash_tn);
}