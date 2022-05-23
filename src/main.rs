use std::env;
mod simulator;
mod cache_def;
mod cache_behav;

use simulator::*;
use cache_def::*;

fn print_help() -> Option<(u32, u32, u32)> {
    println!("Cache Simulator.");
    println!("This is a logical cache simulator developed by rust and cargo.");
    println!("Used cache model is set-associative cache that is wildly used.");
    println!("The default number of sets is 8, number of associativity is 1, number of block size in bits is 8 (1 byte).");
    println!("Use arguments to specify your own cache parameters.\n");
    println!("usage: cachesim [args]\n");
    println!("[args]:");
    println!("    -s <u32>, -S <u32>, --set <u32>: to set the number of sets in the cache.");
    println!("                                     Notice that the real sets is the 2-power of this given parameter.");
    println!("    -e <u32>, -E <u32>, --associativity <u32>: to set the number of associativity in the cache.");
    println!("    -b <u32>, -B <u32>, --bit <u32>: to set the number of block size in bits in the cache.");
    println!("                                     Notice that the real bits is the 2-power of this given parameter.");
    println!("    -h, --help: print this manual.");
    None
}

fn resolve_arguments() -> Result<Option<(u32, u32, u32)>, String> {
    let args: Vec<String> = env::args().collect();
    let mut s = 2;
    let mut e = 1;
    let mut b = 3;
    let mut state:char = '_';

    for arguement in &args[1..] {
        let arg:&str = arguement;
        match arg {
            "-h" | "--help" => {
                return Ok(print_help())
            },
            "-s" | "-S" | "--set" => {
                if state != '_' {
                    return Err(format!("Unexpected argument: {}", arg))
                } else {
                    state = 's';
                }
            },
            "-e" | "-E" | "--associativity" => {                
                if state != '_' {
                    return Err(format!("Unexpected argument: {}", arg))
                } else {
                    state = 'e';
                }
            },
            "-b" | "-B" | "--bit" => {                
                if state != '_' {
                    return Err(format!("Unexpected argument: {}", arg))
                } else {
                    state = 'b';
                }
            },
            _ => {
                match &state {
                    '_' => {
                        return Err(format!("Unexpected argument: {}", arg))
                    }
                    's' => {
                        s = arg.parse::<u32>().expect("Failed to parse integrator.");
                        state = '_';
                    }
                    'e' => {
                        e = arg.parse::<u32>().expect("Failed to parse integrator.");
                        state = '_';
                    }
                    'b' => {
                        b = arg.parse::<u32>().expect("Failed to parse integrator.");
                        state = '_';
                    }
                    _ => {
                        return Err(format!("Unexpected error."))
                    }
                }
            },
        }
    }
    if state != '_' {
        return Err(format!("Missed arguments \"-{} <u32>\"", state))
    }
    Ok(Some((s, e, b)))
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let res = resolve_arguments();
    let (s, e, b): (u32, u32, u32) = match res {
        Err(info) => {
            println!("{}", info);
            println!("Please use \"-h\" or \"--help\"");
            return
        }
        Ok(None) => {
            return
        }
        Ok(Some((x, y, z))) => (x, y, z)

    };
    println!("{}, {}, {}", s, e, b);

    let sim:CacheSimulator<CacheDefault> = CacheSimulator::<CacheDefault>::new(CacheDefault::new(s,e,b));
    println!("{}", sim.get_cache_type());
}
