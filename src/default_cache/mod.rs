//! Default Cache Examples.
//!
//! Default cache has a similar behavior with conventional SRAM cache.
//! It is organized in set-associated architecture.
//! To simplify this example, this cache uses a fifo evict policy.

use crate::cache_behav::general_cache_behavior::*;

#[derive(Debug)]
struct CacheLineWay {
    lines: Vec<u32>,
    replaceptr: u32
}

/// Default Cache struct.
#[derive(Debug)]
pub struct DefaultCache {
    cachetype: String,
    sets: u32,
    set_bit_len: u32,
    associativity: u32,
    blocksize: u32,
    block_bit_len: u32,
    hit_latency: f64,
    miss_penalty: f64,
    ways: Vec<CacheLineWay>,
}

impl DefaultCache {
    pub fn new() -> Self {
        DefaultCache {
            cachetype: String::from("default"),
            sets: 4,
            set_bit_len: 0,
            associativity: 1,
            blocksize: 8,
            block_bit_len: 0,
            hit_latency: 0.0,
            miss_penalty: 0.0,
            ways: vec![],
        }
    }

    /// access a default cache
    fn default_cache_access(&mut self, addr:u32) -> AccessResult {
        // get the set number with address tag
        let b_n_mask = !((1 << self.block_bit_len) - 1);
        let cache_line_number = addr & b_n_mask;
        let s_mask = ((1 << self.set_bit_len) - 1) << self.block_bit_len;
        let set_number = (cache_line_number & s_mask) >> self.block_bit_len;

        // find if the address is in cache
        let way = &mut self.ways[set_number as usize];
        for line in &mut way.lines {
            if *line == cache_line_number {
                // in
                return AccessResult(HitOrMiss::Hit, self.hit_latency);
            }
        }
        // not in
        if way.lines.len() < self.associativity.try_into().unwrap() {
            // if the way is not full, add a new line
            way.lines.push(cache_line_number);
        } else {
            // if the way is full, evict a line
            way.lines[way.replaceptr as usize] = cache_line_number;
            way.replaceptr += 1;
            if way.replaceptr >= self.associativity.try_into().unwrap() {
                way.replaceptr = 0;
            }
        }
        return AccessResult(HitOrMiss::Miss, self.miss_penalty + self.hit_latency);
    }
}

/// Get the len in bits of a u32 int.
fn get_bit_lens(size: u32) -> u32 {
    let mut len: u32 = 0;
    let mut temp = size;
    while temp > 1 {
        temp /= 2;
        len += 1;
    }
    len
}

/// Resolving config file
fn resolve_default_cache_config(content: String) -> Result<(u32, u32, u32, f64, f64), String> {
    println!("Resolving default cache config...");

    let s_start = content.find("sets=").unwrap();
    let s_end = content[s_start..].find("$").unwrap();
    let s = content[s_start+5..s_start+s_end].parse::<u32>().unwrap();
    println!("Parsed config: sets={}", s);
    
    let a_start = content.find("associativity=").unwrap();
    let a_end = content[a_start..].find("$").unwrap();
    let a = content[a_start+14..a_start+a_end].parse::<u32>().unwrap();
    println!("Parsed config: associativity={}", a);

    let b_start = content.find("blocksize=").unwrap();
    let b_end = content[b_start..].find("$").unwrap();
    let b = content[b_start+10..b_start+b_end].parse::<u32>().unwrap();
    println!("Parsed config: blocksize={}", b);

    let h_start = content.find("hit latency=").unwrap();
    let h_end = content[h_start..].find("$").unwrap();
    let h = content[h_start+12..h_start+h_end].parse::<f64>().unwrap();
    println!("Parsed config: hit latency={}", h);
    
    let m_start = content.find("miss penalty=").unwrap();
    let m_end = content[m_start..].find("$").unwrap();
    let m = content[m_start+13..m_start+m_end].parse::<f64>().unwrap();
    println!("Parsed config: miss penalty={}", m);

    Ok((s, a, b, h, m))
}

impl GeneralCacheBehavior for DefaultCache {
    fn init(&mut self, filename: &str) -> Result<(), String> {
        println!("Initializing default cache...");
        use std::fs::*;

        let content = read_to_string(filename);
        match content {
            Err(_) => Err(format!("failed to read {}", filename)),
            Ok(_) => {
                // verifing type is default.
                let content = content.unwrap();
                match content.find("type=default") {
                    None => Err(format!("type mismatched: except default")),
                    Some(_) => match resolve_default_cache_config(content) {
                        Err(s) => Err(s),
                        Ok((s, a, b, h, m)) => {
                            // set config
                            self.sets = s;
                            self.set_bit_len = get_bit_lens(s);
                            self.associativity = a;
                            self.blocksize = b;
                            self.block_bit_len = get_bit_lens(b);
                            self.hit_latency = h;
                            self.miss_penalty = m;

                            // add ways to the cache
                            for _i in 0..self.sets {
                                self.ways.push(CacheLineWay{ lines:vec![], replaceptr:0});
                            }
                            Ok(())
                        }
                    },
                }
            }
        }
    }

    fn get_type(&self) -> &str {
        &self.cachetype
    }

    fn access(&mut self, addr: u32) -> AccessResult {
        self.default_cache_access(addr)
    }

    fn clear(&mut self) {
        for way in &mut self.ways {
            way.lines.clear();
        }
    }

    fn size(&self) -> usize {
        (self.sets * self.associativity * self.blocksize) as usize
    }
}
