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
        /// Initialize cache status. Receiving a config file to decide 
        /// the configure of the cache.
        /// 
        /// # Errors
        /// 
        /// If this function meets any error, the implementor 
        /// should remember to handle it and return an error in String.
        /// In example cache implementation, an error will occured when 
        /// an IO error occured, or failed to parse the config file.
        /// 
        /// # Panics
        /// 
        /// In example cache implementation, panic if failed to read any 
        /// content in passed config file.
        fn init(&mut self, filename:&str) -> Result<(), String>;

        /// Get cache type in ref str.
        fn get_type(&self) -> &str;

        /// Method simulating single access to the cache.
        fn access(&mut self, addr:u32) -> AccessResult;

        /// Clear the whold cache.
        fn clear(&mut self);

        /// Get the total size of the cache in byte.
        fn size(&self) -> usize;
    }
}
