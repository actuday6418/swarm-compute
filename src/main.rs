#![feature(unboxed_closures)]
mod slave;
mod swarm;
use crate::swarm::IteratorExt;
use serde_closure::Fn;

fn main() {
    if std::env::args().nth(1).is_some() {
        slave::spawn_slave();
    } else {
        let mut manager = swarm::TaskManager::discover();
        (1..100u32).do_it(
            Box::new(Fn!(|x: Vec<u8>| {
                let mut a: u32 = bincode::deserialize(&x).expect("Couldn't deserialize u32");
                a += 1;
                bincode::serialize(&a).unwrap()
            })),
            &mut manager,
        );
    }
}
