use crate::*;

defcard!(ExtraTail);

impl Card for ExtraTail {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Extra Tail" }
    fn id(&self) -> CardID { CardID::ExtraTail }
    fn cardtype(&self) -> CardType { CardType::Upgrade }

    fn description(&self) -> &'static str {
        "This card can only enter a Stable if there is a Basic Unicorn in that Stable. \
        If this card is in your Stable at the beginning of your turn, you may DRAW an extra card."
    }

    fn get_destinations(&self, _player: &Player, game: &Game) -> Vec<u64> {
        // Find all players who have at least one basic unicorn in their stable
        game.player_uuids().into_iter().filter(
            |&player_uuid| game.p(player_uuid).stable().any(
                |card| card.cardtype() == CardType::Unicorn(Unicorn::Basic)
            )
        ).collect()
    }

    fn on_bot(&mut self, player: &Player, _game: &Game) -> ResDeltas {
        Ok(vec!{ delta_update_draw_number!(self.uuid(), player.uuid(), Stable => 1) })
    }
}