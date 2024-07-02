use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
struct HashRing {
    nodes: Arc<RwLock<BTreeMap<u64, f64>>>,
}

impl HashRing {
    fn new() -> Self {
        HashRing {
            nodes: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    fn hash(&self, value: f64) -> u64 {
        let mut s = DefaultHasher::new();
        let mut val_str = String::new();
        write!(&mut val_str, "{}", value).unwrap();
        val_str.hash(&mut s);
        s.finish()
    }

    fn add(&self, item: f64) {
        let hash = self.hash(item);
        let mut nodes = self.nodes.write().unwrap();
        nodes.insert(hash, item);
    }

    fn delete(&self, item: f64) {
        let hash = self.hash(item);
        let mut nodes = self.nodes.write().unwrap();
        nodes.remove(&hash);
    }

    fn get(&self, item: f64) -> Option<f64> {
        let hash = self.hash(item);
        let nodes = self.nodes.read().unwrap();

        if nodes.is_empty() {
            return None;
        }

        let keys: Vec<_> = nodes.keys().cloned().collect();
        let idx = match keys.binary_search(&hash) {
            Ok(i) => i,
            Err(i) => i,
        };

        let key = if idx == keys.len() {
            keys[0]
        } else {
            keys[idx]
        };
        nodes.get(&key).cloned()
    }
}

struct HashRingGrid {
    fleet: Vec<HashRing>,
}

impl HashRingGrid {
    fn new(size: usize) -> Self {
        let mut fleet = Vec::with_capacity(size);
        for _ in 0..size {
            fleet.push(HashRing::new());
        }
        HashRingGrid { fleet }
    }

    fn add_to_ring(&self, index: usize, item: f64) {
        if index < self.fleet.len() {
            self.fleet[index].add(item);
        }
    }

    fn delete_from_ring(&self, index: usize, item: f64) {
        if index < self.fleet.len() {
            self.fleet[index].delete(item);
        }
    }
}

fn main() {
    let grid = HashRingGrid::new(3);
    grid.add_to_ring(0, 1.23);
    grid.add_to_ring(1, 4.56);
    grid.add_to_ring(2, 7.89);

    println!("Added items to the hash ring grid.");

    for (i, ring) in grid.fleet.iter().enumerate() {
        match ring.get(1.23) {
            Some(value) => println!("Ring {}: Retrieved: {}", i, value),
            None => println!("Ring {}: Value not found.", i),
        }
    }

    grid.delete_from_ring(0, 1.23);
    println!("Deleted item from the hash ring grid.");

    for (i, ring) in grid.fleet.iter().enumerate() {
        match ring.get(1.23) {
            Some(value) => println!("Ring {}: Retrieved: {}", i, value),
            None => println!("Ring {}: Value not found.", i),
        }
    }
}
