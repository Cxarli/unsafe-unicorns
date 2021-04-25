use crate::{Card, CardList};

use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Drawpile {
    cards: Vec<Box<dyn Card>>,
    processing: Option<u64>,

    /// By default, the drawpile automatically shuffles whenever you add a card to it.
    /// However, you can disable that by setting this field to false.
    pub auto_shuffle: bool,
}

impl Drawpile {
    pub fn new() -> Drawpile {
        Drawpile {
            cards: vec! {},
            processing: None,

            auto_shuffle: true,
        }
    }

    /// Draw one card from the pile
    /// This returns None if there are no cards left
    pub fn draw(&mut self) -> Option<Box<dyn Card>> {
        if self.cards.len() > 0 {
            Some(self.cards.swap_remove(0))
        } else {
            None
        }
    }

    /// Shuffle all cards in the drawpile
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut ::rand::thread_rng());
    }
}

impl CardList for Drawpile {
    fn __list(&self) -> &Vec<Box<dyn Card>> { &self.cards }
    fn __list_mut(&mut self) -> &mut Vec<Box<dyn Card>> { &mut self.cards }
    fn __processing(&self) -> Option<u64> { self.processing }
    fn __processing_mut(&mut self, new: Option<u64>) -> Option<u64> { ::std::mem::replace(&mut self.processing, new) }


    fn add_card(&mut self, card: Box<dyn Card>) {
        // Copy of default implementation for this function
        self.cards.push(card);

        // Shuffle if needed
        if self.auto_shuffle {
            self.shuffle();
        }
    }

    fn add_all(&mut self, mut cards: Vec<Box<dyn Card>>) {
        // The default implementation calls add_card for every card, which means we'll shuffle
        // on every new card, which is a bit too much shuffling.
        // That's why this function is overwritten to just append and then shuffle once
        self.cards.append(&mut cards);

        if self.auto_shuffle {
            self.shuffle();
        }
    }

    /// The difference with Drawpile is that a random card is given on uuid=0
    fn take_card(&mut self, uuid: u64) -> Option<Box<dyn Card>> {
        if uuid != 0 {
            // Default implementation
            self.__list().iter().position(|c| c.uuid() == uuid).map(|index| self.__list_mut().swap_remove(index))
        } else {
            // Special case where we're asked for a random card
            self.draw()
        }
    }
}
