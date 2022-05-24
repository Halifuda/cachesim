pub mod general_cache_behavior {
    pub enum HitOrMiss {
        Hit,
        Miss
    }

    pub struct AccessResult (pub HitOrMiss, pub f64);

    pub trait GeneralCacheBehavior {
        fn init(&self, filename:&str) -> Result<(), String>;
        fn get_type(&self) -> &str;
        fn access(&self, addr:u32) -> AccessResult;
    }
}
