#![feature(platform_intrinsics, nll)]
pub mod rng;
pub mod dice;

pub use rng::Rng;
pub use dice::Dice;
