// iarray.rs
pub trait IArray<T> {
    fn reset(&mut self);
    fn size(&self) -> usize;
    fn add(&mut self, item: T);
    fn get(&self, index: usize) -> T;
    fn add_at(&mut self, item: T, index: usize);
    fn remove(&mut self, index: usize) -> T;
}

// double_linked_list.rs
// use crate::iarray::IArray;

struct DRec<T> {
    item: T,
    next: Option<Box<DRec<T>>>,
    prev: Option<*mut DRec<T>>,
}

impl<T> DRec<T> {
    fn new(item: T) -> Self {
        DRec {
            item,
            next: None,
            prev: None,
        }
    }
}

pub struct DoubleLinkedList<T> {
    first: Option<Box<DRec<T>>>,
    last: *mut DRec<T>,
    size: usize,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            first: None,
            last: std::ptr::null_mut(),
            size: 0,
        }
    }
}

impl<T> IArray<T> for DoubleLinkedList<T> {
    fn reset(&mut self) {
        self.size = 0;
        self.last = std::ptr::null_mut();
        while let Some(rec) = self.first.take() {
            self.first = rec.next.take();
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn add(&mut self, item: T) {
        self.add_at(item, self.size);
    }

    fn get(&self, index: usize) -> T {
        self.find(index).item
    }

    fn add_at(&mut self, item: T, index: usize) {
        let mut new_rec = Box::new(DRec::new(item));
        if self.size == 0 {
            self.first = Some(new_rec);
            self.last = &mut *self.first.as_mut().unwrap();
        } else if index == 0 {
            new_rec.next = self.first.take();
            new_rec.next.as_mut().unwrap().prev = Some(&mut *new_rec);
            self.first = Some(new_rec);
        } else if index == self.size {
            new_rec.prev = Some(self.last);
            unsafe {
                (*self.last).next = Some(new_rec);
            }
            self.last = &mut *unsafe { (*self.last).next.as_mut().unwrap() };
        } else {
            let exist = self.find_mut(index);
            exist.prev.unwrap().as_mut().unwrap().next = Some(new_rec);
            new_rec.prev = exist.prev;
            new_rec.next = Some(exist);
            exist.prev = Some(&mut *new_rec);
        }
        self.size += 1;
    }

    fn remove(&mut self, index: usize) -> T {
        let rec = self.find_mut(index);
        if self.size == 1 {
            self.first = None;
            self.last = std::ptr::null_mut();
        } else if rec.prev.is_none() {
            // first
            self.first = rec.next.take();
            self.first.as_mut().unwrap().prev = None;
        } else if rec.next.is_none() {
            // last
            self.last = rec.prev.unwrap();
            unsafe {
                (*self.last).next = None;
            }
        } else {
            rec.prev.unwrap().as_mut().unwrap().next = rec.next.take();
            rec.next.as_mut().unwrap().prev = rec.prev;
        }
        self.size -= 1;
        rec.item
    }
}

impl<T> DoubleLinkedList<T> {
    fn find(&self, index: usize) -> &DRec<T> {
        if index > self.size / 2 {
            let mut rec = unsafe { &*self.last };
            let mut ix = self.size - 1;
            while ix != index {
                ix -= 1;
                rec = unsafe { &*rec.prev.unwrap() };
            }
            rec
        } else {
            let mut rec = self.first.as_ref().unwrap();
            let mut ix = 0;
            while ix != index {
                ix += 1;
                rec = rec.next.as_ref().unwrap();
            }
            rec
        }
    }

    fn find_mut(&mut self, index: usize) -> &mut DRec<T> {
        if index > self.size / 2 {
            let mut rec = unsafe { &mut *self.last };
            let mut ix = self.size - 1;
            while ix != index {
                ix -= 1;
                rec = unsafe { &mut *rec.prev.unwrap() };
            }
            rec
        } else {
            let mut rec = self.first.as_mut().unwrap();
            let mut ix = 0;
            while ix != index {
                ix += 1;
                rec = rec.next.as_mut().unwrap();
            }
            rec
        }
    }
}

// performance.rs
use std::time::{Duration, Instant};

fn check_valid<T>(array: &mut dyn IArray<T>) -> bool
where
    T: Default + PartialEq + std::fmt::Debug,
{
    let s0 = array.size();
    array.add(T::default());
    let s1 = array.size();
    array.add(T::default());
    let s2 = array.size();
    array.add(T::default(), 1);
    let s3 = array.size();
    if s0 == 0 && s1 == 1 && s2 == 2 && s3 == 3 {
        println!("Size - OK");
    } else {
        println!("Size - Fail");
        return false;
    }
    if array.get(0) == T::default() && array.get(1) == T::default() && array.get(2) == T::default()
    {
        println!("Add/Get - OK");
    } else {
        println!("Add/Get - Fail");
        return false;
    }
    if array.remove(1) == T::default()
        && array.get(0) == T::default()
        && array.get(1) == T::default()
        && array.size() == 2
        && array.remove(1) == T::default()
        && array.get(0) == T::default()
        && array.size() == 1
        && array.remove(0) == T::default()
        && array.size() == 0
    {
        println!("Remove - OK");
    } else {
        println!("Remove - Fail");
        return false;
    }
    array.reset();
    for i in 0..1000 {
        array.add(T::default());
    }
    for j in 0..1000 {
        if array.get(j) != T::default() {
            println!("Back insert - Fail");
            return false;
        }
    }
    println!("Back insert - OK");
    array.reset();
    for i in 0..1000 {
        array.add(T::default(), 0);
    }
    for j in 0..1000 {
        if array.get(j) != T::default() {
            println!("Front insert - Fail");
            return false;
        }
    }
    println!("Front insert - OK");
    true
}

fn check_performance<T>(array: &mut dyn IArray<T>, n: usize)
where
    T: Default + PartialEq + std::fmt::Debug,
{
    let start = Instant::now();
    array.reset();
    for i in 0..n {
        array.add(T::default(), 0);
    }
    let end = Instant::now();
    println!(
        "{} inserts into first position {}ms.",
        n,
        end.duration_since(start).as_millis()
    );
    array.reset();

    let mut ixes = Vec::with_capacity(n);
    for i in 0..n {
        ixes.push(rand::random::<usize>() % (i + 1));
    }
    let start = Instant::now();
    for &ix in &ixes {
        array.add(T::default(), ix);
    }
    let end = Instant::now();
    println!(
        "{} inserts into random position {}ms.",
        n,
        end.duration_since(start).as_millis()
    );
    array.reset();

    let start = Instant::now();
    for i in 0..n {
        array.add(T::default());
    }
    let end = Instant::now();
    println!(
        "{} inserts into last position {}ms.",
        n,
        end.duration_since(start).as_millis()
    );

    let start = Instant::now();
    for i in 0..n {
        array.get(0);
    }
    let end = Instant::now();
    println!(
        "{} reads from first position {}ms.",
        n,
        end.duration_since(start).as_millis()
    );
    for i in 0..n {
        ixes[i] = rand::random::<usize>() % n;
    }
    let start = Instant::now();
    for &ix in &ixes {
        array.get(ix);
    }
    let end = Instant::now();
    println!(
        "{} reads from random position {}ms.",
        n,
        end.duration_since(start).as_millis()
    );
    let start = Instant::now();
    for i in 0..n {
        array.get(n - 1);
    }
    let end = Instant::now();
    println!(
        "{} reads from last position {}ms.",
        n,
        end.duration_since(start).as_millis()
    );
    array.reset();
    for i in 0..n {
        array.add(T::default(), i);
    }
    let start = Instant::now();
    for i in 0..n {
        array.remove(0);
    }
    let end = Instant::now();
    println!(
        "{} removes from first position {}ms.",
        n,
        end.duration_since(start).as_millis()
    );
    array.reset();
    ixes.clear();
    ixes.resize(n, 0);
    for i in 0..n {
        array.add(T::default(), i);
        ixes[i] = rand::random::<usize>() % (n - i);
    }
    let start = Instant::now();
    for &ix in &ixes {
        array.remove(ix);
    }
    let end = Instant::now();
    println!(
        "{} removes from random position {}ms.",
        n,
        end.duration_since(start).as_millis()
    );
    array.reset();
    for i in 0..n {
        array.add(T::default(), i);
    }
    let start = Instant::now();
    for i in 0..n {
        array.remove(array.size() - 1);
    }
    let end = Instant::now();
    println!(
        "{} removes from last position {}ms.",
        n,
        end.duration_since(start).as_millis()
    );
    array.reset();
}

// single_array.rs
// use crate::iarray::IArray;

pub struct SingleArray<T> {
    array: Vec<T>,
}

impl<T> SingleArray<T> {
    pub fn new() -> Self {
        SingleArray { array: Vec::new() }
    }
}

impl<T> IArray<T> for SingleArray<T>
where
    T: Clone,
{
    fn reset(&mut self) {
        self.array.clear();
    }

    fn size(&self) -> usize {
        self.array.len()
    }

    fn add(&mut self, item: T) {
        self.array.push(item);
    }

    fn get(&self, index: usize) -> T {
        self.array[index].clone()
    }

    fn add_at(&mut self, item: T, index: usize) {
        self.array.insert(index, item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.array.remove(index)
    }
}

// vector_array.rs
// use crate::iarray::IArray;

pub struct VectorArray<T> {
    array: Vec<T>,
    vector: usize,
}

impl<T> VectorArray<T> {
    pub fn new(vector: usize) -> Self {
        VectorArray {
            array: Vec::new(),
            vector,
        }
    }
}

impl<T> IArray<T> for VectorArray<T>
where
    T: Clone,
{
    fn reset(&mut self) {
        self.array.clear();
    }

    fn size(&self) -> usize {
        self.array.len()
    }

    fn add(&mut self, item: T) {
        self.add_at(item, self.size());
    }

    fn get(&self, index: usize) -> T {
        self.array[index].clone()
    }

    fn add_at(&mut self, item: T, index: usize) {
        if self.size() == self.array.capacity() {
            let new_capacity = self.array.capacity() + self.vector;
            self.array.reserve(new_capacity - self.array.capacity());
        }
        self.array.insert(index, item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.array.remove(index)
    }
}

// factor_array.rs
// use crate::iarray::IArray;

pub struct FactorArray<T> {
    array: Vec<T>,
    factor: usize,
    init_length: usize,
}

impl<T> FactorArray<T> {
    pub fn new(factor: usize, init_length: usize) -> Self {
        FactorArray {
            array: Vec::with_capacity(init_length),
            factor,
            init_length,
        }
    }
}

impl<T> IArray<T> for FactorArray<T>
where
    T: Clone,
{
    fn reset(&mut self) {
        self.array.clear();
        self.array.reserve(self.init_length);
    }

    fn size(&self) -> usize {
        self.array.len()
    }

    fn add(&mut self, item: T) {
        self.add_at(item, self.size());
    }

    fn get(&self, index: usize) -> T {
        self.array[index].clone()
    }

    fn add_at(&mut self, item: T, index: usize) {
        if self.size() == self.array.capacity() {
            let new_capacity = self.array.capacity() + self.array.capacity() * self.factor / 100;
            self.array.reserve(new_capacity - self.array.capacity());
        }
        self.array.insert(index, item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.array.remove(index)
    }
}

// matrix_array.rs
// use crate::iarray::IArray;
// use crate::vector_array::VectorArray;

pub struct MatrixArray<T> {
    array: VectorArray<VectorArray<T>>,
    size: usize,
    vector: usize,
}

impl<T> MatrixArray<T> {
    pub fn new(vector: usize) -> Self {
        MatrixArray {
            array: VectorArray::new(10),
            size: 0,
            vector,
        }
    }
}

impl<T> IArray<T> for MatrixArray<T>
where
    T: Clone,
{
    fn reset(&mut self) {
        for i in 0..self.array.size() {
            self.array.get(i).reset();
        }
        self.array.reset();
        self.size = 0;
    }

    fn size(&self) -> usize {
        self.size
    }

    fn add(&mut self, item: T) {
        self.add_at(item, self.size());
    }

    fn get(&self, index: usize) -> T {
        self.array.get(index / self.vector).get(index % self.vector)
    }

    fn add_at(&mut self, item: T, index: usize) {
        if self.size == self.array.size() * self.vector {
            self.array.add(VectorArray::new(self.vector));
        }
        let bin_to_insert = index / self.vector;
        for i in (bin_to_insert + 1..self.array.size()).rev() {
            self.array
                .get_mut(i)
                .add_at(self.array.get(i - 1).get(self.array.get(i - 1).size() - 1), 0);
            self.array.get_mut(i - 1).remove(self.array.get(i - 1).size() - 1);
        }
        self.array.get_mut(bin_to_insert).add_at(item, index % self.vector);
        self.size += 1;
    }

    fn remove(&mut self, index: usize) -> T {
        let bin_to_remove = index / self.vector;
        let removed = self.array.get_mut(bin_to_remove).remove(index % self.vector);
        for i in bin_to_remove + 1..self.array.size() {
            self.array
                .get_mut(i - 1)
                .add(self.array.get(i).get(0));
            self.array.get_mut(i).remove(0);
        }
        if self.array.get(self.array.size() - 1).size() == 0 {
            self.array.remove(self.array.size() - 1);
        }
        self.size -= 1;
        removed
    }
}

// space_array.rs
// use crate::iarray::IArray;
// use crate::linked_list::{LinkedList, Node};
// use crate::vector_array::VectorArray;

pub struct SpaceArray<T> {
    first_block: Option<Box<Node<VectorArray<T>>>>,
    last_block: *mut Node<VectorArray<T>>,
    size: usize,
    vector: usize,
}

impl<T> SpaceArray<T> {
    pub fn new(vector: usize) -> Self {
        let mut first_block = Box::new(Node::new(VectorArray::new(vector)));
        let last_block = &mut *first_block as *mut Node<VectorArray<T>>;
        SpaceArray {
            first_block: Some(first_block),
            last_block,
            size: 0,
            vector,
        }
    }
}

impl<T> IArray<T> for SpaceArray<T>
where
    T: Clone,
{
    fn reset(&mut self) {
        while let Some(block) = self.first_block.take() {
            self.first_block = block.next.take();
        }
        let mut first_block = Box::new(Node::new(VectorArray::new(self.vector)));
        self.last_block = &mut *first_block as *mut Node<VectorArray<T>>;
        self.first_block = Some(first_block);
        self.size = 0;
    }

    fn size(&self) -> usize {
        self.size
    }

    fn add(&mut self, item: T) {
        if unsafe { (*self.last_block).item.size() == self.vector } {
            self.extend_block(self.last_block);
        }
        unsafe {
            (*self.last_block).item.add(item);
        }
        self.size += 1;
    }

    fn get(&self, index: usize) -> T {
        let result = self.find(index);
        result.block.as_ref().unwrap().item.get(result.index)
    }

    fn add_at(&mut self, item: T, index: usize) {
        let result = self.find(index);
        let ins_rec = result.block.as_ptr();
        if unsafe { (*ins_rec).item.size() == self.vector } {
            self.extend_block(ins_rec);
            let result = self.find(index);
            let ins_rec = result.block.as_ptr();
        }
        unsafe {
            (*ins_rec).item.add_at(item, result.index);
        }
        self.size += 1;
    }

    fn remove(&mut self, index: usize) -> T {
        let result = self.find(index);
        let deleted = unsafe { (*result.block.as_ptr()).item.remove(result.index) };
        if unsafe { (*result.block.as_ptr()).item.size() == 0 } {
            if result.block.as_ptr() != self.first_block.as_ref().unwrap().as_ptr() {
                let mut block = self.first_block.as_mut().unwrap();
                while block.next.as_ref().unwrap().as_ptr() != result.block.as_ptr() {
                    block = block.next.as_mut().unwrap();
                }
                block.next = unsafe { (*result.block.as_ptr()).next.take() };
                if block.next.is_none() {
                    self.last_block = block as *mut Node<VectorArray<T>>;
                }
            }
        }
        self.size -= 1;
        deleted
    }
}

impl<T> SpaceArray<T> {
    fn find(&self, index: usize) -> FindResult<T> {
        if self.size - index < unsafe { (*self.last_block).item.size() } {
            let index = index - (self.size - unsafe { (*self.last_block).item.size() });
            FindResult {
                block: Some(unsafe { &*self.last_block }),
                index,
            }
        } else {
            let mut block = self.first_block.as_ref().unwrap();
            let mut index = index;
            while block.item.size() <= index && block.next.is_some() {
                index -= block.item.size();
                block = block.next.as_ref().unwrap();
            }
            FindResult {
                block: Some(block),
                index,
            }
        }
    }

    fn extend_block(&mut self, ins_rec: *mut Node<VectorArray<T>>) {
        let mut new_rec = Box::new(Node::new(VectorArray::new(self.vector)));
        new_rec.next = unsafe { (*ins_rec).next.take() };
        unsafe {
            (*ins_rec).next = Some(new_rec);
        }
        for _ in 0..self.vector / 2 {
            let item_to_add = unsafe { (*ins_rec).item.remove((*ins_rec).item.size() - 1) };
            unsafe {
                (*(*ins_rec).next.as_ref().unwrap()).item.add_at(item_to_add, 0);
            }
        }
        if unsafe { (*self.last_block).next.is_some() } {
            self.last_block = unsafe { (*self.last_block).next.as_ref().unwrap().as_ptr() };
        }
    }
}

struct FindResult<'a, T> {
    block: Option<&'a Node<VectorArray<T>>>,
    index: usize,
}

// priority_queue.rs
// use crate::vector_array::VectorArray;
// use crate::double_linked_list::DoubleLinkedList;

struct PriorityRec<T> {
    priority: i32,
    recs: DoubleLinkedList<T>,
}

pub struct PriorityQueue<T> {
    data: VectorArray<PriorityRec<T>>,
}

impl<T> PriorityQueue<T> {
    pub fn enqueue(&mut self, priority: i32, item: T) {
        for i in 0..self.data.size() {
            let rec = self.data.get(i);
            if rec.priority == priority {
                rec.recs.add(item);
                return;
            }
            if rec.priority < priority {
                let new_rec = PriorityRec {
                    priority,
                    recs: DoubleLinkedList::new(),
                };
                new_rec.recs.add(item);
                self.data.add_at(new_rec, i);
                return;
            }
        }
        let new_rec = PriorityRec {
            priority,
            recs: DoubleLinkedList::new(),
        };
        new_rec.recs.add(item);
        self.data.add(new_rec);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.data.size() == 0 {
            return None;
        }
        let rec = self.data.get_mut(0);
        let item = rec.recs.remove(0);
        if rec.recs.size() == 0 {
            self.data.remove(0);
        }
        Some(item)
    }
}

// main.rs
// mod iarray;
// mod performance;
// mod single_array;
// mod vector_array;
// mod factor_array;
// mod matrix_array;
// mod space_array;
// mod stl_vector;
// mod stl_deque;
// mod stl_forward_list;
// mod stl_list;
// mod priority_queue;

// use single_array::SingleArray;
// use vector_array::VectorArray;
// use factor_array::FactorArray;
// use matrix_array::MatrixArray;
// use space_array::SpaceArray;
// use stl_vector::STLVector;
// use stl_deque::STLDeque;
// use stl_forward_list::STLForwardList;
// use stl_list::STLList;
// use priority_queue::PriorityQueue;

fn main() {
    const TESTS: usize = 100000;

    let mut single_array = SingleArray::new();
    performance::test(&mut single_array, TESTS);

    let mut vector_array = VectorArray::new(10);
    performance::test(&mut vector_array, TESTS);

    let mut factor_array = FactorArray::new(50, 10);
    performance::test(&mut factor_array, TESTS);

    let mut matrix_array = MatrixArray::new(10);
    performance::test(&mut matrix_array, TESTS);

    let mut space_array = SpaceArray::new(10);
    performance::test(&mut space_array, TESTS);

    let mut stl_vector = STLVector::new();
    performance::test(&mut stl_vector, TESTS);

    let mut stl_deque = STLDeque::new();
    performance::test(&mut stl_deque, TESTS);

    let mut stl_forward_list = STLForwardList::new();
    performance::test(&mut stl_forward_list, TESTS);

    let mut stl_list = STLList::new();
    performance::test(&mut stl_list, TESTS);

    let mut priority_queue = PriorityQueue::new();
    priority_queue.enqueue(0, 1000);
    priority_queue.enqueue(1, 100);
    priority_queue.enqueue(1, 101);
    assert_eq!(priority_queue.dequeue(), Some(100));
    priority_queue.enqueue(-1, -1000);
    priority_queue.enqueue(100, 1);
    assert_eq!(priority_queue.dequeue(), Some(1));
    assert_eq!(priority_queue.dequeue(), Some(101));
    assert_eq!(priority_queue.dequeue(), Some(1000));
    assert_eq!(priority_queue.dequeue(), Some(-1000));
    println!("Priority Queue - OK");
}