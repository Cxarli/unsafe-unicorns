use crate::*;

defcard!(BarbedWire);

impl Card for BarbedWire {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Barbed Wire" }
    fn id(&self) -> CardID { CardID::BarbedWire }
    fn cardtype(&self) -> CardType { CardType::Downgrade }

    fn description(&self) -> &'static str { "Each time a Unicorn card enters or leaves your Stable, DISCARD a card." }

    fn stable_update_enter(&mut self, card: &dyn Card, player: &Player, _game: &Game) -> ResDeltas {
        // @TODO: Cancel if there is no card to discard!
        assert!(!player.hand().is_empty());

        // Only trigger if the newly added card is a unicorn and there is at least one card in the player's hand
        if is_unicorn(card) {
            let card_uuid = *user_choose("Which card do you want to discard?", &player.hand().uuids()); // SAFE

            // Move the card from the player's hand to the discard pile
            Ok(vec!{ delta_same_player!(card_uuid, player.uuid(), Hand => Discard) })
        } else {
            delta_nothing!()
        }
    }

    fn stable_update_leave(&mut self, card: &dyn Card, player: &Player, game: &Game) -> ResDeltas {
        // This card has the same action on leave as on enter
        self.stable_update_enter(card, player, game)
    }
}