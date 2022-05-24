use crate::cache_behav::{general_cache_behavior::*, general_cache_behavior::AccessResult::*};

#[derive(Debug)]
pub struct OracleCache {
    cachetype:String
}

impl OracleCache {
    pub fn new(&self) -> Self {
        OracleCache{ cachetype:String::from("oracle") }
    }
}

impl GeneralCacheBehavior for OracleCache {
    fn init(&self, _filename:&str) {}
    fn get_type(&self) -> &str {
        &self.cachetype
    }
    fn access(&self, _addr:u32) -> AccessResult {
        Hit
    }
}