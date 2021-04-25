/// A CardStage is a helper to use different stages for a card's actions
#[derive(Debug, Clone, Copy)]
pub enum CardStage {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    // add more if needed
}

impl From<u64> for CardStage {
    fn from(other: u64) -> CardStage {
        use CardStage::*;

        match other {
            x if x == A as u64 => A,
            x if x == B as u64 => B,
            x if x == C as u64 => C,
            x if x == D as u64 => D,
            x if x == E as u64 => E,
            x if x == F as u64 => F,
            // add more if needed

            _ => unimplemented!("Invalid enum value"),
        }
    }
}