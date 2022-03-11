//! Provides a wrapper that supports synchronized access to [Cache]
//! from multiple concurrent threads.

#[cfg(test)]
use mock_instant::Instant;

#[cfg(not(test))]
use std::time::Instant;
use std::{
    hash::Hash,
    sync::{Arc, RwLock},
};

use crate::Cache;

/// Synchronized, thread-safe key/value cache that supports multiple
/// concurrent readers.
#[derive(Debug, Default)]
pub struct SyncCache<K, V> {
    cache: Arc<RwLock<Cache<K, V>>>,
}

impl<K: Clone + Eq + Hash + Ord, V: Clone> SyncCache<K, V> {
    /// Stores a value for the given key, potentially replacing a previously cached value.
    /// The entry never expires.
    /// Blocks until it acquires an exclusive lock.
    pub fn put(&self, key: K, value: V) {
        self.cache
            .write()
            .expect("failed to acquire write lock")
            .put(key, value);
    }

    /// Stores a value for the given key, with an optional expiration time.
    /// Blocks until it acquires an exclusive lock.
    pub fn put_exp(&self, key: K, value: V, expires: Option<Instant>) {
        self.cache
            .write()
            .expect("failed to acquire write lock")
            .put_exp(key, value, expires);
    }

    /// Returns a clone of the cached value for the given key, if present and not expired.
    /// Blocks until it acquires a shared lock.
    pub fn get(&self, key: &K) -> Option<V> {
        self.cache
            .read()
            .expect("failed to acquire read lock")
            .get(key)
            .cloned()
    }

    /// Deletes any cached value for the given key.
    /// Blocks until it acquires an exclusive lock.
    pub fn delete(&self, key: &K) {
        self.cache
            .write()
            .expect("failed to acquire write lock")
            .delete(key);
    }
}
