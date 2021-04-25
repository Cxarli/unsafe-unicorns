use crate::*;

defcard!(UnfairBargain);

impl Card for UnfairBargain {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Unfair Bargain" }
    fn id(&self) -> CardID { CardID::UnfairBargain }
    fn cardtype(&self) -> CardType { CardType::Magic }

    fn description(&self) -> &'static str { "Trade hands with any other player" }

    fn get_targets(&self, player: &Player, game: &Game) -> Vec<u64> {
        // Find the UUIDs of all players that have at least one card in their hand
        // Except for the current player
        game.player_uuids_without(player.uuid()).into_iter().filter(
            |&player_uuid| !game.p(player_uuid).hand().is_empty()
        ).collect()
    }

    fn on_play(&mut self, player: &Player, game: &Game) -> ResDeltas {
        let target_uuids = self.get_targets(player, game);
        let target_uuid = *user_choose("With which player do you want to trade hands?", &target_uuids); // SAFE

        let mut actions = vec!{ };

        // Move this card to the discard pile
        actions.push(delta_same_player!(self.uuid(), player.uuid(), Hand => Discard));

        // Move all cards (except this one) from this player's hand to the other player
        actions.append(&mut player.hand().uuids().into_iter().filter(|&uuid| uuid != self.uuid()).map(
            |card_uuid| delta!(card_uuid, [player.uuid(), Hand] => [target_uuid, Hand])
        ).collect());

        // Move all cards from the other player's hand to this player
        // NOTE: Since these actions are determined without being executed, this is safe
        actions.append(&mut game.p(target_uuid).hand().uuids().into_iter().map(
            |card_uuid| delta!(card_uuid, [target_uuid, Hand] => [player.uuid(), Hand])
        ).collect());

        Ok(actions)
    }
}