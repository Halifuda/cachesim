use crate::cache_behav::{general_cache_behavior::*};

#[derive(Debug)]
pub struct CacheSimulator<T:GeneralCacheBehavior> {
    device: T
}

impl<T:GeneralCacheBehavior> CacheSimulator<T> {
    pub fn new(cache:T) -> Self {
        CacheSimulator::<T>{device:cache}
    }

    pub fn get_cache_type(&self) -> &str { self.device.get_type() }
}