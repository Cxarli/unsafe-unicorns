use crate::*;

defcard!(GinormousUnicorn);

impl Card for GinormousUnicorn {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Ginormous Unicorn" }
    fn id(&self) -> CardID { CardID::GinormousUnicorn }
    fn cardtype(&self) -> CardType { CardType::Unicorn(Unicorn::Magical) }

    fn description(&self) -> &'static str { "This card counts for 2 Unicorns. You cannot play any Instant cards." }

    fn owner_may_play(&self, card: &dyn Card) -> bool { card.cardtype() != CardType::Instant }

    // The counts for 2 unicorns part is handled by Stable::count_unicorns because it's too specific
    // to make an explicit function for
}