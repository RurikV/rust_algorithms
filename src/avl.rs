#[derive(Clone, Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    height: i32,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }

    fn height(node: &Option<Box<TreeNode>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn update_height(node: &mut Box<TreeNode>) {
        node.height = 1 + i32::max(Self::height(&node.left), Self::height(&node.right));
    }

    fn balance_factor(node: &Option<Box<TreeNode>>) -> i32 {
        node.as_ref()
            .map_or(0, |n| Self::height(&n.left) - Self::height(&n.right))
    }

    fn rotate_left(mut node: Box<TreeNode>) -> Box<TreeNode> {
        let mut right = node.right.take().unwrap();
        node.right = right.left.take();
        Self::update_height(&mut node);
        right.left = Some(node);
        Self::update_height(&mut right);
        right
    }

    fn rotate_right(mut node: Box<TreeNode>) -> Box<TreeNode> {
        let mut left = node.left.take().unwrap();
        node.left = left.right.take();
        Self::update_height(&mut node);
        left.right = Some(node);
        Self::update_height(&mut left);
        left
    }

    fn balance(mut node: Box<TreeNode>) -> Box<TreeNode> {
        Self::update_height(&mut node);
        let balance = Self::balance_factor(&Some(node.clone()));

        if balance > 1 {
            if Self::balance_factor(&node.left) < 0 {
                node.left = Some(Self::rotate_left(node.left.take().unwrap()));
            }
            return Self::rotate_right(node);
        }
        if balance < -1 {
            if Self::balance_factor(&node.right) > 0 {
                node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            }
            return Self::rotate_left(node);
        }
        node
    }

    fn insert(node: Option<Box<TreeNode>>, value: i32) -> Box<TreeNode> {
        let mut node = match node {
            Some(node) => node,
            None => return Box::new(TreeNode::new(value)),
        };

        match value.cmp(&node.value) {
            std::cmp::Ordering::Less => {
                node.left = Some(Self::insert(node.left.take(), value));
            }
            std::cmp::Ordering::Greater => {
                node.right = Some(Self::insert(node.right.take(), value));
            }
            std::cmp::Ordering::Equal => {
                return node; // Duplicate values not allowed
            }
        }

        Self::balance(node)
    }
}

struct AVLTree {
    root: Option<Box<TreeNode>>,
}

impl AVLTree {
    fn new() -> Self {
        AVLTree { root: None }
    }

    fn insert(&mut self, value: i32) {
        self.root = Some(TreeNode::insert(self.root.take(), value));
    }

    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
            }
        }

        false // Not found
    }

    fn remove(&mut self, value: i32) {
        self.root = Self::remove_helper(self.root.take(), value);
    }

    fn remove_helper(node: Option<Box<TreeNode>>, value: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => {
                        node.left = Self::remove_helper(node.left.take(), value);
                    }
                    Ordering::Greater => {
                        node.right = Self::remove_helper(node.right.take(), value);
                    }
                    Ordering::Equal => {
                        if node.left.is_none() {
                            return node.right; // Replace with right child
                        } else if node.right.is_none() {
                            return node.left; // Replace with left child
                        } else {
                            // Two children: Find inorder successor (smallest in right subtree)
                            let successor_value = Self::find_min(&node.right).value;
                            node.value = successor_value;
                            node.right = Self::remove_helper(node.right.take(), successor_value);
                        }
                    }
                }

                Some(TreeNode::balance(node)) // Rebalance
            }
            None => None, // Not found
        }
    }

    fn find_min(node: &Option<Box<TreeNode>>) -> &TreeNode {
        let mut current = node.as_ref().unwrap(); // Safe because node can't be None here
        while let Some(left) = &current.left {
            current = left;
        }
        current
    }
}

extern crate rand; // 0.8.5
use rand::Rng;
use std::{cmp::Ordering, time::Instant};

fn main() {
    let n = 10_000; // Adjust N based on performance requirements
    let mut rng = rand::thread_rng();

    // Random Order Tree
    let mut random_tree = AVLTree::new();
    let start_random_insert = Instant::now();
    for _ in 0..n {
        let value = rng.gen_range(0..n);
        random_tree.insert(value);
    }
    let random_insert_duration = start_random_insert.elapsed();

    // Ascending Order Tree
    let mut ascending_tree = AVLTree::new();
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
    println!(
        "Insertion (Ascending Order): {:?}",
        ascending_insert_duration
    );
    println!("Search (Random Order): {:?}", random_search_duration);
    println!("Search (Ascending Order): {:?}", ascending_search_duration);
    println!("Delete (Random Order): {:?}", random_delete_duration);
    println!("Delete (Ascending Order): {:?}", ascending_delete_duration);
}
