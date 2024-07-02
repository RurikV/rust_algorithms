use std::collections::HashMap;

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

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::default);
        }
        node.is_end_of_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_end_of_word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return false,
            }
        }
        true
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    println!("{}", trie.search("apple".to_string()));   // return True
    println!("{}", trie.search("app".to_string()));     // return False
    println!("{}", trie.starts_with("app".to_string())); // return True
    trie.insert("app".to_string());
    println!("{}", trie.search("app".to_string()));     // return True
}
