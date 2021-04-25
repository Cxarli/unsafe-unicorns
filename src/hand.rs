use crate::{Card, CardList};

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Box<dyn Card>>,
    processing: Option<u64>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: vec! {},
            processing: None,
        }
    }
}

impl CardList for Hand {
    fn __list(&self) -> &Vec<Box<dyn Card>> { &self.cards }
    fn __list_mut(&mut self) -> &mut Vec<Box<dyn Card>> { &mut self.cards }
    fn __processing(&self) -> Option<u64> { self.processing }
    fn __processing_mut(&mut self, new: Option<u64>) -> Option<u64> { ::std::mem::replace(&mut self.processing, new) }
}