use crate::*;

defcard!(GlitterBomb);

impl Card for GlitterBomb {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Glitter Bomb" }
    fn id(&self) -> CardID { CardID::GlitterBomb }
    fn cardtype(&self) -> CardType { CardType::Upgrade }

    fn description(&self) -> &'static str {
        "If this card is in your Stable at the beginning of your turn, \
        you may SACRIFICE a card. If you do, DESTROY a card."
    }

    fn on_bot(&mut self, player: &Player, game: &Game) -> ResDeltas {
        if user_choose_bool("Do you want to activate the Glitter Bomb?") {
            // Get card to sacrifice
            let mut card_uuids = player.stable().uuids();

            // Also include the current card
            card_uuids.push(self.uuid());

            let card_uuid = *user_choose("Which card do you want to SACRIFICE?", &card_uuids);

            // Get player to destroy
            let target_uuids = game.player_uuids_without(player.uuid());
            let target_uuid = *user_choose("From who do you want to DESTROY a card?", &target_uuids);

            // Get card to destroy
            let target_card_uuids = game.p(target_uuid).stable().uuids();
            let target_card_uuid = *user_choose("Which card do you want to DESTROY?", &target_card_uuids);

            Ok(vec!{
                // Sacrifice card
                delta_same_player!(card_uuid, player.uuid(), Stable => Discard),

                // Destroy card
                delta_same_player!(target_card_uuid, target_uuid, Stable => Discard),
            })
        } else {
            delta_nothing!()
        }
    }
}