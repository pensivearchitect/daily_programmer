#![feature(nll)]
extern crate lazy_static;
extern crate dice_roller;

use dice_roller::rng::Rng;

fn main() {
    println!("{}", Rng.next());
    println!("{}", Rng.next());
}
