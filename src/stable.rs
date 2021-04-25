use crate::*;

#[derive(Debug)]
pub struct Stable {
    cards: Vec<Box<dyn Card>>,
    processing: Option<u64>,

    had: Vec<u64>,
}

impl Stable {
    pub fn new() -> Stable {
        Stable {
            cards: vec! {},
            processing: None,

            had: vec! {},
        }
    }

    /// Count the amount of unicorns in this stable
    pub fn count_unicorns(&self) -> usize {
        self.cards.iter().filter(|&card| is_unicorn(&**card)).count()

        // Ginormous Unicorns count for two
        + self.cards.iter().filter(|&card| card.id() == CardID::GinormousUnicorn).count()
    }

    /// May the player of this stable play this card in this stable?
    /// Example: Slowdown prevents owners from playing Instant cards
    pub fn owner_may_play(&self, card: &dyn Card) -> bool {
        self.cards.iter().all(|c| c.owner_may_play(card))
    }

    /// May an other player destroy the card from this stable?
    /// Example: Rainbow Aura prevents Unicorns from being destroyed
    pub fn other_may_destroy(&self, card: &dyn Card) -> bool {
        self.cards.iter().all(|c| c.other_may_destroy(card))
    }


    /// Reset the processed state of all cards
    pub fn reset_cards(&mut self) {
        self.had.clear();
    }

    /// Find a card that isn't processed yet
    /// It is the CALLER's responsibility to call `readd_card` when done processing
    /// @HACK @TODO @FIXME this is a REALLY dirty hack
    pub fn next_card(&mut self) -> Option<Box<dyn Card>> {
        // Find a card we haven't processed
        if let Some(card) = self.cards.iter().find(|c| !self.had.contains(&c.uuid())) {
            // Get UUID of the card
            let uuid = card.uuid();

            // Take card from stable for processing
            let card = self.process_card(uuid).unwrap();

            // Store that we've processed it
            self.had.push(uuid);

            // Give the card to the caller to process it
            Some(card)
        } else {
            None
        }
    }
}

impl CardList for Stable {
    fn __list(&self) -> &Vec<Box<dyn Card>> { &self.cards }
    fn __list_mut(&mut self) -> &mut Vec<Box<dyn Card>> { &mut self.cards }
    fn __processing(&self) -> Option<u64> { self.processing }
    fn __processing_mut(&mut self, new: Option<u64>) -> Option<u64> { ::std::mem::replace(&mut self.processing, new) }
}
