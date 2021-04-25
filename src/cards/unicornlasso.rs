use crate::*;

#[derive(Debug)]
pub struct UnicornLasso {
    pub uuid: u64,

    target_uuid: Option<u64>,
    stolen_card_uuid: Option<u64>,
}

impl UnicornLasso {
    pub fn new() -> UnicornLasso {
        UnicornLasso {
            uuid: uuid(),
            target_uuid: None,
            stolen_card_uuid: None,
        }
    }
}

impl Card for UnicornLasso {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Unicorn Lasso" }
    fn id(&self) -> CardID { CardID::UnicornLasso }
    fn cardtype(&self) -> CardType { CardType::Upgrade }

    fn description(&self) -> &'static str {
        "If this card is in your Stable at the beginning of your turn, \
        you may STEAL a Unicorn card. At the end of your turn, return that \
        Unicorn card to the Stable from which you stole it."
    }

    fn get_targets(&self, player: &Player, game: &Game) -> Vec<u64> {
        // Find all players that aren't the current player and that have at least 1 unicorn card in their stable
        game.player_uuids_without(player.uuid()).into_iter()
            .filter(|&player_uuid| game.p(player_uuid).stable().count_unicorns() > 0).collect()
    }

    fn on_bot(&mut self, player: &Player, game: &Game) -> ResDeltas {

        if user_choose_bool("Do you want to play the Unicorn Lasso?") {
            let target_uuids = self.get_targets(player, game);
            let target_uuid = *user_choose("From which player do you want to STEAL a card?", &target_uuids); // SAFE
            let target = game.p(target_uuid);

            // Make sure we don't overwrite the old target
            if self.target_uuid.is_none() && self.stolen_card_uuid.is_none() {
                self.target_uuid = Some(target_uuid);

                // Find all unicorn cards from the target's stable
                let card_uuids = target.stable().filter_uuids(|c| is_unicorn(c));
                let card_uuid = *user_choose("Which card do you want to STEAL?", &card_uuids); // SAFE

                self.stolen_card_uuid = Some(card_uuid);

                // Move card from target.stable to player.stable
                Ok(vec!{ delta!(card_uuid, [target_uuid, Stable] => [player.uuid(), Stable]) })
            } else {
                Err("Card still in BOT phase")
            }
        } else {
            delta_nothing!()
        }
    }

    fn on_eot(&mut self, player: &Player, _game: &Game) -> ResDeltas {
        if let (Some(target_uuid), Some(stolen_card_uuid)) = (self.target_uuid, self.stolen_card_uuid) {
            // Reset fields
            self.target_uuid = None;
            self.stolen_card_uuid = None;

            // Move card back from player.stable to target.stable
            Ok(vec!{ delta!(stolen_card_uuid, [player.uuid(), Stable] => [target_uuid, Stable]) })
        } else {
            delta_nothing!()
        }
    }
}