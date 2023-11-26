#[derive(Clone, PartialEq)]
pub enum Coin {
    Empty,
    Red,
    Yellow,
}

impl Coin {
    pub fn equals_or_empty(&self, other: &Self) -> bool {
        if *self == Coin::Empty || *other == Coin::Empty {
            return true;
        }

        self == other
    }
}
