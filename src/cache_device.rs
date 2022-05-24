use crate::cache_behav::{general_cache_behavior::*, general_cache_behavior::HitOrMiss::*};

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
        let res = CacheDevice::<T>{ device:cache, hits:0, misses:0, latency:0.0 };
        res.device.init(filename).expect("Cache Init Error");
        println!("Cache Type: {}", res.device.get_type());
        println!("Cache Device constructed.");
        res
    }

    pub fn get_type(&self) -> &str { self.device.get_type() }

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

    pub fn get_result(&self) -> (u32, u32, f64) {
        (self.hits, self.misses, self.latency)
    }
}