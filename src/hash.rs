use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

const INITIAL_CAPACITY: usize = 8;
const LOAD_FACTOR: f64 = 0.75;

#[derive(Debug, Clone, PartialEq)]
enum Entry<K, V> {
    Occupied(K, V),
    Deleted,
    Empty,
}

pub struct HashTable<K, V> {
    table: Vec<Entry<K, V>>,
    size: usize,
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        let table = vec![Entry::Empty; INITIAL_CAPACITY];
        HashTable { table, size: 0 }
    }

    fn hash<Q>(&self, key: &Q, capacity: usize) -> usize
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % capacity as u64) as usize
    }

    fn resize(&mut self) {
        let new_capacity = self.table.len() * 2;
        let mut new_table = vec![Entry::Empty; new_capacity];

        let items_to_reinsert: Vec<_> = self.table.iter().filter_map(|entry| {
            if let Entry::Occupied(ref key, ref value) = entry {
                Some((key.clone(), value.clone()))
            } else {
                None
            }
        }).collect();

        self.table = new_table;
        self.size = 0;

        for (key, value) in items_to_reinsert {
            self.insert_entry(key, value);
        }
    }


    fn insert_entry(&mut self, key: K, value: V) {
        let mut i = 0;
        let capacity = self.table.len();
        loop {
            let index = (self.hash(&key, capacity) + i * i) % capacity;
            match &self.table[index] {
                Entry::Occupied(ref k, _) if *k == key => {
                    self.table[index] = Entry::Occupied(key, value);
                    return;
                }
                Entry::Empty | Entry::Deleted => {
                    self.table[index] = Entry::Occupied(key, value);
                    return;
                }
                _ => {
                    i += 1;
                }
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.size as f64 / self.table.len() as f64 >= LOAD_FACTOR {
            self.resize();
        }
        self.insert_entry(key, value);
        self.size += 1;
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut i = 0;
        let capacity = self.table.len();
        loop {
            let index = (self.hash(key, capacity) + i * i) % capacity;
            match self.table[index] {
                Entry::Occupied(ref k, _) if k.borrow() == key => {
                    if let Entry::Occupied(_, ref value) = self.table[index] {
                        let value = value.clone();
                        self.table[index] = Entry::Deleted;
                        self.size -= 1;
                        return Some(value);
                    }
                }
                Entry::Empty => return None,
                _ => {
                    i += 1;
                }
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut i = 0;
        let capacity = self.table.len();
        loop {
            let index = (self.hash(key, capacity) + i * i) % capacity;
            match self.table[index] {
                Entry::Occupied(ref k, ref v) if k.borrow() == key => return Some(v),
                Entry::Empty => return None,
                _ => {
                    i += 1;
                }
            }
        }
    }
}

fn main() {
    let mut table = HashTable::new();
    table.insert("key1", "value1");
    table.insert("key2", "value2");
    table.insert("key3", "value3");

    println!("{:?}", table.get(&"key1")); // Some("value1")
    println!("{:?}", table.get(&"key2")); // Some("value2")
    println!("{:?}", table.get(&"key3")); // Some("value3")

    table.remove(&"key2");

    println!("{:?}", table.get(&"key2")); // None
}
