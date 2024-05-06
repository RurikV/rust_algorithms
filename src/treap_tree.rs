type NodePointer<T> = Option<Box<Node<T>>>;

pub struct Node<T: Copy + PartialOrd> {
    pub value: T,
    left: NodePointer<T>,
    right: NodePointer<T>
}

impl<T: Copy + PartialOrd> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value, left: None, right: None }
    }
}

pub struct Tree<T: Copy + PartialOrd> {
    pub root: NodePointer<T>
}

// Finds a node with maximum value in a tree with the given root.
fn max_node_mut<T: Copy + PartialOrd>(root: &mut NodePointer<T>) -> &mut NodePointer<T> {
    let mut current = root;

    while current.is_some() && current.as_ref().unwrap().right.is_some() {
        let node = current.as_mut().unwrap();
        current = &mut node.right;
    }

    current
}

/// Deletes a given node. It must be a pointer to non-None node.
fn delete_node<T: Copy + PartialOrd>(root: &mut NodePointer<T>) {
    let mut this = root.take().unwrap();
    let left = this.left.take();
    let right = this.right.take();

    if left.is_none() && right.is_none() {
    }
    else if left.is_none() {
        root.replace(right.unwrap());
    }
    else if right.is_none() {
        root.replace(left.unwrap());
    }
    else {
        this.left = left;
        this.right = right;
        let next = max_node_mut(&mut this.left);
        this.value = next.as_ref().unwrap().value;
        delete_node(next);
        root.replace(this);
    }
}

/// The Drop trail for a tree. Because this data structure is recursive, the default, compiler-generated
/// implementation may cause stack overflows. 
/// This great link contains more details: https://rust-unofficial.github.io/too-many-lists/first-drop.html
impl<T: Copy + PartialOrd> Drop for Tree<T> {
    fn drop(&mut self) {
        if let Some(root_node) = self.root.take() {
            let mut stack = Vec::new();
            stack.push(root_node);
            
            while let Some(mut current_node) = stack.pop() {
                if let Some(left_node) = current_node.left.take() {
                    stack.push(left_node);
                }
                if let Some(right_node) = current_node.right.take() {
                    stack.push(right_node);
                }
            }
        }
    }
}

impl<T: Copy + PartialOrd> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}
/// Implementation for the Binary Search Tree.
impl<T: Copy + PartialOrd> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { root: None }
    }

    
    pub fn contains(self: &Tree<T>, value: T) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            if value == node.value {
                return true;
            }
            else if value < node.value {
                current = &node.left;
            }
            else {
                current = &node.right;
            }
        }
        
        false
    }
    
    pub fn insert(self: &mut Tree<T>, value: T) {
        let mut current = &mut self.root;
        
        while let Some(node) = current {
            if value <= node.value {
                current = &mut node.left;
            }
            else {
                current = &mut node.right;
            }
        }

        let new_node = Box::new(Node::new(value));
        current.replace(new_node);
    }

    pub fn search(self: &Tree<T>, value: T) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            if value == node.value {
                return true;
            }
            else if value < node.value {
                current = &node.left;
            }
            else {
                current = &node.right;
            }
        }

        false
    }

    pub fn remove(self: &mut Tree<T>, value: T) -> bool {
        let mut current = &mut self.root;

        while current.is_some() {
            let current_value  = current.as_ref().unwrap().value;
            if value == current_value {
                delete_node(current);
                return true;
            }
            if value < current_value {
                current = &mut current.as_mut().unwrap().left;
            }
            else {
                current = &mut current.as_mut().unwrap().right;
            }
        }

        false
    }

    // Finds the minimum value in the tree.
    pub fn min(self: &Tree<T>) -> Option<T> {
        let mut current = &self.root;

        while let Some(node) = current {
            match &node.left {
                Some(_) => {
                    current = &node.left;
                },
                None => {
                    return Some(node.value);
                }
            }
        }

        None
    }

    // Finds the maximum value in the tree.
    pub fn max(self: &Tree<T>) -> Option<T> {
        let mut current = &self.root;
        
        while let Some(node) = current {
            match &node.right {
                Some(_) => {
                    current = &node.right;
                },
                None => {
                    return Some(node.value);
                }
            }
        }

        None
    }
    
    pub fn iter<'a>(self: &'a Tree<T>) -> TreeIntoIterator<'a, T> {
        let mut stack = Vec::new();
    
        if let Some(node) = &self.root {
            stack.push((false, &**node));
        }
    
        TreeIntoIterator { stack }
    }
    
}

pub struct TreeIntoIterator<'a, T: Copy + PartialOrd> {
    stack: Vec<(bool, &'a Node<T>)>
}

impl<'a, T: Copy + PartialOrd> Iterator for TreeIntoIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;

        while let Some((left_visited, node)) = self.stack.pop() {
            if left_visited {
                if let Some(right) = &node.right {
                    self.stack.push((false, &right));
                }
                result = Some(node.value);
                break;
            }
            else {
                self.stack.push((true, node));
                if let Some(left) = &node.left {
                    self.stack.push((false, &left));
                }
            }
        }
        
        result
    }
}

extern crate rand; // 0.8.5
use rand::Rng;
use std::time::Instant;

fn main() {
    let n = 10_000; // Adjust N based on performance requirements
    let mut rng = rand::thread_rng();

    // Random Order Tree
    let mut random_tree = Tree::new();
    let start_random_insert = Instant::now();
    for _ in 0..n {
        let value = rng.gen_range(0..n);
        random_tree.insert(value);
    }
    let random_insert_duration = start_random_insert.elapsed();

    // Ascending Order Tree
    let mut ascending_tree = Tree::new();
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
