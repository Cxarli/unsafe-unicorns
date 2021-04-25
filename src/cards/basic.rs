use crate::{Card, CardID, CardType, Unicorn, uuid};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicTheme {
    PoppedCollars, VinylRecords, Dancer, Emoji, Beards, Hashtags, PumpkinSpice, Narwhal,
}

#[derive(Debug)]
pub struct Basic {
    uuid: u64,
    theme: BasicTheme,
}

impl Basic {
    pub fn new(theme: BasicTheme) -> Basic {
        Basic {
            uuid: uuid(),
            theme: theme,
        }
    }

    /// Get the theme of this card
    pub fn theme(&self) -> BasicTheme {
        self.theme
    }
}

impl Card for Basic {
    fn uuid(&self) -> u64 { self.uuid }
    fn name(&self) -> &'static str { if self.theme == BasicTheme::Narwhal { "Narwhal" } else { "Basic Unicorn" } }
    fn id(&self) -> CardID { CardID::Basic }
    fn cardtype(&self) -> CardType { CardType::Unicorn(Unicorn::Basic) }

    fn description(&self) -> &'static str {
        use BasicTheme::*;

        match self.theme {
            PoppedCollars => "Popped collars are for date nights only.",
            VinylRecords => "Vinyl records and mixtapes only.",
            Dancer => "Dance like nobody's watching.",
            Emoji => "💖🙌💅🙌💖💁💁😂😂😂",
            Beards => "Beards are like, so hot.",
            Hashtags => "#nomakeup #nofilter #sunnies #shameless #selfie #basic #TGIF #unicornhairdontcare",
            PumpkinSpice => "Pumpkin spice is the pumpkin spice of life.",
            Narwhal => "This card has no special powers, but it sure is cute!",
        }
    }
}