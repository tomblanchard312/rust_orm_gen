use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Cache<K, V> {
    store: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Cache {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        let store = self.store.read().await;
        store.get(key).cloned()
    }

    pub async fn set(&self, key: K, value: V) {
        let mut store = self.store.write().await;
        store.insert(key, value);
    }
}