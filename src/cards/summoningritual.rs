use crate::*;

defcard!(SummoningRitual);


impl Card for SummoningRitual {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Summoning Ritual" }
    fn id(&self) -> CardID { CardID::SummoningRitual }
    fn cardtype(&self) -> CardType { CardType::Upgrade }

    fn description(&self) -> &'static str {
        "If this card is in your Stable at the beginning of your turn, \
        you may DISCARD 2 Unicorn cards. If you do, bring a Unicorn card \
        directly from the discard pile into your Stable."
    }

    fn on_bot(&mut self, player: &Player, _game: &Game) -> ResDeltas {
        // Check if user has two unicorn cards
        let discard_uuids = player.hand().filter_uuids(is_unicorn);

        // Check if the user wants to play
        if discard_uuids.len() >= 2 && user_choose_bool("Do you want to activate the Summoning Ritual?") {

            // Discard two unicorn cards
            let discard_uuids: Vec<u64> = user_choose_n_copy("Which cards do you want to discard?", &discard_uuids, 2); // SAFE

            let mut actions: Vec<Delta> = discard_uuids.iter().map(|&card_uuid|
                delta_same_player!(card_uuid, player.uuid(), Hand => Discard)
            ).collect();

            // Go to second stage
            actions.push(delta_stage!(self.uuid(), player.uuid(), Stable => CardStage::A as u64));

            Ok(actions)
        } else {
            delta_nothing!()
        }
    }

    fn on_play_stage(&mut self, player: &Player, game: &Game, stage: u64) -> ResDeltas {
        match CardStage::from(stage) {
            CardStage::A => {
                let card_uuids = game.table.discard.filter_uuids(is_unicorn);

                // @TODO: Cancel action if not allowed
                assert!(card_uuids.len() >= 1);

                // Revive a card from the discard pile
                let card_uuid = *user_choose("Which card do you want to revive?", &card_uuids); // SAFE

                Ok(vec!{ delta_same_player!(card_uuid, player.uuid(), Discard => Stable) })
            },

            _ => unreachable!("Stage not used by card"),
        }
    }
}