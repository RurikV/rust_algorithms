use std::collections::HashMap;
use std::time::{Instant, Duration};

// Extended TrieNode to support both Trie and AssociativeArray
struct TrieNode<T> {
    children: HashMap<char, TrieNode<T>>,
    is_end_of_word: bool,
    value: Option<T>,
}

impl<T> TrieNode<T> {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
            value: None,
        }
    }
}

impl TrieNode<()> {
    fn default() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
            value: None,
        }
    }
}

// Trie Implementation
pub struct Trie {
    root: TrieNode<()>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::default);
        }
        node.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_end_of_word
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return false,
            }
        }
        true
    }

    pub fn delete(&mut self, word: &str) -> bool {
        Trie::delete_recursive(&mut self.root, word, 0).is_some()
    }

    fn delete_recursive(node: &mut TrieNode<()>, word: &str, depth: usize) -> Option<bool> {
        if depth == word.len() {
            if !node.is_end_of_word {
                return None;
            }
            node.is_end_of_word = false;
            return Some(node.children.is_empty());
        }

        let ch = word.chars().nth(depth)?;
        if let Some(child) = node.children.get_mut(&ch) {
            if let Some(delete_child) = Trie::delete_recursive(child, word, depth + 1) {
                if delete_child {
                    node.children.remove(&ch);
                }
                return Some(node.children.is_empty() && !node.is_end_of_word);
            }
        }
        None
    }
}

// AssociativeArray using the extended TrieNode
pub struct AssociativeArray<T> {
    root: TrieNode<T>,
}

impl<T> AssociativeArray<T> {
    pub fn new() -> Self {
        AssociativeArray {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        let mut node = &mut self.root;
        for ch in key.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.is_end_of_word = true;
        node.value = Some(value);
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        let mut node = &self.root;
        for ch in key.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return None,
            }
        }
        if node.is_end_of_word {
            node.value.as_ref()
        } else {
            None
        }
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return false,
            }
        }
        true
    }

    pub fn delete(&mut self, key: &str) -> bool {
        AssociativeArray::delete_recursive(&mut self.root, key, 0).is_some()
    }

    fn delete_recursive(node: &mut TrieNode<T>, key: &str, depth: usize) -> Option<bool> {
        if depth == key.len() {
            if !node.is_end_of_word {
                return None;
            }
            node.is_end_of_word = false;
            node.value = None;
            return Some(node.children.is_empty());
        }

        let ch = key.chars().nth(depth)?;
        if let Some(child) = node.children.get_mut(&ch) {
            if let Some(delete_child) = AssociativeArray::delete_recursive(child, key, depth + 1) {
                if delete_child {
                    node.children.remove(&ch);
                }
                return Some(node.children.is_empty() && !node.is_end_of_word);
            }
        }
        None
    }
}

// Main Function for Testing
fn main() {
    // Mandatory Tests
    let mut trie = Trie::new();
    trie.insert("apple");
    println!("search('apple'): {}", trie.search("apple")); // return True
    println!("search('app'): {}", trie.search("app"));     // return False
    println!("starts_with('app'): {}", trie.starts_with("app")); // return True
    trie.insert("app");
    println!("search('app'): {}", trie.search("app"));     // return True

    let mut assoc_array = AssociativeArray::new();

    let words: Vec<String> = (0..10_000_000).map(|i| format!("word{}", i)).collect();

    println!("\nOperation  | Trie Time | AssociativeArray Time");
    println!("-----------|-----------|---------------------");

    // Insert
    let trie_insert_time = measure_time(|| {
        for word in &words {
            trie.insert(word);
        }
    });

    let assoc_array_insert_time = measure_time(|| {
        for (i, word) in words.iter().enumerate() {
            assoc_array.insert(word, i);
        }
    });

    println!("Insert     | {:9.6}s | {:19.6}s", trie_insert_time.as_secs_f64(), assoc_array_insert_time.as_secs_f64());

    // Search
    let trie_search_time = measure_time(|| {
        for word in &words {
            trie.search(word);
        }
    });

    let assoc_array_search_time = measure_time(|| {
        for word in &words {
            assoc_array.get(word);
        }
    });

    println!("Search     | {:9.6}s | {:19.6}s", trie_search_time.as_secs_f64(), assoc_array_search_time.as_secs_f64());

    // Starts With
    let trie_starts_with_time = measure_time(|| {
        for word in &words {
            trie.starts_with(&word[0..3]);
        }
    });

    let assoc_array_starts_with_time = measure_time(|| {
        for word in &words {
            assoc_array.starts_with(&word[0..3]);
        }
    });

    println!("StartsWith | {:9.6}s | {:19.6}s", trie_starts_with_time.as_secs_f64(), assoc_array_starts_with_time.as_secs_f64());

    // Delete
    let trie_delete_time = measure_time(|| {
        for word in &words {
            trie.delete(word);
        }
    });

    let assoc_array_delete_time = measure_time(|| {
        for word in &words {
            assoc_array.delete(word);
        }
    });

    println!("Delete     | {:9.6}s | {:19.6}s", trie_delete_time.as_secs_f64(), assoc_array_delete_time.as_secs_f64());
}

// Measure time function
fn measure_time<F>(f: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    start.elapsed()
}
