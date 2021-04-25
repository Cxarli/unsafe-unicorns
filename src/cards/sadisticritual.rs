use crate::*;

defcard!(SadisticRitual);

impl Card for SadisticRitual {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Sadistic Ritual" }
    fn id(&self) -> CardID { CardID::SadisticRitual }
    fn cardtype(&self) -> CardType { CardType::Downgrade }

    fn description(&self) -> &'static str {
        "If this card is in your Stable at the beginning of your turn, \
        SACRIFICE a Unicorn card, then DRAW a card."
    }

    fn on_bot(&mut self, player: &Player, game: &Game) -> ResDeltas {
        let card_uuids = player.stable().filter_uuids(|c| is_unicorn(c));

        // Make sure we have the necessary cards to be able to play this card
        if !card_uuids.is_empty() && !game.table.drawpile.is_empty() {
            let card_uuid = *user_choose("Which Unicorn do you want to sacrifice?", &card_uuids); // SAFE

            Ok(vec!{
                // Move the chosen card to the discard pile
                delta_same_player!(card_uuid, player.uuid(), Stable => Discard),

                // Draw a new card
                delta_same_player!(0, player.uuid(), Drawpile => Hand),
            })
        } else {
            delta_nothing!()
        }
    }
}