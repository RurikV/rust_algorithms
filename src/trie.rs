use std::collections::HashMap;
use std::time::{Instant, Duration};

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

pub struct Trie {
    root: TrieNode,
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

    pub fn delete(&mut self, word: &str) -> bool {
        Trie::delete_recursive(&mut self.root, word, 0).is_some()
    }

    fn delete_recursive(node: &mut TrieNode, word: &str, depth: usize) -> Option<bool> {
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

// AssociativeTrie implementation (corrected)
struct AssociativeTrieNode<T> {
    children: HashMap<char, AssociativeTrieNode<T>>,
    is_end_of_word: bool,
    value: Option<T>,
}

impl<T> AssociativeTrieNode<T> {
    fn new() -> Self {
        AssociativeTrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
            value: None,
        }
    }
}

pub struct AssociativeTrie<T> {
    root: AssociativeTrieNode<T>,
}

impl<T> AssociativeTrie<T> {
    pub fn new() -> Self {
        AssociativeTrie {
            root: AssociativeTrieNode::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        let mut node = &mut self.root;
        for ch in key.chars() {
            node = node.children.entry(ch).or_insert_with(AssociativeTrieNode::new);
        }
        node.is_end_of_word = true;
        node.value = Some(value);
    }

    pub fn search(&self, key: &str) -> bool {
        let mut node = &self.root;
        for ch in key.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_end_of_word
    }

    pub fn delete(&mut self, key: &str) -> bool {
        AssociativeTrie::delete_recursive(&mut self.root, key, 0).is_some()
    }

    fn delete_recursive(node: &mut AssociativeTrieNode<T>, key: &str, depth: usize) -> Option<bool> {
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
            if let Some(delete_child) = AssociativeTrie::delete_recursive(child, key, depth + 1) {
                if delete_child {
                    node.children.remove(&ch);
                }
                return Some(node.children.is_empty() && !node.is_end_of_word);
            }
        }
        None
    }
}

fn main() {
    let mut trie = Trie::new();
    let mut assoc_trie = AssociativeTrie::new();

    let words: Vec<String> = (0..100000).map(|i| format!("word{}", i)).collect();

    println!("Operation | Trie Time | AssociativeTrie Time");
    println!("----------|-----------|---------------------");

    // Insert
    let trie_insert_time = measure_time(|| {
        for word in &words {
            trie.insert(word);
        }
    });

    let assoc_trie_insert_time = measure_time(|| {
        for (i, word) in words.iter().enumerate() {
            assoc_trie.insert(word, i);
        }
    });

    println!("Insert    | {:9.6}s | {:19.6}s", trie_insert_time.as_secs_f64(), assoc_trie_insert_time.as_secs_f64());

    // Search
    let trie_search_time = measure_time(|| {
        for word in &words {
            trie.search(word);
        }
    });

    let assoc_trie_search_time = measure_time(|| {
        for word in &words {
            assoc_trie.search(word);
        }
    });

    println!("Search    | {:9.6}s | {:19.6}s", trie_search_time.as_secs_f64(), assoc_trie_search_time.as_secs_f64());

    // Delete
    let trie_delete_time = measure_time(|| {
        for word in &words {
            trie.delete(word);
        }
    });

    let assoc_trie_delete_time = measure_time(|| {
        for word in &words {
            assoc_trie.delete(word);
        }
    });

    println!("Delete    | {:9.6}s | {:19.6}s", trie_delete_time.as_secs_f64(), assoc_trie_delete_time.as_secs_f64());
}

fn measure_time<F>(f: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    start.elapsed()
}
