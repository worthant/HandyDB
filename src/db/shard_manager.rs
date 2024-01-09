use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::Shard;

pub struct ShardManager {
    shards: Vec<Shard>,
    num_shards: usize,
}

impl ShardManager {
    pub fn new(num_shards: usize) -> Self {
        let mut shards = Vec::with_capacity(num_shards);
        for _ in 0..num_shards {
            shards.push(Shard::new());
        }

        ShardManager {
            shards,
            num_shards,
        }
    }

    pub fn get_shard(&self, key: &str) -> &Shard {
        let shard_index = self.calculate_shard_index(key);
        &self.shards[shard_index]
    }

    fn calculate_shard_index(&self, key: &str) -> usize {
        let hash = hash_string(key);
        hash % self.num_shards
    }
}

fn hash_string(s: &str) -> usize {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish() as usize
}
