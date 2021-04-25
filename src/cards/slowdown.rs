use crate::*;

defcard!(Slowdown);

impl Card for Slowdown {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Slowdown" }
    fn id(&self) -> CardID { CardID::Slowdown }
    fn cardtype(&self) -> CardType { CardType::Downgrade }

    fn description(&self) -> &'static str { "You cannot play Instant cards" }

    fn owner_may_play(&self, card: &dyn Card) -> bool { card.cardtype() != CardType::Instant }
}