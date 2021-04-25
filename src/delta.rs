#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delta {
    pub card_uuid: u64,
    pub from: Location,
    pub to: Location,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub player_uuid: u64, // ignored iff destination=Discard|Drawpile|Nursery
    pub destination: Destination,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Destination {
    // Table
    Discard,
    Drawpile,
    Nursery,

    // Player
    Hand,
    Stable,

    // Special
    /// Make the game use the actions from a different stage of the card
    /// Only allowed in the to location
    /// to.player_uuid contains the stage number
    Stage,
    /// Update the number of cards the player may draw
    /// Only allowed in the to location
    /// to.player_uuid contains the (non-zero) DIFFERENCE of the draw number
    ///   (ie. +1 increases the number with 1, instead of setting it to 1)
    /// if to.player_uuid is 0, the number of cards the user may draw is set to 0
    UpdateDrawNumber,
}
