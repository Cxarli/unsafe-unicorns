/// Shortcut to define a basic card
#[macro_export]
macro_rules! defcard {
    ($structname:ident) => {
        use crate::uuid;

        #[derive(Debug)]
        pub struct $structname {
            uuid: u64,
        }

        impl $structname {
            pub fn new() -> Self {
                Self {
                    uuid: uuid(),
                }
            }
        }
    };
}

/// Shortcut to make a delta
#[macro_export]
macro_rules! delta {
    ($card_uuid:expr, [$playerfrom_uuid:expr, $from:ident] => [$playerto_uuid:expr, $to:ident]) => {{
        #[allow(unused_imports)]
        use Destination::*;

        Delta {
            card_uuid: $card_uuid,
            from: Location {
                player_uuid: $playerfrom_uuid,
                destination: $from,
            },
            to: Location {
                player_uuid: $playerto_uuid,
                destination: $to,
            },
        }
    }};
}

/// Shortcut to make a delta for the same player
#[macro_export]
macro_rules! delta_same_player {
    ($card_uuid:expr, $player_uuid:expr, $from:ident => $to:ident) => (delta!($card_uuid, [$player_uuid, $from] => [$player_uuid, $to]));
}

/// Shortcut to do nothing
#[macro_export]
macro_rules! delta_nothing { () => (Ok(vec!{})); }

/// Shortcut to go to a different stage
#[macro_export]
macro_rules! delta_stage {
    ($card_uuid:expr, $player_uuid:expr, $from:ident => $stage:expr) => {{
        #[allow(unused_imports)]
        use Destination::*;

        Delta {
            card_uuid: $card_uuid,
            from: Location { player_uuid: $player_uuid, destination: $from, },
            to: Location { player_uuid: $stage, destination: Destination::Stage, }
        }
    }};
}

/// Shortcut to update the number of cards the player may draw
#[macro_export]
macro_rules! delta_update_draw_number {
    ($card_uuid:expr, $player_uuid:expr, $from:ident => $diff:expr) => {{
        #[allow(unused_imports)]
        use Destination::*;

        Delta {
            card_uuid: $card_uuid,
            from: Location { player_uuid: $player_uuid, destination: $from, },
            to: Location { player_uuid: $diff, destination: Destination::UpdateDrawNumber, }
        }
    }};
}

/// Shortcut to get a boxed card of the given type
#[macro_export]
macro_rules! card {
    ($type:ident) => (Box::new(cards::$type::new()));
}

/// Shortcut to get a boxed baby unicorn card of the given theme
#[macro_export]
macro_rules! baby {
    ($theme:ident) => (Box::new(cards::Baby::new(cards::BabyTheme::$theme)));
}

/// Shortcut to get a boxed basic unicorn card of the given theme
#[macro_export]
macro_rules! basic {
    ($theme:ident) => (Box::new(cards::Basic::new(cards::BasicTheme::$theme)));
}