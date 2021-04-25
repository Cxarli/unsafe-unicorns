use crate::*;

defcard!(RainbowAura);

impl Card for RainbowAura {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Rainbow Aura" }
    fn id(&self) -> CardID { CardID::RainbowAura }
    fn cardtype(&self) -> CardType { CardType::Upgrade }

    fn description(&self) -> &'static str { "Your Unicorn cards cannot be destroyed." }

    fn other_may_destroy(&self, card: &dyn Card) -> bool { !is_unicorn(card) }
}