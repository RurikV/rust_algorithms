use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

const INITIAL_CAPACITY: usize = 16;
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

    fn hash<Q>(&self, key: &Q) -> u64
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn find_slot<Q>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = self.hash(key);
        let mut index = hash as usize % self.table.len();
        let mut i = 0;

        loop {
            match &self.table[index] {
                Entry::Empty => return index,
                Entry::Occupied(ref k, _) if k.borrow() == key => return index,
                Entry::Deleted => return index,
                _ => {
                    i += 1;
                    index = (hash as usize + i) % self.table.len();
                }
            }
        }
    }

    fn resize(&mut self) {
        let new_capacity = self.table.len() * 2;
        let mut new_table = vec![Entry::Empty; new_capacity];

        let old_table = std::mem::replace(&mut self.table, Vec::new());
        
        for entry in old_table {
            if let Entry::Occupied(key, value) = entry {
                let mut index = self.hash(&key) as usize % new_capacity;
                let mut _i = 0;

                loop {
                    if matches!(new_table[index], Entry::Empty) {
                        new_table[index] = Entry::Occupied(key, value);
                        break;
                    }
                    _i += 1;
                    index = (index + 1) % new_capacity;
                }
            }
        }

        self.table = new_table;
    }

    pub fn insert(&mut self, key: K, value: V) {
        if (self.size + 1) as f64 / self.table.len() as f64 >= LOAD_FACTOR {
            self.resize();
        }

        let index = self.find_slot(&key);
        match self.table[index] {
            Entry::Occupied(ref mut k, ref mut v) if *k == key => *v = value,
            Entry::Occupied(_, _) | Entry::Empty | Entry::Deleted => {
                self.table[index] = Entry::Occupied(key, value);
                self.size += 1;
            }
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = self.find_slot(key);
        if let Entry::Occupied(k, _) = &self.table[index] {
            if k.borrow() == key {
                if let Entry::Occupied(_, value) = std::mem::replace(&mut self.table[index], Entry::Deleted) {
                    self.size -= 1;
                    return Some(value);
                }
            }
        }
        None
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = self.find_slot(key);
        if let Entry::Occupied(ref k, ref v) = self.table[index] {
            if k.borrow() == key {
                return Some(v);
            }
        }
        None
    }
}
fn main() {
    let mut table = HashTable::new();
    
    // Insertion
    table.insert("key1".to_string(), "value1".to_string());
    table.insert("key2".to_string(), "value2".to_string());
    table.insert("key3".to_string(), "value3".to_string());

    // Retrieval
    println!("{:?}", table.get(&"key1".to_string())); // Some("value1")
    println!("{:?}", table.get(&"key2".to_string())); // Some("value2")
    println!("{:?}", table.get(&"key3".to_string())); // Some("value3")

    // Lazy deletion
    table.remove(&"key2".to_string());
    println!("{:?}", table.get(&"key2".to_string())); // None

    // Check reuse of deleted slot
    table.insert("key2".to_string(), "value2_new".to_string());
    println!("{:?}", table.get(&"key2".to_string())); // Some("value2_new")

    // Check overflow and rehashing
    for i in 4..100 {
        table.insert(format!("key{}", i), format!("value{}", i));
    }

    for i in 1..100 {
        println!("{:?}", table.get(&format!("key{}", i))); // All keys should be found
    }

    // Check insertion with value update
    table.insert("key1".to_string(), "value1_updated".to_string());
    println!("{:?}", table.get(&"key1".to_string())); // Some("value1_updated")
}