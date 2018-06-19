#![feature(nll)]
extern crate lazy_static;
extern crate dice_roller;

use std::io;

use dice_roller::rng::Rng;
use dice_roller::dice::Dice;

fn main() -> io::Result<()> {
    unsafe {
        Rng::feature_check();
    }
    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                run(buffer.to_string());
            },
            Err(error) => println!("{}", error),
        }
    }
    Ok(())
}

fn run(roll: String) {
    let num: Vec<&str> = roll.trim().split(char::is_alphabetic).collect();
    let (number, sides) = (num[0].parse::<u8>().unwrap(), num[1].parse::<u8>().unwrap());
    let mut count = number;
    let mut result = 0;
    while count != 0 {
        let die = Dice::new().sides(sides);
        result = result + die.roll();
        count -= 1;
    }
    println!("{}", result);
}
