use crate::*;

defcard!(GoodDeal);

impl Card for GoodDeal {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Good Deal" }
    fn id(&self) -> CardID { CardID::GoodDeal }
    fn cardtype(&self) -> CardType { CardType::Magic }

    fn description(&self) -> &'static str { "DRAW 3 cards and DISCARD a card" }

    fn on_play(&mut self, player: &Player, _game: &Game) -> ResDeltas {
        let mut actions = vec!{};

        // Move this card to the discard pile
        actions.push(delta_same_player!(self.uuid(), player.uuid(), Hand => Discard));

        // Draw 3 cards
        for _ in 0..3 {
            // card_uuid=0 means we draw a random card from the pile
            actions.push(delta_same_player!(0, player.uuid(), Drawpile => Hand));
        }

        // Jump to the discard stage of this card
        // Despite the fact that the card is in the Discard pile, we still need to give
        // the player UUID to be able to run on_play_stage with the right player!
        actions.push(delta_stage!(self.uuid(), player.uuid(), Discard => CardStage::A as u64));

        Ok(actions)
    }

    fn on_play_stage(&mut self, player: &Player, _game: &Game, stage: u64) -> ResDeltas {
        match CardStage::from(stage) {
            CardStage::A => {
                let card_uuid = *user_choose("Which card do you want to discard?", &player.hand().uuids()); // SAFE

                // Move the chosen card to the discard pile
                Ok(vec!{ delta_same_player!(card_uuid, player.uuid(), Hand => Discard) })
            },

            _ => unreachable!("Stage not used by card"),
        }
    }
}