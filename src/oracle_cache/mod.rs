//! Oracle Cache Examples.
//! 
//! Oracle cache hit on every access with exact 0 latency.

use crate::cache_behav::{general_cache_behavior::*};

/// Oracle Cache struct.
#[derive(Debug)]
pub struct OracleCache {
    cachetype:String
}

impl OracleCache {
    pub fn new() -> Self {
        OracleCache{ cachetype:String::from("oracle") }
    }
}

impl GeneralCacheBehavior for OracleCache {
    fn init(&self, filename:&str) -> Result<(), String> {
        use std::fs::*;

        let content = read_to_string(filename);
        match content {
            Err(_) => Err(format!("failed to read {}", filename)),
            Ok(_) => {
                // verifing type is oracle.
                let content = content.unwrap();
                match content.find("type=oracle") {
                    None => Err(format!("type mismatched: except oracle")),
                    Some(_) => Ok(()),
                }
            },
        }
    }
    fn get_type(&self) -> &str {
        &self.cachetype
    }
    fn access(&self, _addr:u32) -> AccessResult {
        AccessResult(HitOrMiss::Hit, 0.0)
    }
}