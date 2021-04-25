use crate::*;

defcard!(NannyCam);

impl Card for NannyCam {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { "Nannycam" }
    fn id(&self) -> CardID { CardID::NannyCam }
    fn cardtype(&self) -> CardType { CardType::Downgrade }

    fn description(&self) -> &'static str { "Your hand must be visible to all players at all times." }

    // This card is handled by Player::is_hand_visible because it's too unique to make
    // an explicit function for
}