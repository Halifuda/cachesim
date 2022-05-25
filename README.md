# cachesim

> A highly scalable cache simulator

`cachesim` provides a highly scalable skeleton of cache simulator. One can use this tool not only to simulate a conventional cache behavior but also introduce his own type of cache and even other level of storage in the storage hierarchy, which will introduce benefits into storage and memory research.

I have made an example to use this crate as a library in a local Rust project. Please refer to [cachesim_example](https://github.com/Halifuda/cachesim_example).

----

The basic usage is like:

```rust
use cachesim::{CacheDevice, DefaultCache, general_cache_behavior::*};

fn main() {
    // To simulate a cache, firstly build a Cache Device, 
    // given a Cache struct and a config file to initialize the cache.
    let mut cache = CacheDevice::new(DefaultCache::new(), "path\\to\\config\\file");

    // The total size of the cache in bytes could be get by get_size().
    println!("cache size:{}B", cache.get_size());

    for i in 0..16 {
        // Access the cache by giving a u32 represented address to access().
        // The result will be returned in a tuble of (AccessResult, f64).
        let AccessResult(r, l) = cache.access(i);

        // If you want to check the result for a single access, use match.
        println!("{}", match r {
            HitOrMiss::Hit => "hit",
            HitOrMiss::Miss => "miss",
        });
    }

    // The total results of all the operations from construction are recorded.
    // To get these results, use get_result(). A tuple (u32, u32, f64) will be 
    // returned, representing hit count, miss count and total access latency.
    let (h, m, l) = cache.get_result();
    println!("total results: hits={}, misses={}, latency={}", h, m, l);

    // The recorded results could be cleared by clear_result().
    cache.clear_result();
}
```

----

To introduce a new type of cache, you need not to modify the source code. For example, if you want to introduce an 'Oracle Cache', firstly you could create a file 'oracle_cache.rs' or a module 'oracle_cache/mod.rs' under your own project directory. Then, define a `struct` representing the cache:

```rust
pub struct OracleCache {
    cachetype:String
    // other field may be needed.
}
```

Then, implement the `GeneralCacheBehavior` for your new cache type:

```rust
use crate::cache_behav::{general_cache_behavior::*};

impl GeneralCacheBehavior for OracleCache {
    // Methods below need to be implemented.
    fn init(&mut self, filename:&str) -> Result<(), String> { ... }
    fn get_type(&self) -> &str { ... }
    fn access(&mut self, _addr:u32) -> AccessResult { ... }
    fn clear(&mut self) { ... }
    fn size(&self) -> usize { ... }
}
```
You can implement any other method for the cache.

Then, use your own cache in your project:

```rust
use cachesim::{CacheDevice, general_cache_behavior::*};
// Import your own cache.
mod oracle_cache;
use oracle_cache::OracleCache;

fn main() {
    // Use your own cache by passing it to CacheDevice.
    let mut cache = CacheDevice::new(OracleCache::new(), "path\\to\\config\\file");
}
```

By this way you could even introduce a hierarchical storage architecture by define and implement several `struct` that are well organized.

----

To use the example cache, you need to provide a config file.

For Default Cache, your file need to include following contents with strict format, each occurs in a single line:

- `type=default$` # this must not be changed
- `sets=<u32>$` # to define the number of sets in the cache
- `associativity=<u32>$` # to define the number of ways(associativity) in one cache set
- `blocksize=<u32>$` # to define the size in byte of a cacheline(block)
- `hit latency=<f64>$`
- `miss penalty=<f64>$` # this 2 configs define the latency feature of the cache, the unit of these float numbers are not needed and must not be written here

do not add any white space in these lines, and do add '$' at the end of each line.

For Oracle Cache, your file need to include following contents in a single line:

- `type=oracle$` # this must not be changed

do not add any white space in these lines, and do add '$' at the end of each line.

See `examples/default.txt`.