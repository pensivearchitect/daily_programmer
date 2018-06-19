#![feature(nll)]
extern crate lazy_static;
extern crate dice_roller;

use dice_roller::rng::Rng;

fn main() {
    unsafe {
        Rng::feature_check();
    }
    println!("{}", Rng.next());
    println!("{}", Rng.next());
}
