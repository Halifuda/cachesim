//! Behavior functions that all types of cache have to implement.

pub mod general_cache_behavior {

    /// Result representing cache hit or miss.
    pub enum HitOrMiss {
        Hit,
        Miss
    }

    /// Result of a single access to the cache, 
    /// including hit-miss state and access latency.
    pub struct AccessResult (pub HitOrMiss, pub f64);

    /// General cache behavior, all types have to implement.
    pub trait GeneralCacheBehavior {
        /// Initialize cache status.
        fn init(&self, filename:&str) -> Result<(), String>;

        /// Get cache type in ref str.
        fn get_type(&self) -> &str;

        /// Method simulating single access to the cache.
        fn access(&self, addr:u32) -> AccessResult;
    }
}
