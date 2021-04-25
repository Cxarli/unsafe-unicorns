use crate::*;

defcard!(ShakeUp);

impl Card for ShakeUp {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Shake Up" }
    fn id(&self) -> CardID { CardID::ShakeUp }
    fn cardtype(&self) -> CardType { CardType::Magic }

    fn description(&self) -> &'static str { "Shuffle this card, your hand, and the discard pile into the deck. DRAW 5 cards." }

    fn on_play(&mut self, player: &Player, game: &Game) -> ResDeltas {
        let mut actions = vec!{};

        // Move this card to the drawpile
        actions.push(delta_same_player!(self.uuid(), player.uuid(), Hand => Drawpile));

        // Move all cards in this player's hand to the drawpile
        actions.append(&mut player.hand().uuids().iter()
            .map(|&card_uuid| delta_same_player!(card_uuid, player.uuid(), Hand => Drawpile)).collect());

        // Move all cards from the discard pile to the drawpile
        actions.append(&mut game.table.discard.uuids().iter()
            .map(|&card_uuid| delta_same_player!(card_uuid, player.uuid(), Discard => Drawpile)).collect());

        // Make the player draw 5 cards
        for _ in 0..5 {
            actions.push(delta_same_player!(0, player.uuid(), Drawpile => Hand));
        }

        Ok(actions)
    }
}