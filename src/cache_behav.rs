pub mod general_cache_behavior {
    pub trait GeneralCacheBehavior {
        fn get_type(&self) -> &str;
    }
}
