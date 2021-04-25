use crate::*;

defcard!(BlatantThievery);

impl Card for BlatantThievery {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Blatant Thievery" }
    fn id(&self) -> CardID { CardID::BlatantThievery }
    fn cardtype(&self) -> CardType { CardType::Magic }

    fn description(&self) -> &'static str {
        "Choose any player and look at that player's hand. \
        Choose a card from that player's hand and add it to your hand."
    }

    fn get_targets(&self, player: &Player, game: &Game) -> Vec<u64> {
        // Get the UUIDs of all other players that have at least one card in their hand
        game.player_uuids_without(player.uuid()).into_iter().filter(|&player_uuid| {
            !game.p(player_uuid).hand().is_empty()
        }).collect()
    }

    fn on_play(&mut self, player: &Player, game: &Game) -> ResDeltas {
        let target_uuids = self.get_targets(player, game);
        let target_uuid = *user_choose("Which user's hand do you want to look at?", &target_uuids); // SAFE
        let target = game.p(target_uuid);

        let card_uuid = *user_choose("Which card do you want to take?", &target.hand().uuids());

        Ok(vec!{
            // Move this card to the discard pile
            delta_same_player!(self.uuid(), player.uuid(), Hand => Discard),

            // Move the target's card to this player's hand
            delta!(card_uuid, [target_uuid, Hand] => [player.uuid(), Hand]),
        })
    }
}