use crate::*;

defcard!(TargetedDestruction);

impl Card for TargetedDestruction {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Targeted Destruction" }
    fn id(&self) -> CardID { CardID::TargetedDestruction }
    fn cardtype(&self) -> CardType { CardType::Magic }

    fn description(&self) -> &'static str { "SACRIFICE or DESTROY an Upgrade or Downgrade card" }

    fn get_targets(&self, _player: &Player, game: &Game) -> Vec<u64> {
        // Find the UUIDs of all players that have at least one upgrade or downgrade in their stable
        game.player_uuids().into_iter().filter(|&player_uuid| {
            game.p(player_uuid).stable().any(|card| {
                use CardType::*;
                card.cardtype() == Upgrade || card.cardtype() == Downgrade
            })
        }).collect()
    }

    fn on_play(&mut self, player: &Player, game: &Game) -> ResDeltas {
        let target_uuids = self.get_targets(player, game);
        let target_uuid = *user_choose("From which player do you want to destroy a card?", &target_uuids); // SAFE
        let target = game.p(target_uuid);

        // Select upgrade or downgrade from that player
        let card_uuids = target.stable().filter_uuids(|card| card.cardtype() == CardType::Upgrade || card.cardtype() == CardType::Downgrade);
        let card_uuid = *user_choose("Which card do you want to discard?", &card_uuids); // SAFE

        Ok(vec!{
            // Move this card to the discard pile
            delta_same_player!(self.uuid(), player.uuid(), Hand => Discard),

            // Move target's card to the discard pile
            delta_same_player!(card_uuid, target_uuid, Stable => Discard),
        })
    }
}