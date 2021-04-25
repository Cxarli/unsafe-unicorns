use crate::*;

use std::collections::{ HashMap, VecDeque };

#[derive(Debug)]
pub struct Game {
    pub table: Table,
    pub players: HashMap<u64, Player>,

    removed_player_uuid: Option<u64>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            table: Table::new(),
            players: HashMap::new(),
            removed_player_uuid: None,
        }
    }

    /// Create a new player in this game
    pub fn new_player(&mut self, name: String) -> u64 {
        let player = Player::new(name.clone());
        let uuid = player.uuid();
        self.players.insert(uuid, player);
        uuid
    }

    /// Easy new player. Use a &str instead of a String
    pub fn ez_new_player(&mut self, name: &str) -> u64 {
        self.new_player(name.to_owned())
    }

    /// Get the UUID of the current player
    fn current_player_uuid(&self) -> Option<u64> {
        // @TODO Get current player instead of choosing a player at random
        let mut names = self.players.values().map(|player| (player.name(), player.uuid())).collect::<Vec<(&String, u64)>>();
        names.sort_unstable();
        names.get(0).map(|(_, uuid)| *uuid)
    }

    /// Apply all actions in the actions vector
    /// (this is a wrapper before apply_actions_deque to turn the vec into a vecdeque)
    pub fn apply_actions(&mut self, actions: Vec<Delta>) -> Result<(), &'static str> {
        self.apply_actions_deque(VecDeque::from(actions))
    }

    /// Get a cardlist by destination
    fn list_by_location(&mut self, location: &Location) -> &mut dyn CardList {
        match location.destination {
            // Table
            Destination::Nursery => &mut self.table.nursery as &mut dyn CardList,
            Destination::Drawpile => &mut self.table.drawpile as &mut dyn CardList,
            Destination::Discard => &mut self.table.discard as &mut dyn CardList,

            // Player
            Destination::Hand => self.p_mut(location.player_uuid).hand_mut() as &mut dyn CardList,
            Destination::Stable => self.p_mut(location.player_uuid).stable_mut() as &mut dyn CardList,

            // Special
            Destination::Stage | Destination::UpdateDrawNumber => unimplemented!("Invalid destination Stage"),
        }
    }

    /// Get the actions corresponding to the stage we jumped to
    fn jump_to_stage(&mut self, action: &Delta) -> ResDeltas {
        // Take the respective card
        match self.list_by_location(&action.from).process_card(action.card_uuid) {
            None => Err("game#apply_special_action: card_uuid not in from"),

            Some(mut card) => {
                // Get the actions that happen at this stage
                // Keep in mind that we stored the stage in action.to.player_uuid
                let actions = card.on_play_stage(self.p(action.from.player_uuid), self, action.to.player_uuid);

                // Put the card back in the list
                self.list_by_location(&action.from).readd_card(card);

                actions
            },
        }
    }

    fn leave_stable(&mut self, action: &Delta, card: &mut Box<dyn Card>) -> ResDeltas {
        match card.on_leave_stable(self.p(action.from.player_uuid), self) {
            Err(err) => Err(err),

            Ok(mut actions) => {

                // Temporarily remove player from game to allow to take cards from stable
                match self.process_player(action.from.player_uuid) {
                    None => Err("game#leave_stable: Unknown player in from"),

                    Some(mut from_player) => {
                        // Prepare to loop over stable
                        from_player.stable_mut().reset_cards();

                        while let Some(mut c) = from_player.stable_mut().next_card() {

                            // Call stable_update_leave on all cards in this stable
                            // @TODO This could possibly depend on the actions from card.on_leave_stable
                            let more_actions = c.stable_update_leave(&**card, &from_player, self);

                            // Add the card back after processing
                            from_player.stable_mut().readd_card(c);

                            if more_actions.is_err() {
                                return more_actions;
                            }

                            actions.append(&mut more_actions.unwrap());  // CONFIRMED SAFE UNWRAP
                        }

                        self.readd_player(from_player);

                        Ok(actions)
                    }
                }
            },
        }
    }

    fn enter_stable(&mut self, action: &Delta, card: &mut Box<dyn Card>) -> ResDeltas {
        match card.on_enter_stable(action.from.destination, self.p(action.to.player_uuid), self) {
            Err(err) => Err(err),

            Ok(mut actions) => {
                // Temporarily remove player from game to allow to take cards from stable
                match self.process_player(action.to.player_uuid) {
                    None => Err("game#enter_stable: Unknown player in to"),

                    Some(mut to_player) => {
                        // Reset cards in order to loop over them
                        to_player.stable_mut().reset_cards();

                        // Loop over all cards in the player's stable
                        while let Some(mut c) = to_player.stable_mut().next_card() {

                            // Get the stable_update_enter actions
                            // @TODO This could possibly depend on the actions from card.on_enter_stable
                            let more_actions = c.stable_update_enter(&**card, &to_player, self);

                            // Add the card back after processing
                            to_player.stable_mut().readd_card(c);

                            if more_actions.is_err() {
                                return more_actions;
                            }

                            actions.append(&mut more_actions.unwrap());  // CONFIRMED SAFE UNWRAP
                        }

                        self.readd_player(to_player);

                        Ok(actions)
                    },
                }
            },
        }
    }

    /// Apply all actions in the actions vecdeque
    /// @TODO Split up in smaller helper functions because this is a madness
    pub fn apply_actions_deque(&mut self, mut actions: VecDeque<Delta>) -> Result<(), &'static str> {
        println!("\n\n################");

        while let Some(action) = actions.pop_front() {
            // Debugging
            println!("### apply action");
            println!(" +    self: {:?}\n", self);
            println!(" +    action: {:?}\n", action);

            /// Shortcut to add more actions or return on errors
            macro_rules! add_more_actions {
                ($more:expr) => {
                    let more_actions = $more;

                    if let Err(err) = more_actions {
                        return Err(err);
                    }

                    let more_actions = more_actions.unwrap();  // CONFIRMED SAFE UNWRAP

                    // @TODO: Add front or back?
                    actions.append(&mut VecDeque::from(more_actions));
                };
            }

            // Handle special stage change action
            if action.to.destination == Destination::Stage {
                add_more_actions!(self.jump_to_stage(&action));

                // Don't do anything from the normal flow
                continue;
            }

            // Handle special update draw number action
            if action.to.destination == Destination::UpdateDrawNumber {
                let draw_number = action.to.player_uuid;

                if draw_number == 0 {
                    self.p_mut(action.from.player_uuid).may_draw_number = 0;
                } else {
                    self.p_mut(action.from.player_uuid).may_draw_number += draw_number;
                }

                // Don't do anything from the normal flow
                continue;
            }

            // Take the card from "from"
            match self.list_by_location(&action.from).take_card(action.card_uuid) {
                None => { return Err("game#apply_actions: Invalid delta: card_uuid not in from location"); },

                Some(mut card) => {
                    use Destination::*;

                    // Add on_leave_stable and stable_update_leave for old stable
                    if action.from.destination == Stable { add_more_actions!(self.leave_stable(&action, &mut card)); }

                    // Add on_enter_stable and stable_update_enter for new stable
                    if action.to.destination == Stable { add_more_actions!(self.enter_stable(&action, &mut card)); }

                    // Check if we need to do more special actions
                    // This match is chosen instead of if-statements so that we get a nice compiler warning
                    // when we add new destinations (which is exactly what we want)
                    match (action.from.destination, action.to.destination) {

                        // --- stuff that needs more actions ---

                        // If the card was discarded
                        (Hand, Discard) => { add_more_actions!(card.on_discard(action.to.destination, self.p(action.from.player_uuid), self)); },

                        // If the card was destroyed or sacrificed
                        (Stable, Discard) => { add_more_actions!(card.on_destroy(action.to.destination, self.p(action.from.player_uuid), self)); },

                        // If the card was moved to a player's hand
                        (_, Hand) => { add_more_actions!(card.on_return_to_hand(action.to.destination, self.p(action.to.player_uuid), self)); },

                        // --- stuff that this match doesn't have to care about ---

                        // Everything from/to stable has already been handled
                        (Stable, _) | (_, Stable) => {},

                        // Moved back to nursery|drawpile needs no special action
                        (_, Nursery) | (_, Drawpile) => {},

                        // Everything to Stage|UpdateDrawNumber can't reach this match
                        (_, Stage) | (_, UpdateDrawNumber) => unreachable!(),

                        // --- stuff that's illegal ---

                        // Discarded from other locations shouldn't be possible
                        (Discard, Discard) | (Drawpile, Discard) | (Nursery, Discard) => panic!("Invalid from/to pair"),

                        // Everything from Stage and UpdateDrawNumber is illegal
                        (Stage, _) | (UpdateDrawNumber, _) => panic!("Invalid from location"),

                        // **DO NOT DO THIS**
                        // We explicitly want the compiler to give an error when a new destination is added
                        // (_, _) => ...

                    }

                    // Add card to "to"
                    self.list_by_location(&action.to).add_card(card);
                },
            }
        }

        // Debugging
        println!("\n### ---\n");
        println!(" +  self: {:?}", self);
        println!("################\n\n");

        Ok(())
    }


    /// Called on the Beginning of Turn phase
    pub fn bot(&mut self) -> Result<(), &'static str> {
        if let Some(player_uuid) = self.current_player_uuid() {
            // Temporarily take player from game to call on_bot on it
            let mut player = self.process_player(player_uuid).unwrap();

            // Reset draw number
            player.may_draw_number = 1;

            let actions = player.on_bot(self);

            self.readd_player(player);

            actions.and_then(|actions| self.apply_actions(actions))
        } else {
            Err("game#bot: No players")
        }
    }

    /// Called on the Draw phase
    pub fn draw(&mut self) -> Result<(), &'static str> {
        if let Some(player_uuid) = self.current_player_uuid() {
            // Take player temporarily from game to call on_eot on it
            let mut player = self.process_player(player_uuid).unwrap();

            let actions = player.on_draw(self);

            self.readd_player(player);

            actions.and_then(|actions| self.apply_actions(actions))
        } else {
            Err("game#eot: No players")
        }
    }

    /// Called on the End of Turn phase
    pub fn eot(&mut self) -> Result<(), &'static str> {
        if let Some(player_uuid) = self.current_player_uuid() {
            // Take player temporarily from game to call on_eot on it
            let mut player = self.process_player(player_uuid).unwrap();

            let actions = player.on_eot(self);

            self.readd_player(player);

            actions.and_then(|actions| self.apply_actions(actions))
        } else {
            Err("game#eot: No players")
        }
    }

    /// Get the UUIDs of the players of this game
    pub fn player_uuids(&self) -> Vec<u64> {
        // Apparently the compiler can't go from Keys<&u64> to Vec<u64> directly,
        // so we need a .map(|&x| x) redirection step to force copying the u64
        let mut player_uuids: Vec<u64> = self.players.keys().map(|&x| x).collect();

        if let Some(removed_player_uuid) = self.removed_player_uuid {
            player_uuids.push(removed_player_uuid);
        }

        // This shouldn't matter, but for testing and debugging matters, we sort the UUIDs
        player_uuids.sort();

        player_uuids
    }

    /// Get the UUIDs of the players of this game, except for player_uuid
    pub fn player_uuids_without(&self, player_uuid: u64) -> Vec<u64> {
        self.player_uuids().into_iter().filter(|&x| x != player_uuid).collect()
    }


    /// Allow to temporarily remove a player from the game
    /// You MUST call readd_player after this
    pub fn process_player(&mut self, uuid: u64) -> Option<Player> {
        assert!(self.removed_player_uuid.is_none(), "Can't take several players at the same time for your own sanity.");

        self.removed_player_uuid = Some(uuid);

        self.players.remove(&uuid)
    }

    /// Allow to re-add a removed player back into the game
    pub fn readd_player(&mut self, player: Player) {
        assert!(self.removed_player_uuid.is_some(), "You didn't take a player so there's nothing to give back.");

        self.players.insert(self.removed_player_uuid.unwrap(), player);

        self.removed_player_uuid = None;
    }



    /// Get player by name NOTE: This unwraps!!
    pub fn p(&self, player_uuid: u64) -> &Player {
        self.players.get(&player_uuid).unwrap()
    }

    /// Get player mutably by name NOTE: This unwraps!!
    pub fn p_mut(&mut self, player_uuid: u64) -> &mut Player {
        self.players.get_mut(&player_uuid).unwrap()
    }

    /// Check if the player may play this card
    pub fn p_may_play(&self, player_uuid: u64, card: &dyn Card) -> Result<bool, &'static str> {
        // We need to check two things:
        // First the card checks if the player may play it, then the player checks if it may play the card.
        // The first case is for example blocked when a card requires a unicorn in the player's stable.
        // The second case is for example blocked when a card in the player's stable blocks
        //   the player from playing instant cards and the card is one.
        Ok(card.may_be_played_by(self.p(player_uuid), self) && self.p(player_uuid).may_play(card))
    }

    /// Make player draw a card
    pub fn p_draw(&mut self, player_uuid: u64) -> Result<(), &'static str> {
        if self.table.drawpile.len() >= 1 {
            // Move a random card (uuid=0) from the drawpile to the player's hand
            self.apply_actions(vec!{ delta_same_player!(0, player_uuid, Drawpile => Hand) })
        } else {
            Err("game#p_draw: There are no more cards to draw")
        }
    }

    /// Make the player play the given card
    pub fn p_play(&mut self, player_uuid: u64, card_uuid: u64) -> Result<(), &'static str> {
        // Take the card from the player to make it able to modify itself without modifying the player and game
        if let Some(mut card) = self.p_mut(player_uuid).hand_mut().take_card(card_uuid) {

            // Check if the player may even play this card
            let result = match self.p_may_play(player_uuid, &*card) {
                // Get the card's on_play actions
                Ok(true) => card.on_play(self.p(player_uuid), self),

                Ok(false) => Err("game#p_play: Player may not play card"),

                Err(err) => Err(err),
            };

            // Add card back because we only removed it temporarily
            self.p_mut(player_uuid).hand_mut().add_card(card);

            // Apply the actions
            result.and_then(|actions| self.apply_actions(actions))
        } else {
            Err("game#p_play: Card unknown")
        }
    }

    /// Easy function to make the player play the card with the given ID
    /// Mostly used for ease of testing
    pub fn p_play_id(&mut self, player_uuid: u64, card_id: CardID) -> Result<(), &'static str> {
        if let Some(card) = self.p(player_uuid).hand().get_id_card(card_id) {
            let card_uuid = card.uuid();
            self.p_play(player_uuid, card_uuid)
        } else {
            Err("game#p_play_id: No such card")
        }
    }

    /// Make the player sacrifice the given card
    /// Mostly used for testing
    pub fn p_sacrifice(&mut self, player_uuid: u64, card_uuid: u64) -> Result<(), &'static str> {
        self.apply_actions(vec!{ delta_same_player!(card_uuid, player_uuid, Stable => Discard) })
    }

    /// Shortcut to sacrifice a card id
    pub fn p_sacrifice_id(&mut self, player_uuid: u64, card_id: CardID) -> Result<(), &'static str> {
        if let Some(card) = self.p(player_uuid).stable().get_id_card(card_id) {
            let card_uuid = card.uuid();
            self.p_sacrifice(player_uuid, card_uuid)
        } else {
            Err("game#p_sacrifice_id: No such card")
        }
    }
}