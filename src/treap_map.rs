use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::Bound;
use std::mem::swap;
use std::ops::{Deref, DerefMut, RangeBounds};

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

pub struct TreapNode<P> {
    inner: Option<TreapNodeInner<P>>,
}

impl<P> Deref for TreapNode<P> {
    type Target = Option<TreapNodeInner<P>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<P> DerefMut for TreapNode<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct TreapNodeInner<P> {
    priority: u64,
    payload: P,
    left: Box<TreapNode<P>>,
    right: Box<TreapNode<P>>,
}

impl<P: Payload> TreapNodeInner<P> {
    fn new(payload: P) -> Self {
        Self {
            priority: rand::thread_rng().gen(),
            payload,
            left: Box::new(TreapNode::NONE),
            right: Box::new(TreapNode::NONE),
        }
    }

    fn update(&mut self) {
        let left_data = self.left.as_ref().as_ref().map(|node| &node.payload);
        let right_data = self.right.as_ref().as_ref().map(|node| &node.payload);
        self.payload.update(left_data, right_data);
    }

    fn push_down(&mut self) {
        self.left.push(&self.payload, Direction::Left);
        self.right.push(&self.payload, Direction::Right);
        self.payload.reset_delta();
    }

    fn push(&mut self, parent_payload: &P, _: Direction) {
        self.payload.push(parent_payload);
    }

    fn push_delta<Delta>(&mut self, delta: Delta)
    where
        P: Pushable<Delta>,
    {
        self.payload.push(delta);
    }

    fn key(&self) -> &P::Key {
        self.payload.key()
    }

    fn into_payload(self) -> P {
        self.payload
    }
}

impl<P: Payload> TreapNode<P> {
    pub const NONE: Self = Self { inner: None };

    pub fn new(payload: P) -> Self {
        Self {
            inner: Some(TreapNodeInner::new(payload)),
        }
    }

    pub fn payload(&self) -> Option<&P> {
        self.as_ref().map(|node| &node.payload)
    }

    pub fn into_payload(self) -> Option<P> {
        self.inner.map(|node| node.into_payload())
    }

    pub fn replace<Data>(&mut self, data: Data) -> Data
    where
        P: Replaceable<Data>,
    {
        self.as_mut().unwrap().payload.replace(data)
    }

    pub fn key(&self) -> Option<&P::Key> {
        self.as_ref().map(|node| node.key())
    }

    pub fn push(&mut self, parent_payload: &P, direction: Direction) {
        if let Some(node) = self.as_mut() {
            node.push(parent_payload, direction);
        }
    }

    pub fn push_delta<Delta>(&mut self, delta: Delta)
    where
        P: Pushable<Delta>,
    {
        if let Some(node) = self.as_mut() {
            node.push_delta(delta);
        }
    }

    pub fn split(self, split_key: &P::Key, to_left: bool) -> (Self, Self) {
        match self.inner {
            Some(mut node) => {
                node.push_down();
                let ordering = node.key().cmp(split_key);
                if ordering == Ordering::Less || to_left && ordering == Ordering::Equal {
                    let (left, right) = node.right.split(split_key, to_left);
                    node.right = Box::new(left);
                    node.update();
                    (Self { inner: Some(node) }, right)
                } else {
                    let (left, right) = node.left.split(split_key, to_left);
                    node.left = Box::new(right);
                    node.update();
                    (left, Self { inner: Some(node) })
                }
            }
            None => (Self::NONE, Self::NONE),
        }
    }

    pub fn split_range(self, range: impl RangeBounds<P::Key>) -> (Self, Self, Self) {
        let (left, right) = match range.start_bound() {
            Bound::Included(left_bound) => {
                let (left, right) = self.split(left_bound, false);
                (left, right)
            }
            Bound::Excluded(left_bound) => {
                let (left, right) = self.split(left_bound, true);
                (left, right)
            }
            Bound::Unbounded => (Self::NONE, self),
        };
        let (middle, right) = match range.end_bound() {
            Bound::Included(right_bound) => {
                let (middle, right) = right.split(right_bound, true);
                (middle, right)
            }
            Bound::Excluded(right_bound) => {
                let (middle, right) = right.split(right_bound, false);
                (middle, right)
            }
            Bound::Unbounded => (right, Self::NONE),
        };
        (left, middle, right)
    }

    pub fn range<'a, 's: 'a>(&'s self, r: impl RangeBounds<&'a P::Key>) -> Iter<'s, P> {
        Iter::new(self.borrow(), r)
    }

    pub fn leftmost(&self) -> Option<&Self> {
        self.as_ref()
            .map(|node| node.left.leftmost().unwrap_or(self))
    }

    pub fn rightmost(&self) -> Option<&Self> {
        self.as_ref()
            .map(|node| node.right.rightmost().unwrap_or(self))
    }

    pub fn lower(&self, key: &P::Key) -> Option<&Self> {
        self.as_ref().and_then(|node| {
            if node.key() < key {
                node.right.lower(key).or(Some(self))
            } else {
                node.left.lower(key)
            }
        })
    }

    pub fn floor(&self, key: &P::Key) -> Option<&Self> {
        self.as_ref().and_then(|node| {
            if node.key() <= key {
                node.right.floor(key).or(Some(self))
            } else {
                node.left.floor(key)
            }
        })
    }

    pub fn higher(&self, key: &P::Key) -> Option<&Self> {
        self.as_ref().and_then(|node| {
            if node.key() > key {
                node.left.higher(key).or(Some(self))
            } else {
                node.right.higher(key)
            }
        })
    }

    pub fn ceil(&self, key: &P::Key) -> Option<&Self> {
        self.as_ref().and_then(|node| {
            if node.key() >= key {
                node.left.ceil(key).or(Some(self))
            } else {
                node.right.ceil(key)
            }
        })
    }

    pub fn priority(&self) -> u64 {
        self.as_ref().map(|node| node.priority).unwrap_or(0)
    }

    pub fn merge(left: Self, right: Self) -> Self {
        if left.is_some() && right.is_some() {
            assert!(
                left.rightmost().unwrap().key().unwrap() < right.leftmost().unwrap().key().unwrap()
            );
        }
        unsafe { Self::merge_unsafe(left, right) }
    }

    pub unsafe fn merge_unsafe(left: Self, right: Self) -> Self {
        match left.inner {
            None => right,
            Some(mut left) => TreapNode {
                inner: Some(match right.inner {
                    None => left,
                    Some(mut right) => {
                        if left.priority > right.priority {
                            left.push_down();
                            left.right = Box::new(Self::merge_unsafe(
                                *left.right,
                                Self { inner: Some(right) },
                            ));
                            left.update();
                            left
                        } else {
                            right.push_down();
                            right.left = Box::new(Self::merge_unsafe(
                                Self { inner: Some(left) },
                                *right.left,
                            ));
                            right.update();
                            right
                        }
                    }
                }),
            },
        }
    }

    pub fn binary_search<'s, F>(&'s self, mut f: F)
    where
        F: FnMut(&'s P::Key, &'s P, Option<&'s P>, Option<&'s P>) -> Option<Direction>,
    {
        if let Some(node) = self.deref() {
            let direction = f(
                node.key(),
                &node.payload,
                node.left.payload(),
                node.right.payload(),
            );
            if let Some(direction) = direction {
                match direction {
                    Direction::Left => {
                        node.left.binary_search(f);
                    }
                    Direction::Right => {
                        node.right.binary_search(f);
                    }
                }
            }
        }
    }

    pub fn find(&self, key: &P::Key) -> &Self {
        match self.deref() {
            None => self,
            Some(node) => {
                let ordering = node.key().cmp(key);
                match ordering {
                    Ordering::Less => node.right.find(key),
                    Ordering::Equal => self,
                    Ordering::Greater => node.left.find(key),
                }
            }
        }
    }
}

impl<P: Payload> From<TreapNode<P>> for Option<P> {
    fn from(data: TreapNode<P>) -> Self {
        data.inner.map(|node| node.payload)
    }
}

impl<P: Payload> Default for TreapNode<P> {
    fn default() -> Self {
        Self::NONE
    }
}

pub trait Payload {
    type Key: Ord;

    fn key(&self) -> &Self::Key;
    fn reset_delta(&mut self);
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>);
    fn push_delta(&mut self, delta: &Self, direction: Direction);
}

pub trait Pushable<Delta>: Payload {
    fn push(&mut self, delta: Delta);
}

impl<P: Payload> Pushable<&P> for P {
    fn push(&mut self, delta: &P) {
        self.push_delta(delta, Direction::Left);
    }
}

pub trait Replaceable<Delta>: Payload {
    fn replace(&mut self, delta: Delta) -> Delta;
}

pub struct SizePayload<InnerPayload: Payload> {
    pub inner: InnerPayload,
    pub size: u32,
}

impl<Data> From<Data> for SizePayload<PureDataPayload<Data>> {
    fn from(data: Data) -> Self {
        Self {
            inner: data.into(),
            size: 1,
        }
    }
}

impl<InnerPayload: Payload> Payload for SizePayload<InnerPayload> {
    type Key = InnerPayload::Key;

    fn key(&self) -> &Self::Key {
        self.inner.key()
    }

    fn reset_delta(&mut self) {
        self.inner.reset_delta();
    }

    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.inner
            .update(left.map(|node| &node.inner), right.map(|node| &node.inner));
        self.size =
            1 + left.map(|node| node.size).unwrap_or(0) + right.map(|node| node.size).unwrap_or(0);
    }

    fn push_delta(&mut self, delta: &Self, direction: Direction) {
        self.inner.push_delta(&delta.inner, direction);
    }
}

impl<Data, InnerPayload: Payload + Replaceable<Data>> Replaceable<Data>
    for SizePayload<InnerPayload>
{
    fn replace(&mut self, delta: Data) -> Data {
        self.inner.replace(delta)
    }
}

pub struct KeyPayload<Key: Ord, InnerPayload: Payload> {
    pub inner: InnerPayload,
    pub key: Key,
}

impl<Key: Ord, InnerPayload: Payload, Data: Into<InnerPayload>> From<(Key, Data)>
    for KeyPayload<Key, InnerPayload>
{
    fn from((key, data): (Key, Data)) -> Self {
        Self {
            inner: data.into(),
            key,
        }
    }
}

impl<Key: Ord, InnerPayload: Payload> Payload for KeyPayload<Key, InnerPayload> {
    type Key = Key;

    fn key(&self) -> &Self::Key {
        &self.key
    }

    fn reset_delta(&mut self) {
        self.inner.reset_delta();
    }

    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.inner
            .update(left.map(|node| &node.inner), right.map(|node| &node.inner));
    }

    fn push_delta(&mut self, delta: &Self, direction: Direction) {
        self.inner.push_delta(&delta.inner, direction);
    }
}

impl<Data, Key: Ord, InnerPayload: Payload + Replaceable<Data>> Replaceable<Data>
    for KeyPayload<Key, InnerPayload>
{
    fn replace(&mut self, delta: Data) -> Data {
        self.inner.replace(delta)
    }
}

pub struct PureDataPayload<Data> {
    pub data: Data,
}

impl<Data> From<Data> for PureDataPayload<Data> {
    fn from(data: Data) -> Self {
        Self { data }
    }
}

impl<Data> Payload for PureDataPayload<Data> {
    type Key = usize;

    fn key(&self) -> &Self::Key {
        unreachable!()
    }

    fn reset_delta(&mut self) {}

    fn update(&mut self, _: Option<&Self>, _: Option<&Self>) {}

    fn push_delta(&mut self, _: &Self, _: Direction) {}
}

impl<Data> Replaceable<Data> for PureDataPayload<Data> {
    fn replace(&mut self, mut data: Data) -> Data {
        swap(&mut self.data, &mut data);
        data
    }
}

pub struct ImpliedKeyPayload<InnerPayload: Payload> {
    pub inner: SizePayload<InnerPayload>,
}

impl<InnerPayload: Payload> From<InnerPayload> for ImpliedKeyPayload<InnerPayload> {
    fn from(inner: InnerPayload) -> Self {
        Self {
            inner: SizePayload { inner, size: 1 },
        }
    }
}

impl<Data, InnerPayload: Payload + Replaceable<Data>> Replaceable<Data>
    for ImpliedKeyPayload<InnerPayload>
{
    fn replace(&mut self, data: Data) -> Data {
        self.inner.inner.replace(data)
    }
}

impl<InnerPayload: Payload> Payload for ImpliedKeyPayload<InnerPayload> {
    type Key = u32;

    fn key(&self) -> &Self::Key {
        &self.inner.size
    }

    fn reset_delta(&mut self) {
        self.inner.reset_delta();
    }

    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.inner
            .update(left.map(|node| &node.inner), right.map(|node| &node.inner));
    }

    fn push_delta(&mut self, delta: &Self, direction: Direction) {
        self.inner.push_delta(&delta.inner, direction);
    }
}

// TODO: get rid of log
pub struct Iter<'a, P> {
    root: &'a TreapNode<P>,
    from: Option<&'a TreapNode<P>>,
    to: Option<&'a TreapNode<P>>,
}

impl<'a, P: Payload> Iter<'a, P> {
    pub fn new<'b>(root: &'a TreapNode<P>, r: impl RangeBounds<&'b P::Key>) -> Self
    where
        'a: 'b,
    {
        Self {
            root,
            from: match r.start_bound() {
                Bound::Included(key) => root.ceil(key),
                Bound::Excluded(key) => root.higher(key),
                Bound::Unbounded => root.leftmost(),
            },
            to: match r.end_bound() {
                Bound::Included(key) => root.floor(key),
                Bound::Excluded(key) => root.lower(key),
                Bound::Unbounded => root.rightmost(),
            },
        }
    }
}

impl<'a, P: Payload> Iterator for Iter<'a, P> {
    type Item = &'a TreapNode<P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.from?.key().unwrap() <= self.to?.key().unwrap() {
            let res = self.from;
            self.from = self.root.higher(self.from.unwrap().key().unwrap());
            res
        } else {
            None
        }
    }
}

impl<'a, P: Payload> DoubleEndedIterator for Iter<'a, P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.from?.key().unwrap() <= self.to?.key().unwrap() {
            let res = self.to;
            self.to = self.root.lower(self.to.unwrap().key().unwrap());
            res
        } else {
            None
        }
    }
}

type TreapValue<T, V> = KeyPayload<T, SizePayload<PureDataPayload<V>>>;

pub struct TreapMap<T: Ord, V> {
    root: TreapNode<TreapValue<T, V>>,
}

impl<T: Ord, V> TreapMap<T, V> {
    pub fn new() -> Self {
        Self {
            root: TreapNode::NONE,
        }
    }

    pub fn insert(&mut self, key: T, value: V) -> Option<V> {
        let mut root = TreapNode::NONE;
        swap(&mut self.root, &mut root);
        let (left, mut node, right) = root.split_range(&key..=&key);
        let res = if node.is_some() {
            Some(node.replace(value))
        } else {
            node = TreapNode::new((key, value).into());
            None
        };
        unsafe {
            self.root = TreapNode::merge_unsafe(left, TreapNode::merge_unsafe(node, right));
        }
        res
    }

    pub fn remove(&mut self, key: &T) -> Option<V> {
        let mut root = TreapNode::NONE;
        swap(&mut self.root, &mut root);
        let (left, node, right) = root.split_range(key..=key);
        unsafe {
            self.root = TreapNode::merge_unsafe(left, right);
        }
        node.into_payload().map(|data| data.inner.inner.data)
    }


    pub fn search(&self, key: &T) -> Option<&V> {
        self.root
            .find(key)
            .payload()
            .map(|data| &data.inner.inner.data)
    }

}

impl<T: Ord, V> Default for TreapMap<T, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct TreapSet<T: Ord>(TreapMap<T, ()>);

impl<T: Ord> TreapSet<T> {
    pub fn new() -> Self {
        Self(TreapMap::new())
    }

    pub fn insert(&mut self, key: T) -> bool {
        self.0.insert(key, ()).is_some()
    }

    pub fn remove(&mut self, key: &T) -> bool {
        self.0.remove(key).is_some()
    }
}

impl<T: Ord> Deref for TreapSet<T> {
    type Target = TreapMap<T, ()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Ord> DerefMut for TreapSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

extern crate rand; // 0.8.5
use rand::Rng;
use std::time::Instant;

fn main() {
    let n = 10_000; // Adjust N based on performance requirements
    let mut rng = rand::thread_rng();

    // Random Order Tree
    let mut random_tree = TreapSet::new();
    let start_random_insert = Instant::now();
    for _ in 0..n {
        let value = rng.gen_range(0..n);
        random_tree.insert(value);
    }
    let random_insert_duration = start_random_insert.elapsed();

    // Ascending Order Tree
    let mut ascending_tree = TreapSet::new();
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