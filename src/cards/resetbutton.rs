use crate::*;

defcard!(ResetButton);


impl Card for ResetButton {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Reset Button" }
    fn id(&self) -> CardID { CardID::ResetButton }
    fn cardtype(&self) -> CardType { CardType::Magic }

    fn description(&self) -> &'static str {
        "Each player must SACRIFICE all Upgrade and Downgrade cards. \
        Shuffle the discard pile into the deck."
    }

    fn on_play(&mut self, player: &Player, game: &Game) -> ResDeltas {
        let mut actions = vec! {};

        // Discard the current card
        actions.push(delta_same_player!(self.uuid(), player.uuid(), Hand => Discard));

        // Move all Upgrades and Downgrades from all players to the Discard pile
        actions.append(&mut game.player_uuids().into_iter().map(
            |player_uuid| game.p(player_uuid).stable().filter_uuids(
                |card| card.cardtype() == CardType::Upgrade || card.cardtype() == CardType::Downgrade
            ).into_iter().map(
                |card_uuid| delta_same_player!(card_uuid, player_uuid, Stable => Discard)
            ).collect::<Vec<Delta>>()
        ).flatten().collect::<Vec<Delta>>()); // @TODO: Flatten

        // Go to stage 2
        actions.push(delta_stage!(self.uuid(), player.uuid(), Discard => CardStage::A as u64));

        Ok(actions)
    }

    fn on_play_stage(&mut self, _player: &Player, game: &Game, stage: u64) -> ResDeltas {
        println!("-------------------on_play_stage------------------");
        println!("{:?}", game);

        match CardStage::from(stage) {
            CardStage::A => {
                // Shuffle the discard pile into the drawpile
                Ok(game.table.discard.uuids().into_iter().map(
                    |card_uuid| delta_same_player!(card_uuid, 0, Discard => Drawpile)
                ).collect())
            },

            _ => unreachable!("Stage not used by card"),
        }
    }
}