use crate::*;

pub trait Card: ::std::fmt::Debug {
    /// The pretty name of the card
    fn name(&self) -> &'static str;

    /// The unique ID of the card
    fn id(&self) -> CardID;

    /// UUID of the card
    fn uuid(&self) -> u64;

    /// The type of the card
    fn cardtype(&self) -> CardType;

    /// The description of the card
    fn description(&self) -> &'static str;

    /// May the one who has this card in their stable, play the given card?
    /// Example: Broken Stable prevents Upgrades from being played
    fn owner_may_play(&self, _card: &dyn Card) -> bool { true }

    /// If this card is in a player's stable, may the given card be destroyed by another player?
    /// Example: Rainbow Aura prevents Unicorn cards from being destroyed
    fn other_may_destroy(&self, _card: &dyn Card) -> bool { true }

    /// May this card be played by the given player in the given game?
    /// The default is to check whether this card has a target (Magic) or a destination (Upgrade|Downgrade|Unicorn)
    fn may_be_played_by(&self, player: &Player, game: &Game) -> bool {
        use CardType::*;

        match self.cardtype() {
            Unicorn(_) | Upgrade | Downgrade => {
                !self.get_destinations(player, game).is_empty()
            },

            Instant | Magic => {
                !self.get_targets(player, game).is_empty()
            },
        }
    }

    /// Get the list of UUIDs that this card could target
    fn get_targets(&self, _player: &Player, game: &Game) -> Vec<u64> { game.player_uuids() }

    /// Get the list of UUIDs of the players in whose stables this card may be played
    fn get_destinations(&self, _player: &Player, game: &Game) -> Vec<u64> { game.player_uuids() }

    /// Function that is called whenever this card is played
    fn on_play(&mut self, player: &Player, game: &Game) -> ResDeltas {
        use CardType::*;

        match self.cardtype() {
            Unicorn(_) | Upgrade | Downgrade => {
                // Also allow playing a card in someone else's stable
                let target_uuids = self.get_destinations(player, game);
                let target_uuid = *user_choose("In whose Stable do you want to play this card?", &target_uuids);

                // Move card from this player's hand to the target's stable
                Ok(vec! { delta!(self.uuid(), [player.uuid(), Hand] => [target_uuid, Stable]) })
            },

            Instant | Magic => Ok(vec!{ delta_same_player!(self.uuid(), player.uuid(), Hand => Discard) }),
        }
    }

    /// If this card has several play stages, you can define them with this function
    /// To get to another stage, use delta_stage!
    fn on_play_stage(&mut self, _player: &Player, _game: &Game, _stage: u64) -> ResDeltas { unimplemented!("This card doesn't have any stages.") }

    /// Function that is called on the beginning of the turn
    fn on_bot(&mut self, _player: &Player, _game: &Game) -> ResDeltas { delta_nothing!() }

    /// Called on the end of the turn
    fn on_eot(&mut self, _player: &Player, _game: &Game) -> ResDeltas { delta_nothing!() }

    /// Called when the card is sacrificed from the stable
    fn on_sacrifice(&mut self, _from: Destination, _player: &Player, _game: &Game) -> ResDeltas { delta_nothing!() }

    /// Called when the card is destroyed from the stable
    fn on_destroy(&mut self, _from: Destination, _player: &Player, _game: &Game) -> ResDeltas { delta_nothing!() }

    /// Called when the card is discarded from the hand
    fn on_discard(&mut self, _from: Destination, _player: &Player, _game: &Game) -> ResDeltas { delta_nothing!() }

    /// Called when the card is returned to the hand
    fn on_return_to_hand(&mut self, _from: Destination, _player: &Player, _game: &Game) -> ResDeltas { delta_nothing!() }

    // Called when the card has entered the stable
    fn on_enter_stable(&mut self, _from: Destination, _player: &Player, _game: &Game) -> ResDeltas { delta_nothing!() }

    /// Called when the card has left the stable
    fn on_leave_stable(&mut self, _player: &Player, _game: &Game) -> ResDeltas { delta_nothing!() }

    /// Called whenever a new card enters the same stable as this card
    fn stable_update_enter(&mut self, _card: &dyn Card, _player: &Player, _game: &Game) -> ResDeltas { delta_nothing!() }

    /// Called whenever a card leaves the same stable as this card
    fn stable_update_leave(&mut self, _card: &dyn Card, _player: &Player, _game: &Game) -> ResDeltas { delta_nothing!() }
}

impl PartialEq for dyn Card {
    fn eq(&self, other: &dyn Card) -> bool {
        self.uuid() == other.uuid()
    }
}

impl Eq for dyn Card {}