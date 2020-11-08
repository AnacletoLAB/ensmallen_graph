
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;


fn encode(value: f64) -> u64 {
    unsafe{
        u64::from_le_bytes(
            value.to_le_bytes()
        )
    }
}

fn decode(value: u64) -> f64 {
    unsafe{
        f64::from_le_bytes(
            value.to_le_bytes()
        )
    }
}

struct ConcurrentHashmap {
    insert_lock: Mutex<()>,
    map: HashMap<(NodeT, NodeT), AtomicU64>
}

impl ConcurrentHashmap {
    pub fn add(&mut self, key: (NodeT, NodeT), value: f64){
        while !self.map.contains(key) {
            let lock = self.insert_lock.lock();
            self.map.insert(AtomicU64::new(encode(0.0)))
        }

        let value = self.map.get(key);

        let mut stored = value.load(
            Ordering::Relaxed,
            Ordering::Relaxed,
        });
        loop {
            let inner = x.compare_and_swap(
                stored, 
                encode(decode(stored) + value), 
                ord
            );
            if inner == stored {
                break;
            }
            stored = inner;
        }
        
    }
}