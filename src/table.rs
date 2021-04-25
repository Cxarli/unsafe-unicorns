use crate::{Discard, Drawpile, Nursery};

#[derive(Debug)]
pub struct Table {
    pub discard: Discard,
    pub drawpile: Drawpile,
    pub nursery: Nursery,
}

impl Table {
    pub fn new() -> Table {
        Table {
            discard: Discard::new(),
            drawpile: Drawpile::new(),
            nursery: Nursery::new(),
        }
    }
}