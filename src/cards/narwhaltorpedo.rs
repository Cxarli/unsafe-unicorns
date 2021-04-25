use crate::*;

defcard!(NarwhalTorpedo);

impl Card for NarwhalTorpedo {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Narwhal Torpedo" }
    fn id(&self) -> CardID { CardID::NarwhalTorpedo }
    fn cardtype(&self) -> CardType { CardType::Unicorn(Unicorn::Magical) }

    fn description(&self) -> &'static str { "When this card enters your Stable, SACRIFICE all Downgrade cards." }

    fn on_enter_stable(&mut self, _from: Destination, player: &Player, _game: &Game) -> ResDeltas {
        let downgrade_uuids = player.stable().filter_uuids(|c| c.cardtype() == CardType::Downgrade);

        // Move all downgrades from this player to the discard pile
        let actions = downgrade_uuids.iter()
            .map(|&card_uuid| delta_same_player!(card_uuid, player.uuid(), Stable => Discard)).collect();

        Ok(actions)
    }
}