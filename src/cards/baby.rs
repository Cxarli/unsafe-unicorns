use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BabyTheme {
    Skeleton, Narwhal, Rainbow, White, Black, Yellow, Green, Blue, Red, Purple, LightPink, Pink, Hotpink,
}

#[derive(Debug)]
pub struct Baby {
    uuid: u64,
    theme: BabyTheme,
}

impl Baby {
    pub fn new(theme: BabyTheme) -> Baby {
        Baby {
            uuid: uuid(),
            theme: theme,
        }
    }

    /// Get the theme of this card
    pub fn theme(&self) -> BabyTheme {
        self.theme
    }
}

impl Card for Baby {
    fn uuid(&self) -> u64 { self.uuid }

    fn name(&self) -> &'static str { if self.theme == BabyTheme::Narwhal { "Baby Narwhal" } else { "Baby Unicorn" } }
    fn id(&self) -> CardID { CardID::Baby }
    fn description(&self) -> &'static str { "If this card would be sacrificed, destroyed, or returned to your hand, return it to the Nursery instead." }
    fn cardtype(&self) -> CardType { CardType::Unicorn(Unicorn::Baby) }

    fn on_destroy(&mut self, from: Destination, player: &Player, _game: &Game) -> ResDeltas {
        Ok(vec!{ delta_same_player!(self.uuid(), player.uuid(), from => Nursery) })
    }

    fn on_sacrifice(&mut self, from: Destination, player: &Player, game: &Game) -> ResDeltas {
        // Same as destroyed
        self.on_destroy(from, player, game)
    }

    fn on_return_to_hand(&mut self, from: Destination, player: &Player, _game: &Game) -> ResDeltas {
        Ok(vec!{ delta_same_player!(self.uuid(), player.uuid(), from => Nursery) })
    }
}