use crate::*;

defcard!(TwoForOne);


impl Card for TwoForOne {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Two-For-One" }
    fn id(&self) -> CardID { CardID::TwoForOne }
    fn cardtype(&self) -> CardType { CardType::Magic }

    fn description(&self) -> &'static str {
        "SACRIFICE a card, then DESTROY 2 cards."
    }

    fn may_be_played_by(&self, player: &Player, game: &Game) -> bool {
        // Need to have at least 1 card in the player's stable and at least 2 cards in total in the stables of others
        !player.stable().is_empty() && game.player_uuids_without(player.uuid()).iter().map(|&player_uuid| {
            game.p(player_uuid).stable().len()
        }).sum::<usize>() >= 2
    }

    fn on_play(&mut self, player: &Player, _game: &Game) -> ResDeltas {
        let mut actions = vec! {};

        let card_uuids = player.stable().uuids();
        let card_uuid = *user_choose("Which card do you want to sacrifice?", &card_uuids);

        // Sacrifice card
        actions.push(delta_same_player!(card_uuid, player.uuid(), Stable => Discard));

        // Discard the card because it's played
        actions.push(delta_same_player!(self.uuid(), player.uuid(), Hand => Discard));

        // Go to stage 2
        actions.push(delta_stage!(self.uuid(), player.uuid(), Discard => CardStage::A as u64));

        Ok(actions)
    }

    fn on_play_stage(&mut self, player: &Player, game: &Game, stage: u64) -> ResDeltas {
        match CardStage::from(stage) {
            CardStage::A => {
                let mut actions = vec! {};

                let target_uuid = *user_choose("Whose card do you want to destroy?", &game.player_uuids_without(player.uuid()));
                let card_uuid = *user_choose("Which card do you want to destroy?", &game.p(target_uuid).stable().uuids());

                // Destroy card
                actions.push(delta_same_player!(card_uuid, target_uuid, Stable => Discard));

                // Go to stage 3
                actions.push(delta_stage!(self.uuid(), player.uuid(), Discard => CardStage::B as u64));

                Ok(actions)
            },

            CardStage::B => {
                let target_uuid = *user_choose("Whose card do you want to destroy?", &game.player_uuids_without(player.uuid()));
                let card_uuid = *user_choose("Which card do you want to destroy?", &game.p(target_uuid).stable().uuids());

                // Destroy card
                Ok(vec!{ delta_same_player!(card_uuid, target_uuid, Stable => Discard) })
            },

            _ => unreachable!("Stage not used by card"),
        }
    }
}