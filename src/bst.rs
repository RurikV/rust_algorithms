#[derive(Clone)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode { value, left: None, right: None }
    }
}

struct BST {
    root: Option<Box<TreeNode>>,
}

impl BST {
    fn new() -> Self {
        BST { root: None }
    }

    fn insert(&mut self, value: i32) {
        self.root = Self::insert_rec(self.root.take(), value);
    }

    fn insert_rec(node: Option<Box<TreeNode>>, value: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut current) => {
                if value < current.value {
                    current.left = Self::insert_rec(current.left.take(), value);
                } else if value > current.value {
                    current.right = Self::insert_rec(current.right.take(), value);
                }
                Some(current)
            }
            None => Some(Box::new(TreeNode::new(value))),
        }
    }

    fn search(&self, value: i32) -> bool {
        Self::search_rec(&self.root, value)
    }

    fn search_rec(node: &Option<Box<TreeNode>>, value: i32) -> bool {
        match node {
            Some(current) => {
                if value == current.value {
                    true
                } else if value < current.value {
                    Self::search_rec(&current.left, value)
                } else {
                    Self::search_rec(&current.right, value)
                }
            }
            None => false,
        }
    }

    fn remove(&mut self, value: i32) {
        self.root = Self::remove_rec(self.root.take(), value);
    }

    fn remove_rec(node: Option<Box<TreeNode>>, value: i32) -> Option<Box<TreeNode>> {
        node.map(|mut current| {
            if value < current.value {
                current.left = Self::remove_rec(current.left.take(), value);
                return Some(current);
            } else if value > current.value {
                current.right = Self::remove_rec(current.right.take(), value);
                return Some(current);
            }

            match (current.left.take(), current.right.take()) {
                (None, None) => None,
                (Some(left), None) => Some(left),
                (None, Some(right)) => Some(right),
                (Some(_left), Some(right)) => {
                    let my_option_box_tree_node: Option<Box<TreeNode>> = Some(right.clone());
                    let min_val = Self::find_min(&my_option_box_tree_node);
                    current.value = min_val;
                    current.right = Self::remove_rec(Some(right), min_val);
                    Some(current)
                }
            }
        }).flatten()
    }

    fn find_min(node: &Option<Box<TreeNode>>) -> i32 {
        node.as_ref().map(|current| {
            match &current.left {
                Some(left) => Self::find_min(&Some(Box::clone(left))),
                None => current.value,
            }
        }).unwrap()
    }
}

extern crate rand; // 0.8.5
use rand::Rng;
use std::time::{Duration, Instant};

fn main() {
    let n = 10_000; // Adjust N based on performance requirements
    let mut rng = rand::thread_rng();

    // Random Order Tree
    let mut random_tree = BST::new();
    let start_random_insert = Instant::now();
    for _ in 0..n {
        let value = rng.gen_range(0..n);
        random_tree.insert(value);
    }
    let random_insert_duration = start_random_insert.elapsed();

    // Ascending Order Tree
    let mut ascending_tree = BST::new();
    let start_ascending_insert = Instant::now();
    for value in 0..n {
        ascending_tree.insert(value);
    }
    let ascending_insert_duration = start_ascending_insert.elapsed();

    // Search Operation
    let start_random_search = Instant::now();
    for _ in 0..n / 10 {
        let value = rng.gen_range(0..n);
        random_tree.search(value);
    }
    let random_search_duration = start_random_search.elapsed();

    let start_ascending_search = Instant::now();
    for _ in 0..n / 10 {
        let value = rng.gen_range(0..n);
        ascending_tree.search(value);
    }
    let ascending_search_duration = start_ascending_search.elapsed();

    // Delete Operation
    let start_random_delete = Instant::now();
    for _ in 0..n / 10 {
        let value = rng.gen_range(0..n);
        random_tree.remove(value);
    }
    let random_delete_duration = start_random_delete.elapsed();

    let start_ascending_delete = Instant::now();
    for _ in 0..n / 10 {
        let value = rng.gen_range(0..n);
        ascending_tree.remove(value);
    }
    let ascending_delete_duration = start_ascending_delete.elapsed();

    // Results
    println!("Insertion (Random Order): {:?}", random_insert_duration);
    println!("Insertion (Ascending Order): {:?}", ascending_insert_duration);
    println!("Search (Random Order): {:?}", random_search_duration);
    println!("Search (Ascending Order): {:?}", ascending_search_duration);
    println!("Delete (Random Order): {:?}", random_delete_duration);
    println!("Delete (Ascending Order): {:?}", ascending_delete_duration);
}
