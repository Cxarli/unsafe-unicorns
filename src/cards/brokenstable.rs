use crate::*;

defcard!(BrokenStable);

impl Card for BrokenStable {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Broken Stable" }
    fn id(&self) -> CardID { CardID::BrokenStable }
    fn cardtype(&self) -> CardType { CardType::Downgrade }

    fn description(&self) -> &'static str { "You cannot play Upgrade cards" }

    fn owner_may_play(&self, card: &dyn Card) -> bool { card.cardtype() != CardType::Upgrade }
}