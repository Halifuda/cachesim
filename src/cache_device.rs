//! Up-level Cache Device Simulator struct-s.

use crate::cache_behav::{general_cache_behavior::*, general_cache_behavior::HitOrMiss::*};

/// Device Simulator struct. Recording total access results.
#[derive(Debug)]
pub struct CacheDevice<T:GeneralCacheBehavior> {
    device: T,
    hits: u32,
    misses: u32,
    latency: f64
}

impl<T:GeneralCacheBehavior> CacheDevice<T> {
    pub fn new(cache:T, filename:&str) -> Self {
        println!("Constructing Cache Device...");
        let mut res = CacheDevice::<T>{ device:cache, hits:0, misses:0, latency:0.0 };
        res.device.init(filename).expect("Cache Init Error");
        println!("Cache Type: {}", res.device.get_type());
        println!("Cache Device constructed.");
        res
    }

    /// Get the cache type of inside cache device.
    pub fn get_type(&self) -> &str { self.device.get_type() }

    /// Method simulating a single access to the cache device.
    /// Recording hits, misses and latency.
    pub fn access(&mut self, addr:u32) -> AccessResult {
        let AccessResult(res, lat) = self.device.access(addr);
        match res {
            Hit => {
                self.hits = self.hits + 1;
                self.latency += lat;
                AccessResult(Hit, lat)
            }
            Miss => {
                self.misses = self.misses + 1;
                self.latency += lat;
                AccessResult(Miss, lat)
            }
        }
    }

    /// Get currently recorded total access results, 
    /// 
    /// in form of (hits cnt:u32, misses cnt:u32, latency:f64).
    pub fn get_result(&self) -> (u32, u32, f64) {
        (self.hits, self.misses, self.latency)
    }

    /// Get total cache capacity in byte.
    pub fn get_size(&self) -> usize {
        self.device.size()
    }

    /// Clear the whole cache.
    pub fn clear(&mut self) {
        self.device.clear();
    }

    /// Clear the recorded results.
    pub fn clear_result(&mut self) {
        self.hits = 0;
        self.misses = 0;
        self.latency = 0.0;
    }
}