//! Simple key/value cache implementation.

#[cfg(test)]
use mock_instant::Instant;

use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;
#[cfg(not(test))]
use std::time::Instant;

/// Simple key/value cache that supports optional item expiration.
///
/// *Storage*
/// Expiring items are tracked upon insertion; whenever an item
/// is stored, any expired items are removed from the cache.
///
/// Thus, the cost of cleaning up expired items is incurred during insertion.
/// The memory required to track expiring items is proportional to the number
/// of items in cache.
///
/// *Retrieval*
/// When an item with expiration is retrieved, its expiration time is checked
/// against the current time. The cached value is only returned if it hasn't
/// expired yet. However, no other maintenance is performed.
///
/// Thus, item retrieval should be constant for a given cache size.
#[derive(Debug, Default)]
pub struct Cache<K, V> {
    map: HashMap<K, CachedValue<V>>,
    expirations: BTreeSet<Expiration<K>>,
}

#[derive(Debug)]
struct CachedValue<V> {
    value: V,
    expires: Option<Instant>,
}

impl<K: Clone + Eq + Hash + Ord, V> Cache<K, V> {
    /// Stores a value for the given key, potentially replacing a previously cached value.
    /// The entry never expires.
    pub fn put(&mut self, key: K, value: V) {
        self.put_exp(key, value, None);
    }

    /// Stores a value for the given key, with an optional expiration time.
    pub fn put_exp(&mut self, key: K, value: V, expires: Option<Instant>) {
        if let Some(old_cached) = self.map.insert(key.clone(), CachedValue { value, expires }) {
            if let Some(expires) = old_cached.expires {
                self.expirations.remove(&Expiration {
                    key: key.clone(),
                    expires,
                });
            }
        }

        if let Some(expires) = expires {
            self.expirations.insert(Expiration { key, expires });
        }

        let now = Instant::now();
        let expired: Vec<_> = self
            .expirations
            .iter()
            .take_while(|&item| item.expires <= now)
            .cloned()
            .collect();

        for item in expired {
            self.map.remove(&item.key);
            self.expirations.remove(&item);
        }
    }

    /// Returns the cached value for the given key, if present and not expired.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key).and_then(|cached| {
            if let Some(expires) = cached.expires {
                let now = Instant::now();
                if expires <= now {
                    return None;
                }
            }

            Some(&cached.value)
        })
    }

    /// Deletes any cached value for the given key.
    pub fn delete(&mut self, key: &K) {
        if let Some(old_cached) = self.map.remove(key) {
            if let Some(expires) = old_cached.expires {
                self.expirations.remove(&Expiration {
                    key: key.clone(),
                    expires,
                });
            }
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Expiration<K> {
    expires: Instant,
    key: K,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_instant::{Instant, MockClock};
    use std::time::Duration;

    #[test]
    fn put_with_no_expiration() {
        let mut cache = Cache::default();
        cache.put("test_key".to_string(), "test_value");
        assert_eq!(cache.map.len(), 1);
        assert!(cache.map.contains_key("test_key"));
        assert_eq!(cache.expirations.len(), 0);
    }

    #[test]
    fn put_with_expiration() {
        let mut cache = Cache::default();
        cache.put_exp(
            "test_key".to_string(),
            "test_value",
            Some(Instant::now() + Duration::from_secs(1)),
        );

        assert_eq!(cache.map.len(), 1);
        assert!(cache.map.contains_key("test_key"));
        assert_eq!(cache.expirations.len(), 1);

        MockClock::advance(Duration::from_secs(2));
        cache.put("another_key".to_string(), "another_value");

        assert_eq!(cache.map.len(), 1);
        assert!(!cache.map.contains_key("test_key"));
        assert_eq!(cache.expirations.len(), 0);
    }

    #[test]
    fn get_unexpired() {
        let mut cache = Cache::default();
        cache.put_exp(
            "test_key".to_string(),
            "test_value",
            Some(Instant::now() + Duration::from_secs(1)),
        );

        assert_eq!(cache.get(&"test_key".to_string()), Some(&"test_value"));
    }

    #[test]
    fn get_expired() {
        let mut cache = Cache::default();
        cache.put_exp(
            "test_key".to_string(),
            "test_value",
            Some(Instant::now() + Duration::from_secs(1)),
        );

        MockClock::advance(Duration::from_secs(2));

        assert_eq!(cache.get(&"test_key".to_string()), None);
    }

    #[test]
    fn delete_unexpired() {
        let mut cache = Cache::default();
        cache.put_exp(
            "test_key".to_string(),
            "test_value",
            Some(Instant::now() + Duration::from_secs(1)),
        );

        cache.delete(&"test_key".to_string());
        assert_eq!(cache.map.len(), 0);
        assert_eq!(cache.expirations.len(), 0);
    }
}
