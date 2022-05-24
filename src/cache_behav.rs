pub mod general_cache_behavior {
    pub enum AccessResult {
        Hit,
        Miss
    }

    pub trait GeneralCacheBehavior {
        fn init(&self, filename:&str);
        fn get_type(&self) -> &str;
        fn access(&self, addr:u32) -> AccessResult;
    }
}
