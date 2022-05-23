use crate::cache_behav::{general_cache_behavior::*, general_cache_behavior::AccessResult::*};

#[derive(Debug)]
pub struct CacheSimulator<T:GeneralCacheBehavior> {
    device: T,
    hits: u32,
    misses: u32
}

impl<T:GeneralCacheBehavior> CacheSimulator<T> {
    pub fn new(cache:T) -> Self {
        CacheSimulator::<T>{ device:cache, hits:0, misses:0 }
    }

    pub fn get_cache_type(&self) -> &str { self.device.get_type() }

    pub fn access(&mut self, addr:u32) {
        match self.device.access(addr) {
            Hit => {
                self.hits = self.hits + 1;
            }
            Miss => {
                self.misses = self.misses + 1;
            }
        }
    }
}