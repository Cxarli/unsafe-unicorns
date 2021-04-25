use crate::*;

#[derive(Debug)]
pub struct SeductiveUnicorn {
    pub uuid: u64,

    target_uuid: Option<u64>,
    stolen_card_uuid: Option<u64>,
}

impl SeductiveUnicorn {
    pub fn new() -> SeductiveUnicorn {
        SeductiveUnicorn {
            uuid: uuid(),
            target_uuid: None,
            stolen_card_uuid: None,
        }
    }
}

impl Card for SeductiveUnicorn {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Seductive Unicorn" }
    fn id(&self) -> CardID { CardID::SeductiveUnicorn }
    fn cardtype(&self) -> CardType { CardType::Unicorn(Unicorn::Magical) }

    fn description(&self) -> &'static str {
        "When this card enters your Stable, STEAL a Unicorn card. \
        If this card leaves your Stable, return that Unicorn card to the Stable from which you stole it."
    }

    fn get_targets(&self, player: &Player, game: &Game) -> Vec<u64> {
        // Find all players that aren't the current player and that have at least 1 unicorn card in their stable
        game.player_uuids_without(player.uuid()).into_iter()
            .filter(|&player_uuid| game.p(player_uuid).stable().count_unicorns() > 0).collect()
    }

    fn on_enter_stable(&mut self, _from: Destination, player: &Player, game: &Game) -> ResDeltas {
        let target_uuids = self.get_targets(player, game);
        let target_uuid = *user_choose("From which player do you want to STEAL a unicorn card?", &target_uuids); // SAFE
        let target = game.p(target_uuid);

        // Make sure we don't overwrite the old target
        if self.target_uuid.is_none() && self.stolen_card_uuid.is_none() {
            self.target_uuid = Some(target_uuid);

            // Find all unicorn cards from the target's stable
            let card_uuids = target.stable().filter_uuids(|c| is_unicorn(c));
            let card_uuid = *user_choose("Which unicorn card do you want to STEAL?", &card_uuids); // SAFE

            self.stolen_card_uuid = Some(card_uuid);

            // Move card from target.stable to player.stable
            Ok(vec!{ delta!(card_uuid, [target_uuid, Stable] => [player.uuid(), Stable]) })
        } else {
            Err("Card still in enter phase")
        }
    }

    fn on_leave_stable(&mut self, player: &Player, _game: &Game) -> ResDeltas {
        if let (Some(target_uuid), Some(stolen_card_uuid)) = (self.target_uuid, self.stolen_card_uuid) {
            // Reset fields
            self.target_uuid = None;
            self.stolen_card_uuid = None;

            // Move card back from player.stable to target.stable
            Ok(vec!{ delta!(stolen_card_uuid, [player.uuid(), Stable] => [target_uuid, Stable]) })
        } else {
            // It can theoretically happen that there were no players to steal unicorns from at the time
            // this card was played, so we don't have to panic! here
            delta_nothing!()
        }
    }
}