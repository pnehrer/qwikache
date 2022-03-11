# qwikache

A simple key/value caching library.

## Overview

The main type provided by this library is *Cache*, which supports
the ability to store and retrieve arbitrary key/value pairs. Optionally,
cache entries may be set to expire at a certain time in the future.

The implementation offers fast and stable lookup latency, as the cache is
backed by the standard *HashMap* implementation. Other than comparing
the item's expiration time to the current system time, no additional computation
is performed during retrieval.

In order to limit memory usage to a minimum when items with expiration are cached,
the implementation removes expired items whenever new items are inserted
(or existing items are replaced). Expiring items are tracked using
a *BTreeSet*; when replacing existing items with expiration times,
old entries are first removed from the set. New entries are then inserted according
to their expiration time (if any). Finally, items that expired before the current system
time are removed from the set as well as the backing hash map.

To facilitate its use in multi-threaded environments, *SyncCache* wraps an instance of
*Cache* and provides synchronized concurrent access through a standard *RwLock*.
As a result, multiple threads can concurrently retrieve cached items, while threads
trying to insert, update, or delete cached items must wait for exclusive access.

## Benchmarks

A *Criterion* benchmark running on a 2.3 GHz Quad-Core Intel i7 puts the average
item lookup time at around 330 ns when used with a data set of 10M cached items
in an unsynchronized cache instance.
