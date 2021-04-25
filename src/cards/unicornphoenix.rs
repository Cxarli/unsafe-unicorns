use crate::*;

defcard!(UnicornPhoenix);

impl Card for UnicornPhoenix {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Unicorn Phoenix" }
    fn id(&self) -> CardID { CardID::UnicornPhoenix }
    fn cardtype(&self) -> CardType { CardType::Unicorn(Unicorn::Magical) }

    fn description(&self) -> &'static str {
        "When this card enters your Stable, DISCARD a card. If this card is sacrificed or destroyed, \
        bring it directly back into your Stable if you have at least 1 card in your hand."
    }

    fn may_be_played_by(&self, player: &Player, _game: &Game) -> bool {
        // Player must be able to discard a card
        player.hand().len() >= 1
    }

    fn on_enter_stable(&mut self, _from: Destination, player: &Player, _game: &Game) -> ResDeltas {
        let card_uuid = *user_choose("Choose a card to discard", &player.hand().uuids()); // SAFE

        // Move the card from the player's hand to the discard pile
        Ok(vec!{ delta_same_player!(card_uuid, player.uuid(), Hand => Discard) })
    }

    fn on_destroy(&mut self, from: Destination, player: &Player, _game: &Game) -> ResDeltas {
        if player.hand().len() >= 1 {
            // Move the card back into the stable
            Ok(vec!{ delta_same_player!(self.uuid(), player.uuid(), from => Stable) })
        } else {
            delta_nothing!()
        }
    }

    fn on_sacrifice(&mut self, from: Destination, player: &Player, game: &Game) -> ResDeltas {
        // The same happens on sacrifice as on destroy
        self.on_destroy(from, player, game)
    }
}