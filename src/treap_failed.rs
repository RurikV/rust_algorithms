use std::collections::{HashMap, HashSet};

extern crate rand;
use self::rand::Rng;

#[derive(Default, Clone, Debug)]
pub struct Treap<T>
where
    T: Clone + Default,
{
    pub keys: Vec<f32>,
    pub priorities: Vec<f32>,
    pub vals: Vec<T>,
    pub link_child: Vec<(Option<usize>, Option<usize>)>,
    pub link_parent: Vec<usize>,
    pub freelist: Vec<usize>,
    pub instances: HashMap<usize, Option<usize>>,
}

pub enum SearchResult<T>
where
    T: Clone + Default,
{
    Exact((usize, f32, T)),
    Nearest((usize, f32, T)),
    Empty,
}

pub enum ChildBranch {
    Left,
    Right,
    NotApplicable,
}

impl<T> Treap<T>
where
    T: Clone + Default,
{
    ///helper function
    fn new_slot(&mut self) -> usize {
        let idx = match self.freelist.pop() {
            Some(x) => x,
            _ => {
                let l = self.keys.len();

                self.keys.push(Default::default());
                self.priorities.push(Default::default());
                self.vals.push(Default::default());
                self.link_child.push(Default::default());
                self.link_parent.push(Default::default());

                l
            }
        };

        idx
    }

    ///helper function
    pub fn key(&self, a: usize) -> f32 {
        assert!(a < self.keys.len());
        self.keys[a]
    }

    ///helper function
    pub fn val(&self, a: usize) -> T {
        assert!(a < self.vals.len());
        self.vals[a].clone()
    }
    pub fn prio(&self, a: usize) -> f32 {
        assert!(a < self.priorities.len());
        self.priorities[a]
    }

    ///helper function
    fn gen_priority_random() -> f32 {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(-1e-12, 1e12);
        r
    }

    pub fn init() -> Self {
        Treap::default()
    }

    pub fn new_instance(&mut self) -> usize {
        let l = self.instances.len();
        self.instances.insert(l, None);
        l
    }

    pub fn search(&self, instance: usize, k: f32) -> SearchResult<T> {
        let r: usize = match self.instances.get(&instance) {
            Some(x) if x.is_some() => x.unwrap(),
            _ => return SearchResult::Empty,
        };
        let mut n = r;
        let ret = loop {
            assert!(n < self.keys.len());
            let node_key = self.key(n);
            if k < node_key {
                if self.link_child[n].0.is_none() {
                    break SearchResult::Nearest((n, node_key, self.val(n)));
                } else {
                    n = self.link_child[n].0.unwrap();
                }
            } else if k > node_key {
                if self.link_child[n].1.is_none() {
                    break SearchResult::Nearest((n, node_key, self.val(n)));
                } else {
                    n = self.link_child[n].1.unwrap();
                }
            } else {
                break SearchResult::Exact((n, k, self.val(n)));
            }
        };
        ret
    }

    ///return position of an item if it has the same key, priority is updated for an existing item
    pub fn insert_with_priority(
        &mut self,
        instance: usize,
        k: f32,
        p: f32,
        val: T,
    ) -> (bool, usize) {
        match self.search(instance, k) {
            SearchResult::Empty => {
                let idx = self.new_slot();
                self.keys[idx] = k;
                self.priorities[idx] = p;
                self.vals[idx] = val;
                self.link_parent[idx] = idx;
                self.instances.insert(instance, Some(idx));

                (false, idx)
            }
            SearchResult::Exact((n_pos, n_key, n_val)) => {
                //item with key already exists
                self.vals[n_pos] = val;
                self.priorities[n_pos] = p;
                self.fixup_priority(instance, n_pos);

                (true, n_pos)
            }
            SearchResult::Nearest((n_pos, n_key, n_val)) => {
                let idx = self.new_slot();
                self.keys[idx] = k;
                self.priorities[idx] = p;
                self.vals[idx] = val;

                self.link_parent[idx] = n_pos;

                match self.key(n_pos) {
                    x if k < x => {
                        self.link_child[n_pos].0 = Some(idx);
                    }
                    x => {
                        self.link_child[n_pos].1 = Some(idx);
                    }
                }

                self.fixup_priority(instance, idx);

                (false, idx)
            }
        }
    }

    pub fn insert(&mut self, instance: usize, k: f32, val: T) -> (bool, usize) {
        let priority = Self::gen_priority_random();
        self.insert_with_priority(instance, k, priority, val)
    }

    fn child_branch(&self, n: usize, parent: usize) -> ChildBranch {
        match self.link_child[parent].0 {
            Some(x) if x == n => return ChildBranch::Left,
            _ => {}
        }
        match self.link_child[parent].1 {
            Some(x) if x == n => return ChildBranch::Right,
            _ => {}
        }
        ChildBranch::NotApplicable
    }

    fn fixup_priority(&mut self, instance: usize, mut n: usize) {
        //fix priority by rotating up the tree

        let mut r: usize = self
            .instances
            .get(&instance)
            .unwrap()
            .expect("root non-existent");

        let mut par = self.link_parent[n];

        while self.prio(par) > self.prio(n) && r != n {
            match self.child_branch(n, par) {
                ChildBranch::Left => {
                    self.rot_left(instance, n, par);
                }
                ChildBranch::Right => {
                    self.rot_right(instance, n, par);
                }
                _ => {
                    panic!("child link error");
                }
            }
            par = self.link_parent[n];
            r = self
                .instances
                .get(&instance)
                .unwrap()
                .expect("root non-existent");
        }
    }

    fn link_left(&mut self, parent: usize, child: Option<usize>) {
        match child {
            Some(x) => {
                self.link_parent[x] = parent;
                self.link_child[parent].0 = Some(x);
            }
            _ => {
                self.link_child[parent].0 = None;
            }
        }
    }

    fn link_right(&mut self, parent: usize, child: Option<usize>) {
        match child {
            Some(x) => {
                self.link_parent[x] = parent;
                self.link_child[parent].1 = Some(x);
            }
            _ => {
                self.link_child[parent].1 = None;
            }
        }
    }

    fn rot_left(&mut self, instance: usize, n: usize, parent: usize) {
        // before:
        //          pp
        //          |
        //          p
        //         / \
        //        n   c
        //       / \
        //      a   b
        //
        // after:
        //          pp
        //          |
        //          n
        //         / \
        //        a   p
        //           / \
        //          b   c
        //

        let parent_is_root = match self
            .instances
            .get(&instance)
            .expect("instance non-existent")
        {
            Some(x) => {
                if *x == n {
                    return;
                } else if *x == parent {
                    true
                } else {
                    false
                }
            }
            _ => {
                panic!("root does not exist");
            }
        };

        if !parent_is_root {
            let pp = self.link_parent[parent];

            match self.child_branch(parent, pp) {
                ChildBranch::Left => {
                    self.link_left(pp, Some(n));
                }
                ChildBranch::Right => {
                    self.link_right(pp, Some(n));
                }
                _ => {
                    panic!();
                }
            }
        } else {
            //update to new root
            self.instances.insert(instance, Some(n));

            self.link_parent[n] = n;
        }

        self.link_left(parent, self.link_child[n].1);

        self.link_right(n, Some(parent));
    }

    fn rot_right(&mut self, instance: usize, n: usize, parent: usize) {
        // before:
        //          pp
        //          |
        //          p
        //         / \
        //        c   n
        //           / \
        //          a   b
        //
        // after:
        //          pp
        //          |
        //          n
        //         / \
        //        p   b
        //       / \
        //      c   a
        //

        let parent_is_root = match self
            .instances
            .get(&instance)
            .expect("instance non-existent")
        {
            Some(x) => {
                if *x == n {
                    return;
                } else if *x == parent {
                    true
                } else {
                    false
                }
            }
            _ => {
                panic!("root does not exist");
            }
        };

        if !parent_is_root {
            let pp = self.link_parent[parent];

            match self.child_branch(parent, pp) {
                ChildBranch::Left => {
                    self.link_left(pp, Some(n));
                }
                ChildBranch::Right => {
                    self.link_right(pp, Some(n));
                }
                _ => {
                    panic!();
                }
            }
        } else {
            //update to new root
            self.instances.insert(instance, Some(n));

            self.link_parent[n] = n;
        }

        self.link_right(parent, self.link_child[n].0);

        self.link_left(n, Some(parent));
    }

    pub fn successor(&self, idx: usize) -> Option<usize> {
        let mut choice1 = None;
        match self.link_child[idx].1 {
            Some(x) => {
                let mut cur = x;
                while let Some(y) = self.link_child[cur].0 {
                    cur = y;
                }
                choice1 = Some(cur);
            }
            _ => {}
        }

        if choice1.is_some() {
            choice1
        } else {
            let mut cur = idx;
            let mut p = self.link_parent[cur];

            loop {
                match self.child_branch(cur, p) {
                    ChildBranch::Left => return Some(p),
                    ChildBranch::Right => {
                        cur = p;
                        p = self.link_parent[p];
                    }
                    _ => return None,
                }
            }
        }
    }

    pub fn predecessor(&self, idx: usize) -> Option<usize> {
        let mut choice1 = None;
        match self.link_child[idx].0 {
            Some(x) => {
                let mut cur = x;
                while let Some(y) = self.link_child[cur].1 {
                    cur = y;
                }
                choice1 = Some(cur);
            }
            _ => {}
        }

        if choice1.is_some() {
            choice1
        } else {
            let mut cur = idx;
            let mut p = self.link_parent[cur];

            loop {
                match self.child_branch(cur, p) {
                    ChildBranch::Right => return Some(p),
                    ChildBranch::Left => {
                        cur = p;
                        p = self.link_parent[p];
                    }
                    _ => return None,
                }
            }
        }
    }

    /// get indices of items with key in [k_start,k_end)
    pub fn query_range(&mut self, instance: usize, k_start: f32, k_end: f32) -> Vec<usize> {
        let n_start = match self.search(instance, k_start) {
            SearchResult::Exact((idx, key, val)) => Some(idx),
            SearchResult::Nearest((idx, key, val)) => {
                if key < k_start {
                    let mut n = self.successor(idx);
                    while let Some(x) = n {
                        if self.key(x) > k_start {
                            break;
                        }
                        n = self.successor(x);
                    }
                    match n {
                        Some(x) if self.key(x) > k_start => Some(x),
                        _ => None,
                    }
                } else {
                    Some(idx)
                }
            }
            _ => None,
        };

        match n_start {
            Some(start) if self.key(start) < k_end => {
                let mut cur = start;

                let mut ret = vec![cur];

                while let Some(x) = self.successor(cur) {
                    if !(self.key(x) < k_end) {
                        break;
                    }

                    cur = x;
                    ret.push(cur);
                }

                ret
            }
            _ => vec![],
        }
    }

    pub fn remove_index(&mut self, instance: usize, idx: usize) {
        loop {
            let rot_index = match self.link_child[idx] {
                (Some(l), Some(r)) => {
                    if self.key(l) < self.key(r) {
                        l
                    } else {
                        r
                    }
                }
                (Some(l), None) => l,
                (None, Some(r)) => r,
                _ => {
                    break;
                }
            };
            match self.child_branch(rot_index, idx) {
                ChildBranch::Left => {
                    self.rot_left(instance, rot_index, idx);
                }
                ChildBranch::Right => {
                    self.rot_right(instance, rot_index, idx);
                }
                _ => {
                    panic!();
                }
            }
        }

        let p = self.link_parent[idx];
        if p != idx {
            match self.child_branch(idx, p) {
                ChildBranch::Left => {
                    self.link_child[p].0 = None;
                }
                ChildBranch::Right => {
                    self.link_child[p].1 = None;
                }
                _ => {
                    panic!();
                }
            }
        }

        self.link_child[idx].0 = None;
        self.link_child[idx].1 = None;
        self.link_parent[idx] = idx;

        match self
            .instances
            .get(&instance)
            .expect("instance non-existent")
        {
            Some(x) if *x == idx => {
                self.instances.insert(instance, None);
            }
            _ => {}
        }

        self.freelist.push(idx);
    }

    /// removes items with key in range of [k_start, k_end)
    pub fn remove_key_range(&mut self, instance: usize, k_start: f32, k_end: f32) {
        self.query_range(instance, k_start, k_end)
            .iter()
            .for_each(|x| self.remove_index(instance, *x));
    }

    /// split given treap instance into two instances: a:[ x | x.key < k ], b:[ x | x.key >= k ]
    /// returns instance handles to split treaps (a,b)
    pub fn split(&mut self, instance: usize, k: f32) -> (usize, usize) {
        fn equal_f32_sentinal(a: f32, b: f32) -> bool {
            if a - 1e-1 < b && a + 1e-1 > b {
                true
            } else {
                false
            }
        }

        let (existing, idx) = self.insert_with_priority(instance, k, -1e20, Default::default());

        assert!(equal_f32_sentinal(
            self.prio(
                self.instances
                    .get(&instance)
                    .unwrap()
                    .expect("root non-existent")
            ),
            -1e20
        ));

        assert_eq!(
            idx,
            self.instances
                .get(&instance)
                .unwrap()
                .expect("root non-existent")
        );

        let l = self.link_child[idx].0;
        match l {
            Some(x) => {
                self.link_parent[x] = x;
            }
            _ => {}
        }

        self.link_child[idx].0 = None;

        //update root
        *self.instances.get_mut(&instance).unwrap() = l;

        let r = if existing {
            Some(idx)
        } else {
            let temp = self.link_child[idx].1;
            self.link_child[idx].1 = None;
            temp
        };

        match r {
            Some(x) => {
                self.link_parent[x] = x;
            }
            _ => {}
        }

        //update root
        let new_inst = self.instances.len();
        self.instances.insert(new_inst, r);

        (instance, new_inst)
    }

    /// merges 2 trees and returns handle to a combined tree
    pub fn merge(&mut self, inst_a: usize, inst_b: usize) -> usize {
        let root_a = *self.instances.get(&inst_a).expect("instance non-existent");
        let root_b = *self.instances.get(&inst_b).expect("instance non-existent");

        match (root_a, root_b) {
            (Some(l), Some(r)) => {
                let idx = self.new_slot();

                *self.instances.get_mut(&inst_a).unwrap() = Some(idx);

                self.link_left(idx, Some(l));
                self.link_right(idx, Some(r));

                self.priorities[idx] = 1e20;

                self.remove_index(inst_a, idx);

                *self.instances.get_mut(&inst_b).unwrap() = None;

                inst_a
            }
            (Some(l), _) => inst_a,
            (_, Some(r)) => inst_b,
            _ => {
                panic!();
            }
        }
    }

    pub fn dbg_depth(&self, instance: usize) -> f32 {
        let r: usize = match self.instances.get(&instance) {
            Some(x) if x.is_some() => x.unwrap(),
            _ => return 0.,
        };

        let mut hm = HashSet::new();
        let mut q = vec![r];

        let mut leaf_depths = vec![];

        while !q.is_empty() {
            let l = q.len();
            let cur = q.pop().unwrap();
            match hm.get(&cur) {
                Some(x) => {
                    leaf_depths.push(l);
                }
                _ => {
                    hm.insert(cur);
                    q.push(cur);
                    if let Some(x) = self.link_child[cur].0 {
                        q.push(x);
                    }
                    if let Some(x) = self.link_child[cur].1 {
                        q.push(x);
                    }
                }
            }
        }

        let total = leaf_depths.iter().fold(0, |acc, val| acc + *val);
        let avg = total as f32 / leaf_depths.len() as f32;
        avg
    }
}

use std::time::Instant;

fn main() {
    let n = 10_000; // Adjust N based on performance requirements
    let mut rng = rand::thread_rng();

    // Random Order Tree
    let mut random_tree = Treap::new();
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

