use crate::cache_behav::{general_cache_behavior::*};

#[derive(Debug)]
pub struct CacheDefault {
    sets:u32,
    associativity:u32,
    bits:u32,
    cachetype:String
}

impl CacheDefault {
    pub fn new(s:u32, e:u32, b:u32) -> Self {
        CacheDefault{sets:s, associativity:e, bits:b, cachetype:"default".to_string()}
    }
}

impl GeneralCacheBehavior for CacheDefault {
    fn get_type(&self) -> &str { &self.cachetype }
}