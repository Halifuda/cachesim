use cachesim::{CacheDevice, DefaultCache, general_cache_behavior::*, general_cache_behavior::HitOrMiss::*};

fn main() {
    // To simulate a cache, firstly build a Cache Device, 
    // given a Cache struct and a config file to initialize the cache.
    let mut cache = CacheDevice::new(DefaultCache::new(), "F:\\Programs\\Programs-Rust\\cachesim_example\\default.txt");

    // The total size of the cache in bytes could be get by get_size().
    println!("cache size:{}B", cache.get_size());

    println!("access first 16 addresses.");
    for i in 0..16 {
        // Access the cache by giving a u32 represented address to access().
        // The result will be returned in a tuble of (AccessResult, f64).
        let AccessResult(r, l) = cache.access(i);

        println!("accessing {:06X}, {}, latency is {}", i, 
        // If you want to check the result for a single access, use match.
        match r {
            Hit => "hit",
            Miss => "miss",
        }, l); 
    }    
    println!("access first 16 addresses again.");
    for i in 0..16 {
        let AccessResult(r, l) = cache.access(i);
        println!("accessing {:06X}, {}, latency is {}", i, 
        match r {
            Hit => "hit",
            Miss => "miss",
        }, l); 
    }    
    println!("access first 16 addresses, clear after each access.");
    for i in 0..16 {
        let AccessResult(r, l) = cache.access(i);
        println!("accessing {:06X}, {}, latency is {}", i, 
        match r {
            Hit => "hit",
            Miss => "miss",
        }, l);
        // Cache could be clear up by clear().
        cache.clear();
    }
    
    // The total results of all the operations from construction are recorded.
    // To get these results, use get_result(). A tuple (u32, u32, f64) will be 
    // returned, representing hit count, miss count and total access latency.
    let (h, m, l) = cache.get_result();
    println!("total results: hits={}, misses={}, latency={}", h, m, l);
    
    // The recorded results could be cleared by clear_result().
    println!("clear records.");
    cache.clear_result();
    let (h, m, l) = cache.get_result();
    println!("cleared results: hits={}, misses={}, latency={}", h, m, l);
}
