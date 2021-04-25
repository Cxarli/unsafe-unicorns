use crate::*;

defcard!(UnicornPoison);

impl Card for UnicornPoison {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Unicorn Poison" }
    fn id(&self) -> CardID { CardID::UnicornPoison }
    fn cardtype(&self) -> CardType { CardType::Magic }

    fn description(&self) -> &'static str { "DESTROY a Unicorn card" }

    fn get_targets(&self, player: &Player, game: &Game) -> Vec<u64> {
        // Find the UUIDs of all players (except current player) that have at least one unicorn in their stable
        game.player_uuids_without(player.uuid()).into_iter().filter(|&player_uuid| {
            game.p(player_uuid).stable().count_unicorns() >= 1
        }).collect()
    }

    fn on_play(&mut self, player: &Player, game: &Game) -> ResDeltas {
        let target_uuids = self.get_targets(player, game);
        let target_uuid = *user_choose("From which player do you want to destroy a Unicorn?", &target_uuids); // SAFE
        let target = game.p(target_uuid);

        // Select unicorns from target
        let card_uuids = target.stable().filter_uuids(is_unicorn);
        let card_uuid = *user_choose("Which Unicorn do you want to destroy?", &card_uuids); // SAFE

        Ok(vec!{
            // Move this card to the discard pile
            delta_same_player!(self.uuid(), player.uuid(), Hand => Discard),

            // Move target's card to the discard pile
            delta_same_player!(card_uuid, target_uuid, Stable => Discard),
        })
    }
}