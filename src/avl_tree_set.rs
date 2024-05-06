use core::iter::Peekable;
use std::cmp::Ordering;
use std::iter::FromIterator;
use std::mem::replace;

#[derive(Debug, PartialEq, Clone)]
pub struct AvlTreeSet<T: Ord> {
    root: AvlTree<T>,
}

impl<'a, T: 'a + Ord> Default for AvlTreeSet<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<'a, T: 'a + Ord> AvlTreeSet<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let mut prev_ptrs = Vec::<*mut AvlNode<T>>::new();
        let mut current_tree = &mut self.root;

        while let Some(current_node) = current_tree {
            prev_ptrs.push(&mut **current_node);

            match current_node.value.cmp(&value) {
                Ordering::Less => current_tree = &mut current_node.right,
                Ordering::Equal => {
                    return false;
                }
                Ordering::Greater => current_tree = &mut current_node.left,
            }
        }

        *current_tree = Some(Box::new(AvlNode {
            value,
            left: None,
            right: None,
            height: 1,
        }));

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
            node.rebalance();
        }

        true
    }

    pub fn search(&mut self, value: &T) -> Option<T> {
        let mut prev_ptrs = Vec::<*mut AvlNode<T>>::new();
        let mut current_tree = &mut self.root;
        let mut target_value = None;

        while let Some(current_node) = current_tree {
            match current_node.value.cmp(value) {
                Ordering::Less => {
                    prev_ptrs.push(&mut **current_node);
                    current_tree = &mut current_node.right;
                }
                Ordering::Equal => {
                    target_value = Some(&mut **current_node);
                    break;
                }
                Ordering::Greater => {
                    prev_ptrs.push(&mut **current_node);
                    current_tree = &mut current_node.left;
                }
            };
        }

        let target_node = target_value?;

        let taken_value = if target_node.left.is_none() || target_node.right.is_none() {
            if let Some(left_node) = target_node.left.take() {
                replace(target_node, *left_node).value
            } else if let Some(right_node) = target_node.right.take() {
                replace(target_node, *right_node).value
            } else if let Some(prev_ptr) = prev_ptrs.pop() {
                let prev_node = unsafe { &mut *prev_ptr };

                let inner_value = if let Some(left_node) = prev_node.left.as_ref() {
                    if left_node.value == target_node.value {
                        prev_node.left.take().unwrap().value
                    } else {
                        prev_node.right.take().unwrap().value
                    }
                } else {
                    prev_node.right.take().unwrap().value
                };

                prev_node.update_height();
                prev_node.rebalance();

                inner_value
            } else {
                self.root.take().unwrap().value
            }
        } else {
            let right_tree = &mut target_node.right;

            if right_tree.as_ref().unwrap().left.is_none() {
                let mut right_node = right_tree.take().unwrap();

                let inner_value = replace(&mut target_node.value, right_node.value);
                target_node.right = right_node.right.take();

                target_node.update_height();
                target_node.rebalance();

                inner_value
            } else {
                let mut next_tree = right_tree;
                let mut inner_ptrs = Vec::<*mut AvlNode<T>>::new();

                while let Some(next_left_node) = next_tree {
                    if next_left_node.left.is_some() {
                        inner_ptrs.push(&mut **next_left_node);
                    }
                    next_tree = &mut next_left_node.left;
                }

                let parent_left_node = unsafe { &mut *inner_ptrs.pop().unwrap() };
                let mut leftmost_node = parent_left_node.left.take().unwrap();

                let inner_value = replace(&mut target_node.value, leftmost_node.value);
                parent_left_node.left = leftmost_node.right.take();

                parent_left_node.update_height();
                parent_left_node.rebalance();

                for node_ptr in inner_ptrs.into_iter().rev() {
                    let node = unsafe { &mut *node_ptr };
                    node.update_height();
                    node.rebalance();
                }

                target_node.update_height();
                target_node.rebalance();

                inner_value
            }
        };

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
            node.rebalance();
        }

        Some(taken_value)
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.search(value).is_some()
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut current_tree = &self.root;

        while let Some(current_node) = current_tree {
            match current_node.value.cmp(value) {
                Ordering::Less => {
                    current_tree = &current_node.right;
                }
                Ordering::Equal => {
                    return true;
                }
                Ordering::Greater => {
                    current_tree = &current_node.left;
                }
            };
        }

        false
    }

    pub fn get(&self, value: &T) -> Option<&T> {
        let mut current_tree = &self.root;

        while let Some(current_node) = current_tree {
            match current_node.value.cmp(value) {
                Ordering::Less => {
                    current_tree = &current_node.right;
                }
                Ordering::Equal => {
                    return Some(&current_node.value);
                }
                Ordering::Greater => {
                    current_tree = &current_node.left;
                }
            };
        }

        None
    }

    pub fn append(&mut self, other: &mut Self) {
        if other.is_empty() {
            return;
        }

        let mut remaining_nodes = Vec::<AvlNode<T>>::default();

        remaining_nodes.push(*other.root.take().unwrap());

        while let Some(mut this_node) = remaining_nodes.pop() {
            loop {
                self.insert(this_node.value);

                if let Some(right_node) = this_node.right.take() {
                    remaining_nodes.push(*right_node);
                }

                if let Some(next_node) = this_node.left.take() {
                    this_node = *next_node;
                } else {
                    break;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.root.take();
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    #[deny(clippy::all)]
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.node_iter().map(|node| &node.value)
    }

    fn node_iter(&'a self) -> impl Iterator<Item = &'a AvlNode<T>> + 'a {
        AvlTreeSetNodeIter {
            prev_nodes: Vec::new(),
            current_tree: &self.root,
        }
    }

    pub fn union(&'a self, other: &'a Self) -> impl Iterator<Item = &'a T> + 'a {
        AvlTreeSetUnionIter {
            left_iter: self.iter().peekable(),
            right_iter: other.iter().peekable(),
        }
    }

    pub fn difference(&'a self, other: &'a Self) -> impl Iterator<Item = &'a T> + 'a {
        self.iter().filter(move |&value| !other.contains(value))
    }

    pub fn symmetric_difference(&'a self, other: &'a Self) -> impl Iterator<Item = &'a T> + 'a {
        AvlTreeSetUnionIter {
            left_iter: self.difference(other).peekable(),
            right_iter: other.difference(self).peekable(),
        }
    }
}

impl<T: Ord> FromIterator<T> for AvlTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();

        for i in iter {
            set.insert(i);
        }

        set
    }
}

#[derive(Debug)]
pub struct AvlTreeSetNodeIter<'a, T: Ord> {
    prev_nodes: Vec<&'a AvlNode<T>>,
    current_tree: &'a AvlTree<T>,
}

impl<'a, T: 'a + Ord> Iterator for AvlTreeSetNodeIter<'a, T> {
    type Item = &'a AvlNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => {
                        return None;
                    }

                    Some(prev_node) => {
                        self.current_tree = &prev_node.right;

                        return Some(prev_node);
                    }
                },

                Some(ref current_node) => {
                    if current_node.left.is_some() {
                        self.prev_nodes.push(current_node);
                        self.current_tree = &current_node.left;

                        continue;
                    }

                    if current_node.right.is_some() {
                        self.current_tree = &current_node.right;

                        return Some(current_node);
                    }

                    self.current_tree = &None;

                    return Some(current_node);
                }
            }
        }
    }
}

pub struct AvlTreeSetUnionIter<'a, T: 'a + Ord, I: Iterator<Item = &'a T>> {
    left_iter: Peekable<I>,
    right_iter: Peekable<I>,
}

impl<'a, T: 'a + Ord, I: Iterator<Item = &'a T>> Iterator for AvlTreeSetUnionIter<'a, T, I> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&left_node) = self.left_iter.peek() {
            if let Some(&right_node) = self.right_iter.peek() {
                match left_node.cmp(&right_node) {
                    Ordering::Less => self.left_iter.next(),
                    Ordering::Equal => {
                        self.right_iter.next();
                        self.left_iter.next()
                    }
                    Ordering::Greater => self.right_iter.next(),
                }
            } else {
                self.left_iter.next()
            }
        } else if self.right_iter.peek().is_some() {
            self.right_iter.next()
        } else {
            None
        }
    }
}

use std::cmp::max;
use std::mem::swap;

#[derive(Debug, PartialEq, Clone)]
pub struct AvlNode<T: Ord> {
    pub value: T,
    pub left: AvlTree<T>,
    pub right: AvlTree<T>,
    pub height: usize,
}

pub type AvlTree<T> = Option<Box<AvlNode<T>>>;

impl<'a, T: 'a + Ord> AvlNode<T> {
    pub fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height)
    }

    pub fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.height)
    }

    pub fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height());
    }

    pub fn balance_factor(&self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    pub fn rotate_left(&mut self) -> bool {
        if self.right.is_none() {
            return false;
        }

        let right_node = self.right.as_mut().unwrap();
        let right_left_tree = right_node.left.take();
        let right_right_tree = right_node.right.take();

        let mut new_left_tree = replace(&mut self.right, right_right_tree);
        swap(&mut self.value, &mut new_left_tree.as_mut().unwrap().value);
        let left_tree = self.left.take();

        let new_left_node = new_left_tree.as_mut().unwrap();
        new_left_node.right = right_left_tree;
        new_left_node.left = left_tree;
        self.left = new_left_tree;

        if let Some(node) = self.left.as_mut() {
            node.update_height();
        }

        self.update_height();

        true
    }

    pub fn rotate_right(&mut self) -> bool {
        if self.left.is_none() {
            return false;
        }

        let left_node = self.left.as_mut().unwrap();
        let left_right_tree = left_node.right.take();
        let left_left_tree = left_node.left.take();

        let mut new_right_tree = replace(&mut self.left, left_left_tree);
        swap(&mut self.value, &mut new_right_tree.as_mut().unwrap().value);
        let right_tree = self.right.take();

        let new_right_node = new_right_tree.as_mut().unwrap();
        new_right_node.left = left_right_tree;
        new_right_node.right = right_tree;
        self.right = new_right_tree;

        if let Some(node) = self.right.as_mut() {
            node.update_height();
        }

        self.update_height();

        true
    }

    pub fn rebalance(&mut self) -> bool {
        match self.balance_factor() {
            -2 => {
                let right_node = self.right.as_mut().unwrap();

                if right_node.balance_factor() == 1 {
                    right_node.rotate_right();
                }

                self.rotate_left();

                true
            }

            2 => {
                let left_node = self.left.as_mut().unwrap();

                if left_node.balance_factor() == -1 {
                    left_node.rotate_left();
                }

                self.rotate_right();

                true
            }
            _ => false,
        }
    }
}

extern crate rand; // 0.8.5
use rand::Rng;
use std::time::Instant;

fn main() {
    let n = 10_000; // Adjust N based on performance requirements
    let mut rng = rand::thread_rng();

    // Random Order Tree
    let mut random_tree = AvlTreeSet::new();
    let start_random_insert = Instant::now();
    for _ in 0..n {
        let value = rng.gen_range(0..n);
        random_tree.insert(value);
    }
    let random_insert_duration = start_random_insert.elapsed();

    // Ascending Order Tree
    let mut ascending_tree = AvlTreeSet::new();
    let start_ascending_insert = Instant::now();
    for value in 0..n {
        ascending_tree.insert(value);
    }
    let ascending_insert_duration = start_ascending_insert.elapsed();

    // Search Operation
    let start_random_search = Instant::now();
    for _ in 0..n / 10 {
        let value = rng.gen_range(0..n);
        random_tree.search(&value);
    }
    let random_search_duration = start_random_search.elapsed();

    let start_ascending_search = Instant::now();
    for _ in 0..n / 10 {
        let value = rng.gen_range(0..n);
        ascending_tree.search(&value);
    }
    let ascending_search_duration = start_ascending_search.elapsed();

    // Delete Operation
    let start_random_delete = Instant::now();
    for _ in 0..n / 10 {
        let value = rng.gen_range(0..n);
        random_tree.remove(&value);
    }
    let random_delete_duration = start_random_delete.elapsed();

    let start_ascending_delete = Instant::now();
    for _ in 0..n / 10 {
        let value = rng.gen_range(0..n);
        ascending_tree.remove(&value);
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
