use crate::*;

defcard!(RainbowMane);

impl Card for RainbowMane {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Rainbow Mane" }
    fn id(&self) -> CardID { CardID::RainbowMane }
    fn cardtype(&self) -> CardType { CardType::Upgrade }

    fn description(&self) -> &'static str {
        "This card can only enter a Stable if there is a Basic Unicorn in that Stable. \
        If this card is in your Stable at the beginning of your turn, you may bring \
        a Basic Unicorn card from your hand diredtly into your Stable."
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
        let card_uuids = player.hand().filter_uuids(|card| card.cardtype() == CardType::Unicorn(Unicorn::Basic));

        if card_uuids.len() >= 1 && user_choose_bool("Do you want to activate the Rainbow Mane?") {
            let card_uuid = *user_choose("Which basic unicorn do you want to put in your stable?", &card_uuids);

            Ok(vec!{ delta_same_player!(card_uuid, player.uuid(), Hand => Stable) })
        } else {
            delta_nothing!()
        }
    }
}