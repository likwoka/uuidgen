//!
//!
//!
use uuid::Uuid;

/// Hard code to generate 20 UUIDs by default.
const NUM_TO_GENERATE: u8 = 20;
 
/// Return a list of X UUIDs where X is the count.  
fn gen_uuids(count: u8) -> Vec<Uuid> {
    let mut vals: Vec<Uuid> = Vec::new();
    for _n in 0..count {
        vals.push(Uuid::new_v4());
    }
    vals
}

fn main() {
    
    // TODO: take command line argument to see how many to generate
    // TODO2: make interactive and ask for number
    // TODO3: add instrumentation/timer

    let uuids = gen_uuids(NUM_TO_GENERATE);
    for x in &uuids {
        println!("{}", x);
    }
}
