extern crate rand;

// --- Tests

#[cfg(test)]
pub mod tests;

// --- Helper functions and macros

/// The default return type of the card functions
/// This gives a vector of Delta's or an error.
pub type ResDeltas = Result<Vec<Delta>, &'static str>;

mod utils;
pub use crate::utils::*;

pub mod macros;
pub use crate::macros::*;

// --- All regular structs and implementations

pub mod cardid;
pub use crate::cardid::CardID;

pub mod card;
pub use crate::card::Card;

pub mod cardstage;
pub use crate::cardstage::CardStage;

pub mod cardtype;
pub use crate::cardtype::{ CardType, Unicorn };

pub mod cardlist;
pub use crate::cardlist::CardList;

pub mod delta;
pub use crate::delta::*;

pub mod discard;
pub use crate::discard::Discard;

pub mod drawpile;
pub use crate::drawpile::Drawpile;

pub mod game;
pub use crate::game::Game;

pub mod hand;
pub use crate::hand::Hand;

pub mod player;
pub use crate::player::Player;

pub mod nursery;
pub use crate::nursery::Nursery;

pub mod stable;
pub use crate::stable::Stable;

pub mod table;
pub use crate::table::Table;

// --- Last set of files to load

pub mod cards;
