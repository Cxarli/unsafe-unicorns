use crate::*;

defcard!(TinyStable);

impl Card for TinyStable {
    fn uuid(&self) -> u64 { self.uuid }

    fn name(&self) -> &'static str { "Tiny Stable" }
    fn id(&self) -> CardID { CardID::TinyStable }
    fn description(&self) -> &'static str { "If at any time you have more than 5 Unicorns in your Stable, SACRIFICE a Unicorn card." }
    fn cardtype(&self) -> CardType { CardType::Downgrade }

    fn stable_update_enter(&mut self, card: &dyn Card, player: &Player, _game: &Game) -> ResDeltas {
        // Keep in mind that the card is not YET in the stable, but WILL BE soon
        let unicorns = player.stable().count_unicorns() + (is_unicorn(card) as usize);

        if unicorns > 5 {
            let mut card_uuids = player.stable().filter_uuids(|c| is_unicorn(c));

            if is_unicorn(card) {
                card_uuids.push(card.uuid());
            }

            let card_uuid = *user_choose("Which unicorn do you want to sacrifice?", &card_uuids);

            Ok(vec!{ delta_same_player!(card_uuid, player.uuid(), Stable => Discard) })
        } else {
            delta_nothing!()
        }
    }
}