use crate::{Card, CardID};

pub trait CardList {
    /// Internal functions to work around the fact that traits can't have members
    fn __list(&self) -> &Vec<Box<dyn Card>>;
    fn __list_mut(&mut self) -> &mut Vec<Box<dyn Card>>;
    fn __processing(&self) -> Option<u64>;
    fn __processing_mut(&mut self, processing: Option<u64>) -> Option<u64>;

    /// Add a card to this stable
    fn add_card(&mut self, card: Box<dyn Card>) {
        self.__list_mut().push(card);
    }

    /// Take the card with the given uuid
    fn take_card(&mut self, uuid: u64) -> Option<Box<dyn Card>> {
        self.__list().iter().position(|c| c.uuid() == uuid).map(|index| self.__list_mut().swap_remove(index))
    }

    /// Take the card with the given uuid to process it
    /// You MUST call readd_card after this function
    fn process_card(&mut self, uuid: u64) -> Option<Box<dyn Card>> {
        assert!(self.__processing_mut(Some(uuid)).is_none());
        self.take_card(uuid)
    }

    /// Re-add the card after processing
    fn readd_card(&mut self, card: Box<dyn Card>) {
        assert!(self.__processing_mut(None).is_some());
        self.__list_mut().push(card)
    }

    /// Get the card with a given uuid
    fn get_card(&self, uuid: u64) -> Option<&Box<dyn Card>> {
        self.__list().iter().find(|c| c.uuid() == uuid)
    }

    /// Get the card with a given ID
    fn get_id_card(&self, id: CardID) -> Option<&Box<dyn Card>> {
        self.__list().iter().find(|c| c.id() == id)
    }

    /// Get the card with a given uuid mutably
    fn get_card_mut(&mut self, uuid: u64) -> Option<&mut Box<dyn Card>> {
        self.__list_mut().iter_mut().find(|c| c.uuid() == uuid)
    }

    /// Get the card with a given id mutably
    fn get_id_card_mut(&mut self, id: CardID) -> Option<&mut Box<dyn Card>> {
        self.__list_mut().iter_mut().find(|c| c.id() == id)
    }

    /// Get the amount of cards in this list
    fn len(&self) -> usize { self.__list().len() }

    /// Is this list empty?
    fn is_empty(&self) -> bool { self.__list().is_empty() }

    /// Does it contain any card that matches the function?
    fn any(&self, filter: fn(card: &Box<dyn Card>) -> bool) -> bool {
        self.__list().iter().any(filter)
    }

    /// Get a vector of all ids from all cards
    fn ids(&self) -> Vec<CardID> {
        self.__list().iter().map(|card| card.id()).collect()
    }

    /// Get a vector of all uuids from all cards
    fn uuids(&self) -> Vec<u64> {
        let mut uuids: Vec<u64> = self.__list().iter().map(|card| card.uuid()).collect();
        if let Some(uuid) = self.__processing() {
            uuids.push(uuid);
        }
        uuids
    }

    /// Get a vector of all ids from all cards matching the given filter
    fn filter_ids(&self, filter: fn(card: &dyn Card) -> bool) -> Vec<CardID> {
        self.__list().iter().filter(|card| filter(&***card)).map(|card| card.id()).collect()
    }

    /// Get a vector of all uuids from all cards matching the given filter
    fn filter_uuids(&self, filter: fn(card: &dyn Card) -> bool) -> Vec<u64> {
        self.__list().iter().filter(|card| filter(&***card)).map(|card| card.uuid()).collect()
    }

    /// Take all cards from the list
    fn take_all(&mut self) -> Vec<Box<dyn Card>> {
        // First get all IDs, then copy those, then take all of those, then put in a vector
        // Instead of blindly draining .__list_mut(), this makes sure we always call .take_card
        //  which structs may override to have side-effects.
        // self.__list().iter().map(|c| c.id()).collect::<Vec<CardID>>().iter().map(|id| self.take_card(*id).unwrap()).collect::<Vec<Box<dyn Card>>>()

        // Haha just kidding, I love efficiency and no cardlists override take_card anyhow
        // I *will* shoot my own foot with this
        // Update: Drawpile override's take_card but it's fiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiine :D
        self.__list_mut().drain(..).collect()
    }

    /// Add all cards to the list
    fn add_all(&mut self, cards: Vec<Box<dyn Card>>) {
        // Instead of just appending the cards to the raw array, we still call
        // add_card to make sure that any cardlists overriding add_card with side-effects
        // also automatically get handled by add_all.
        for card in cards {
            self.add_card(card);
        }
    }
}