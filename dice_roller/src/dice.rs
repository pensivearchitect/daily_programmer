/// A representation of a virtual dice with (hopefully) TRNG characteristics
pub struct Dice {
    sides: u8
}

impl Dice {
    pub fn new() -> Dice {
        Dice { sides: 6 }
    }

    pub fn sides(mut self, sides: u8) -> Self {
        self.sides = sides;
        self
    }

    /// Assumes a die roll can be represented as a range from 1 to `self.sides`
    pub fn roll(&self) -> u64 {
        use rng::Rng;
        Rng.range(self.sides as u64)
    }
}

#[cfg(test)]
mod tests{
#[test]
    fn six_sided() {
        use super::*;
        let die = Dice::new();
        let roll = die.roll();
        assert!(1 <= roll && roll <= 6);
    }
#[test]
    fn thirty_two_sided() {
        use super::*;
        let die = Dice::new().sides(32);
        let mut number = 32;
        while number != 0 {
            let roll = die.roll();
            assert!(1 <= roll && roll <= 32);
            number -= 1;
        }
    }
}
