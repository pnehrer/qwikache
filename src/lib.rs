//! A simple key/value caching library.
//!
//! The main type provided by this library is [Cache], which supports
//! the ability to store and retrieve arbitrary key/value pairs. Optionally,
//! cache entries may be set to expire at a certain time in the future.
//!
//! The implementation offers fast and stable lookup latency, as the cache is
//! backed by the standard [std::collections::HashMap] implementation. Other than comparing
//! the item's expiration time to the current system time, no additional computation
//! is performed during retrieval.
//!
//! In order to limit memory usage to a minimum when items with expiration are cached,
//! the implementation removes expired items whenever new items are inserted
//! (or existing items are replaced). Expiring items are tracked using
//! a [std::collections::BTreeSet]; when replacing existing items with expiration times,
//! old entries are first removed from the set. New entries are then inserted according
//! to their expiration time (if any). Finally, items that expired before the current system
//! time are removed from the set as well as the backing hash map.
//!
//! To facilitate its use in multi-threaded environments, [SyncCache] wraps an instance of
//! [Cache] and provides synchronized concurrent access through a standard [std::sync::RwLock].
//! As a result, multiple threads can concurrently retrieve cached items, while threads
//! trying to insert, update, or delete cached items must wait for exclusive access.

pub mod cache;
pub mod sync;

pub use cache::Cache;
pub use sync::SyncCache;
