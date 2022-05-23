use crate::cache_behav::{general_cache_behavior::*};

#[derive(Debug)]
struct CacheLine {
    address:u32,
    vaild:bool
}

#[derive(Debug)]
struct AssociateGroup {
    lines:Vec<CacheLine>
}

#[derive(Debug)]
pub struct CacheDefault {
    sets:u32,
    associativity:u32,
    bits:u32,
    cachetype:String,
    groups:Vec<AssociateGroup>
}

impl CacheDefault {
    pub fn new(s:u32, e:u32, b:u32) -> Self {
        CacheDefault{sets:u32::pow(2, s), associativity:e, bits:u32::pow(2, b), cachetype:"default".to_string(), groups:vec![]}
    }
}

impl GeneralCacheBehavior for CacheDefault {
    fn get_type(&self) -> &str { &self.cachetype }
    fn access(&self, _addr:u32) -> AccessResult { AccessResult::Miss }
}