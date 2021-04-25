use crate::*;

#[derive(Debug)]
pub struct Player {
    uuid: u64,

    name: String,
    hand: Hand,
    stable: Stable,

    /// The number of cards this player may draw in their draw phase
    /// This can be increased by cards (eg. Extra Tail) and
    ///   is reset by the Game before player.on_bot is called
    pub may_draw_number: u64,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            uuid: uuid(),
            name: name,
            hand: Hand::new(),
            stable: Stable::new(),

            may_draw_number: 1,
        }
    }

    /// Get UUID
    pub fn uuid(&self) -> u64 {
        self.uuid
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Lend a hand
    pub fn hand(&self) -> &Hand {
        &self.hand
    }

    /// Lend a mutable hand (scary)
    pub fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }

    /// Get stable
    pub fn stable(&self) -> &Stable {
        &self.stable
    }

    /// Get mutable stable
    pub fn stable_mut(&mut self) -> &mut Stable {
        &mut self.stable
    }

    /// Does this player have a card that makes their hand visible at all times?
    /// Example: Nanny Cam
    pub fn is_hand_visible(&self) -> bool {
        self.stable.ids().contains(&CardID::NannyCam)
    }

    /// Check if this player may play this card
    pub fn may_play(&self, card: &dyn Card) -> bool {
        self.stable.owner_may_play(card)
    }

    /// Called on the beginning of turn
    pub fn on_bot(&mut self, game: &Game) -> ResDeltas {
        let mut actions = vec!{};

        self.stable.reset_cards();

        // Take card from stable temporarily to call on_bot
        while let Some(mut card) = self.stable.next_card() {
            match card.on_bot(self, game) {
                Err(err) => { return Err(err); },

                Ok(mut card_actions) => {
                    // Put the card back in the stable
                    self.stable.readd_card(card);

                    // Add this card's actions
                    actions.append(&mut card_actions);
                },
            }
        }

        Ok(actions)
    }

    /// Called on the draw phase
    pub fn on_draw(&mut self, game: &Game) -> ResDeltas {
        let upperbound = ::std::cmp::min(self.may_draw_number as usize, game.table.drawpile.len());

        let number = {
            if upperbound <= 1 {
                // Don't ask for 0 or 1 cards
                upperbound
            } else {
                // This is reversed because players probably want to draw as many cards as possible
                // Just kidding, it's because we always choose the first one in testing and the
                // tests rely on the player drawing as many cards as allowed
                *user_choose("How many cards do you want to draw?", &(1..=upperbound).rev().collect())
            }
        };

        // Draw `draw` cards
        Ok((0..number).map(|_| delta_same_player!(0, self.uuid(), Drawpile => Hand)).collect())
    }

    /// Called on the action phase
    pub fn on_action(&mut self, game: &Game) -> ResDeltas {
        // Let the player choose between playing a card and drawing another card
        if user_choose_bool("Do you want to play a card?") {
            unimplemented!("Choose a card and play it")
        } else {
            // Reset draw number to 1
            self.may_draw_number = 1;

            // Draw another card
            self.on_draw(game)
        }
    }

    /// Called on the end of turn
    pub fn on_eot(&mut self, game: &Game) -> ResDeltas {
        let mut actions = vec!{};

        self.stable.reset_cards();

        // @TODO @FIXME @HACK take cards from stable temporarily to call on_bot
        while let Some(mut card) = self.stable.next_card() {
            match card.on_eot(self, game) {
                Err(err) => { return Err(err); },

                Ok(mut card_actions) => {
                    // Put the card back in the stable
                    self.stable.readd_card(card);

                    // Add this card's actions
                    actions.append(&mut card_actions);
                },
            }
        }

        Ok(actions)
    }
}