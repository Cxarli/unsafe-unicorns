use crate::*;

macro_rules! assert_eq_ids {
    ($left:expr, $right:expr) => {
        assert_eq!(id_sort($left), id_sort($right));
    };
}

fn id_sort(mut cards: Vec<CardID>) -> Vec<CardID> {
    cards.sort();
    cards
}

#[test]
pub fn test_baby() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    let player = game.p_mut(uuid1);
    player.hand_mut().add_card(baby!(Yellow));
    player.hand_mut().add_card(baby!(White));
    player.hand_mut().add_card(baby!(Purple));
    player.hand_mut().add_card(baby!(Black));

    assert_eq!(game.p_play_id(uuid1, Baby).unwrap(), ());
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ Baby });

    assert_eq!(game.p_sacrifice_id(uuid1, Baby).unwrap(), ());
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ });
    assert_eq_ids!(game.table.nursery.ids(), vec!{ Baby });
}


#[test]
pub fn test_shakeup() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    for _ in 0..3 {
        game.p_mut(uuid1).hand_mut().add_card(basic!(Narwhal));
    }
    for _ in 0..10 {
        game.table.drawpile.add_card(card!(ShakeUp));
        game.table.discard.add_card(card!(RainbowAura));
    }

    assert_eq!(game.p_draw(uuid1).unwrap(), ());

    assert_eq!(game.p(uuid1).hand().len(), 4);
    assert_eq!(game.table.discard.len(), 10);
    assert_eq!(game.table.drawpile.len(), 9);

    assert_eq!(game.p_play_id(uuid1, ShakeUp).unwrap(), ());

    assert_eq!(game.p(uuid1).hand().len(), 5);
    assert_eq!(game.table.discard.len(), 0);
    assert_eq!(game.table.drawpile.len(), 18);
}

#[test]
pub fn test_narwhal_torpedo() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    let stable = game.p_mut(uuid1).stable_mut();
    for _ in 0..3 {
        stable.add_card(basic!(Narwhal));
        stable.add_card(card!(RainbowAura));
        stable.add_card(card!(BrokenStable));
    }

    game.p_mut(uuid1).hand_mut().add_card(card!(NarwhalTorpedo));

    // Narwhal Torpedo: When this card enters your Stable, SACRIFICE all Downgrade cards
    assert_eq!(game.p_play_id(uuid1, NarwhalTorpedo).unwrap(), ());

    let stable = game.p(uuid1).stable();
    assert_eq_ids!(game.table.discard.ids(), [BrokenStable, BrokenStable, BrokenStable].to_vec());
    assert_eq_ids!(stable.ids(), [Basic, Basic, Basic, RainbowAura, RainbowAura, RainbowAura, NarwhalTorpedo].to_vec());
}

#[test]
pub fn test_sadistic_ritual() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    // After every time we call a function on the game that takes a mutable refernce,
    // our read-only pointers become invalid. To ease debugging, I've made a helper
    // macro that refreshes the pointers again. Because of the nature of Rust's
    // hygenic macros, we have to specify the names every time.
    // This macro is thus used as:
    //    extract!(player, drawpile, discard);
    // Disclaimer: No liability is given when the parameters are in the wrong order
    macro_rules! extract {
        ($player:ident, $drawpile:ident, $discard:ident) => {
            let $player = game.p(uuid1);
            let $drawpile = &game.table.drawpile;
            let $discard = &game.table.discard;
        };
    }

    game.p_mut(uuid1).stable_mut().add_card(basic!(Dancer));
    game.p_mut(uuid1).stable_mut().add_card(card!(SadisticRitual));

    // We depend on the order of the drawpile for testing, so we disable auto-shuffle
    // before adding cards to the drawpile
    game.table.drawpile.auto_shuffle = false;
    game.table.drawpile.add_card(card!(BrokenStable));
    game.table.drawpile.add_card(card!(GinormousUnicorn));

    // Hand:
    // Stable: Basic(Dancer), SadisticRitual
    // Drawpile: BrokenStable, GinormousUnicorn
    // Discard:

    // Sadistic Ritual:
    // On beginning of turn:
    //  1. Sacrifice a unicorn card
    //  2. Draw a card
    assert_eq!(game.bot().unwrap(), ());
    extract!(player, drawpile, discard);


    // Hand: BrokenStable
    // Stable: SadisticRitual
    // Drawpile: GinormousUnicorn
    // Discard: Basic(Dancer)

    assert_eq_ids!(player.hand().ids(), vec!{ BrokenStable });
    assert_eq_ids!(player.stable().ids(), vec!{ SadisticRitual });
    assert_eq_ids!(drawpile.ids(), vec!{ GinormousUnicorn });
    assert_eq_ids!(discard.ids(), vec!{ Basic });


    // The player doesn't have any more unicorns, so shouldn't draw more cards
    assert_eq!(game.bot().unwrap(), ());
    extract!(player, drawpile, discard);

    // Hand: BrokenStable
    // Stable: SadisticRitual
    // Drawpile: GinormousUnicorn
    // Discard: Basic(Dancer)

    assert_eq_ids!(player.hand().ids(), vec!{ BrokenStable });
    assert_eq_ids!(player.stable().ids(), vec!{ SadisticRitual });
    assert_eq_ids!(drawpile.ids(), vec!{ GinormousUnicorn });
    assert_eq_ids!(discard.ids(), vec!{ Basic });

    // Draw so the pile is empty
    assert_eq!(game.p_draw(uuid1).unwrap(), ());
    extract!(player, drawpile, discard);

    // Hand: BrokenStable, GinormousUnicorn
    // Stable: SadisticRitual
    // Drawpile:
    // Discard: Basic(Dancer)

    assert_eq_ids!(player.hand().ids(), vec!{ BrokenStable, GinormousUnicorn });
    assert_eq_ids!(player.stable().ids(), vec!{ SadisticRitual });
    assert_eq_ids!(drawpile.ids(), vec!{ });
    assert_eq_ids!(discard.ids(), vec!{ Basic });

    // Play a new unicorn
    assert_eq!(game.p_play_id(uuid1, GinormousUnicorn).unwrap(), ());
    extract!(player, drawpile, discard);

    // Hand: BrokenStable
    // Stable: SadisticRitual, GinormousUnicorn
    // Drawpile:
    // Discard: Basic(Dancer)

    assert_eq_ids!(player.hand().ids(), vec!{ BrokenStable });
    assert_eq_ids!(player.stable().ids(), vec!{ SadisticRitual, GinormousUnicorn });
    assert_eq_ids!(drawpile.ids(), vec!{ });
    assert_eq_ids!(discard.ids(), vec!{ Basic });

    // Player can sacrifice unicorn but can't draw a card so don't do anything
    assert_eq!(game.bot().unwrap(), ());
    extract!(player, drawpile, discard);

    // Hand: BrokenStable
    // Stable: SadisticRitual, GinormousUnicorn
    // Drawpile:
    // Discard: Basic(Dancer)

    assert_eq_ids!(player.hand().ids(), vec!{ BrokenStable });
    assert_eq_ids!(player.stable().ids(), vec!{ SadisticRitual, GinormousUnicorn });
    assert_eq_ids!(drawpile.ids(), vec!{ });
    assert_eq_ids!(discard.ids(), vec!{ Basic });
}



#[test]
pub fn test_barbed_wire() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    // After every time we call a function on the game that takes a mutable refernce,
    // our read-only pointers become invalid. To ease debugging, I've made a helper
    // macro that refreshes the pointers again. Because of the nature of Rust's
    // hygenic macros, we have to specify the names every time.
    // This macro is thus used as:
    //    extract!(player, hand, stable, drawpile, discard);
    // Disclaimer: No liability is given when the parameters are in the wrong order
    macro_rules! extract {
        ($player:ident, $hand:ident, $stable:ident, $drawpile:ident, $discard:ident) => {
            let $player = game.p(uuid1);
            let $hand = $player.hand();
            let $stable = $player.stable();
            let $drawpile = &game.table.drawpile;
            let $discard = &game.table.discard;
        };
    }


    let hand = game.p_mut(uuid1).hand_mut();
    hand.add_card(card!(RainbowAura));
    hand.add_card(card!(GinormousUnicorn));
    hand.add_card(card!(Slowdown));
    hand.add_card(card!(BarbedWire));
    hand.add_card(card!(RainbowAura));
    hand.add_card(basic!(PumpkinSpice));

    // PumpkinSpice
    // No effects currently apply
    println!("~~~ play Basic");
    assert_eq!(game.p_play_id(uuid1, Basic).unwrap(), ());
    extract!(player, hand, stable, _drawpile, discard);

    assert_eq_ids!(hand.ids(), vec!{ RainbowAura, GinormousUnicorn, Slowdown, BarbedWire, RainbowAura });
    assert_eq_ids!(stable.ids(), vec! { Basic });
    assert_eq_ids!(discard.ids(), vec! { });

    // BarbedWire:
    // Each time a Unicorn card enters or leaves your Stable, DISCARD a card
    println!("~~~ play BarbedWire");
    assert_eq!(game.p_play_id(uuid1, BarbedWire).unwrap(), ());
    extract!(player, hand, stable, _drawpile, discard);

    assert_eq_ids!(hand.ids(), vec!{ RainbowAura, GinormousUnicorn, Slowdown, RainbowAura });
    assert_eq_ids!(stable.ids(), vec!{ Basic, BarbedWire });
    assert_eq_ids!(discard.ids(), vec! { });

    // Play non-unicorn
    println!("~~~ play Slowdown");
    assert_eq!(game.p_play_id(uuid1, Slowdown).unwrap(), ());
    extract!(player, hand, stable, _drawpile, discard);

    assert_eq_ids!(hand.ids(), vec!{ RainbowAura, GinormousUnicorn, RainbowAura });
    assert_eq_ids!(stable.ids(), vec!{ Basic, BarbedWire, Slowdown });
    assert_eq_ids!(discard.ids(), vec! { });

    // Play unicorn: Enter stable
    println!("~~~ play GinormousUnicorn");
    assert_eq!(game.p_play_id(uuid1, GinormousUnicorn).unwrap(), ());
    extract!(player, hand, stable, _drawpile, discard);

    assert_eq_ids!(hand.ids(), vec!{ RainbowAura });
    assert_eq_ids!(stable.ids(), vec!{ Basic, Slowdown, BarbedWire, GinormousUnicorn });
    assert_eq_ids!(discard.ids(), vec! { RainbowAura });

    // Discard unicorn: Leave stable
    println!("~~~ sacrifice GinormousUnicorn");
    assert_eq!(game.p_sacrifice_id(uuid1, GinormousUnicorn).unwrap(), ());
    extract!(player, hand, stable, _drawpile, discard);

    assert_eq_ids!(hand.ids(), vec!{ });
    assert_eq_ids!(stable.ids(), vec!{ Basic, BarbedWire, Slowdown });
    assert_eq_ids!(discard.ids(), vec! { RainbowAura, GinormousUnicorn, RainbowAura });
}

#[test]
pub fn test_unicorn_phoenix() {
    use CardID::*;

    // Unicorn Phoenix
    // When this card enters your Stable, DISCARD a card.
    // If this card is sacrificed or destroyed,
    //    bring it directly back into your Stable
    //      if you have at least 1 card in your hand

    // When this card enters your Stable, DISCARD a card
    {
        let mut game = Game::new();
        let uuid1 = game.ez_new_player("Tester");

        let hand = game.p_mut(uuid1).hand_mut();
        hand.add_card(card!(UnicornPhoenix));
        hand.add_card(card!(BarbedWire));

        assert_eq!(game.p_play_id(uuid1, UnicornPhoenix).unwrap(), ());

        let player = game.p(uuid1);
        assert_eq_ids!(game.table.discard.ids(), vec!{ BarbedWire });
        assert_eq_ids!(player.hand().ids(), vec!{ });
        assert_eq_ids!(player.stable().ids(), vec!{ UnicornPhoenix });
    }

    // If this card is sacrificed, bring it directly back into your Stable if you have at least 1 card in your hand
    // (assuming you have at least 1 card in your hand)
    {
        let mut game = Game::new();
        let uuid1 = game.ez_new_player("Tester");

        let hand = game.p_mut(uuid1).hand_mut();
        hand.add_card(card!(UnicornPhoenix));
        hand.add_card(card!(RainbowAura));
        hand.add_card(card!(GinormousUnicorn));

        assert_eq!(game.p_play_id(uuid1, UnicornPhoenix).unwrap(), ());
        assert_eq!(game.table.discard.ids(), [GinormousUnicorn]);

        // Still RainbowAura left in hand
        assert_eq!(game.p_sacrifice_id(uuid1, UnicornPhoenix).unwrap(), ());

        // Directly back into stable, not to discard pile
        // However, since it's put back in the stable, you have to discard another card
        assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ UnicornPhoenix });
        assert_eq_ids!(game.table.discard.ids(), vec!{ GinormousUnicorn, RainbowAura });
    }

    // If this card is sacrificed, bring it directly back into your Stable if you have at least 1 card in your hand
    // (without cards in your hand)
    {
        let mut game = Game::new();
        let uuid1 = game.ez_new_player("Tester");

        let hand = game.p_mut(uuid1).hand_mut();
        hand.add_card(card!(UnicornPhoenix));
        hand.add_card(card!(GinormousUnicorn));

        assert_eq!(game.p_play_id(uuid1, UnicornPhoenix).unwrap(), ());
        assert_eq_ids!(game.table.discard.ids(), vec!{ GinormousUnicorn });

        // No cards left in hand
        assert_eq!(game.p_sacrifice_id(uuid1, UnicornPhoenix).unwrap(), ());

        // To discard pile, not back to stable
        assert!(game.p(uuid1).stable().is_empty());
        assert_eq_ids!(game.table.discard.ids(), vec!{ GinormousUnicorn, UnicornPhoenix });
    }
}

#[test]
pub fn test_tiny_stable() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    let stable = game.p_mut(uuid1).stable_mut();
    stable.add_card(basic!(PoppedCollars));
    stable.add_card(basic!(VinylRecords));
    stable.add_card(basic!(Dancer));

    let hand = game.p_mut(uuid1).hand_mut();
    hand.add_card(basic!(Emoji));
    hand.add_card(basic!(Beards));
    hand.add_card(basic!(Hashtags));
    hand.add_card(card!(TinyStable));

    assert_eq!(game.p_play_id(uuid1, TinyStable).unwrap(), ());
    assert_eq!(game.p_play_id(uuid1, Basic).unwrap(), ()); // Emoji, 4
    assert_eq_ids!(game.table.discard.ids(), vec!{});
    assert_eq!(game.p_play_id(uuid1, Basic).unwrap(), ()); // Hashtags, 5
    assert_eq_ids!(game.table.discard.ids(), vec!{});
    assert_eq!(game.p_play_id(uuid1, Basic).unwrap(), ()); // Beards, 6 => 5
    assert_eq_ids!(game.table.discard.ids(), vec!{ Basic });
}

#[test]
pub fn test_good_deal() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    game.p_mut(uuid1).hand_mut().add_card(card!(GoodDeal));

    for _ in 0..3 {
        game.table.drawpile.add_card(card!(BrokenStable));
    }

    assert_eq!(game.p_play_id(uuid1, GoodDeal).unwrap(), ());

    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ BrokenStable, BrokenStable });
    assert_eq_ids!(game.table.discard.ids(), vec!{ GoodDeal, BrokenStable });
}

#[test]
pub fn test_blatant_thievery() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");
    let uuid2 = game.ez_new_player("Tester");

    game.p_mut(uuid1).hand_mut().add_card(card!(BlatantThievery));
    game.p_mut(uuid2).hand_mut().add_card(card!(ShakeUp));

    // Player 1 steals from player 2
    assert_eq!(game.p_play_id(uuid1, BlatantThievery).unwrap(), ());

    assert_eq_ids!(game.p(uuid2).hand().ids(), vec!{ });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ ShakeUp });
    assert_eq_ids!(game.table.discard.ids(), vec!{ BlatantThievery });
}

#[test]
pub fn test_targeted_destruction() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    game.p_mut(uuid1).hand_mut().add_card(card!(TargetedDestruction));
    game.p_mut(uuid1).stable_mut().add_card(card!(Slowdown));

    assert_eq!(game.p_play_id(uuid1, TargetedDestruction).unwrap(), ());

    assert_eq_ids!(game.table.discard.ids(), vec!{ TargetedDestruction, Slowdown });
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ });
}


#[test]
pub fn test_unicorn_lasso() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");
    let uuid2 = game.ez_new_player("Tester");

    game.p_mut(uuid1).hand_mut().add_card(card!(UnicornLasso));
    game.p_mut(uuid2).stable_mut().add_card(basic!(PumpkinSpice));

    assert_eq!(game.p_play_id(uuid1, UnicornLasso).unwrap(), ());

    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ UnicornLasso });
    assert_eq_ids!(game.p(uuid2).stable().ids(), vec!{ Basic });

    assert_eq!(game.bot().unwrap(), ());

    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ UnicornLasso, Basic });
    assert_eq_ids!(game.p(uuid2).stable().ids(), vec!{ });

    assert_eq!(game.eot().unwrap(), ());

    assert_eq!(game.p(uuid1).stable().ids(), vec!{ UnicornLasso });
    assert_eq!(game.p(uuid2).stable().ids(), vec!{ Basic });
}

#[test]
pub fn test_seductive_unicorn() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");
    let uuid2 = game.ez_new_player("Tester");
    let uuid3 = game.ez_new_player("Tester");

    game.p_mut(uuid1).stable_mut().add_card(baby!(White));
    game.p_mut(uuid2).stable_mut().add_card(basic!(PumpkinSpice));

    let player1 = game.p_mut(uuid1);
    player1.hand_mut().add_card(card!(SeductiveUnicorn));

    assert_eq!(game.p_play_id(uuid1, SeductiveUnicorn).unwrap(), ());

    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ Baby, SeductiveUnicorn, Basic });
    assert_eq_ids!(game.p(uuid2).stable().ids(), vec!{ });

    // Make player 3 steal the seductive unicorn
    // This means that first the leave action of the SU is applied and player 2 gets its basic unicorn back,
    // but then player 3 has its enter action and steals the baby from player 1.
    assert_eq!(game.apply_actions(vec! {
        delta!(game.p(uuid1).stable().get_id_card(SeductiveUnicorn).unwrap().uuid(), [uuid1, Stable] => [uuid3, Stable]),
    }).unwrap(), ());

    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ });
    assert_eq_ids!(game.p(uuid2).stable().ids(), vec!{ Basic });
    assert_eq_ids!(game.p(uuid3).stable().ids(), vec!{ Baby, SeductiveUnicorn});
}

#[test]
pub fn test_unicorn_poison() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");
    let uuid2 = game.ez_new_player("Tester");

    game.p_mut(uuid1).hand_mut().add_card(card!(UnicornPoison));
    game.p_mut(uuid2).stable_mut().add_card(baby!(Yellow));

    assert_eq!(game.p_play_id(uuid1, UnicornPoison).unwrap(), ());
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ });
    assert_eq_ids!(game.p(uuid2).stable().ids(), vec!{ });
    assert_eq_ids!(game.table.discard.ids(), vec!{ UnicornPoison });
    assert_eq_ids!(game.table.nursery.ids(), vec!{ Baby });
}

#[test]
pub fn test_two_for_one() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");
    let uuid2 = game.ez_new_player("Tester");

    game.p_mut(uuid1).hand_mut().add_card(card!(TwoForOne));
    game.p_mut(uuid1).stable_mut().add_card(card!(TinyStable));
    game.p_mut(uuid2).stable_mut().add_card(card!(BarbedWire));
    game.p_mut(uuid2).stable_mut().add_card(card!(BrokenStable));

    assert_eq!(game.p_play_id(uuid1, TwoForOne).unwrap(), ());

    assert_eq_ids!(game.p(uuid2).stable().ids(), vec!{ });
    assert_eq_ids!(game.table.discard.ids(), vec!{ TwoForOne, TinyStable, BarbedWire, BrokenStable });
}

#[test]
pub fn test_glitter_bob() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");
    let uuid2 = game.ez_new_player("Tester");

    game.p_mut(uuid1).hand_mut().add_card(card!(GlitterBomb));
    game.p_mut(uuid1).stable_mut().add_card(card!(Slowdown));
    game.p_mut(uuid2).stable_mut().add_card(card!(GinormousUnicorn));

    assert_eq!(game.p_play_id(uuid1, GlitterBomb).unwrap(), ());

    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ Slowdown, GlitterBomb });
    assert_eq_ids!(game.p(uuid2).stable().ids(), vec!{ GinormousUnicorn });

    assert_eq!(game.bot().unwrap(), ());

    assert_eq_ids!(game.table.discard.ids(), vec!{ Slowdown, GinormousUnicorn });
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ GlitterBomb });
    assert_eq_ids!(game.p(uuid2).stable().ids(), vec!{ });
}

#[test]
pub fn test_summoning_ritual() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    game.p_mut(uuid1).hand_mut().add_card(card!(SummoningRitual));
    game.p_mut(uuid1).hand_mut().add_card(basic!(Beards));
    game.p_mut(uuid1).hand_mut().add_card(basic!(Emoji));
    game.p_mut(uuid1).hand_mut().add_card(basic!(PumpkinSpice));

    assert_eq!(game.p_play_id(uuid1, SummoningRitual).unwrap(), ());

    assert_eq_ids!(game.table.discard.ids(), vec!{  });
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ SummoningRitual });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ Basic, Basic, Basic });

    assert_eq!(game.bot().unwrap(), ());

    assert_eq_ids!(game.table.discard.ids(), vec!{ Basic });
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ SummoningRitual, Basic });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ Basic });

    assert_eq!(game.bot().unwrap(), ());

    assert_eq_ids!(game.table.discard.ids(), vec!{ Basic });
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ SummoningRitual, Basic });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ Basic });
}

#[test]
pub fn test_extra_tail() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    // We depend on the order of the drawpile for testing
    game.table.drawpile.auto_shuffle = false;

    game.p_mut(uuid1).hand_mut().add_card(card!(ExtraTail));

    // May not be played
    assert_eq!(game.p_play_id(uuid1, ExtraTail).unwrap_err(), "game#p_play: Player may not play card");
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ ExtraTail });

    // Now it may be played
    game.p_mut(uuid1).stable_mut().add_card(basic!(PumpkinSpice));

    assert_eq!(game.p_play_id(uuid1, ExtraTail).unwrap(), ());
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ Basic, ExtraTail });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{  });

    // Only the BOT edits the number of cards that may be drawn
    assert_eq!(game.bot().unwrap(), ());

    // User should draw the highest amount of cards allowed and possible (!)
    assert_eq!(game.draw().unwrap(), ());
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{  });

    // Make sure there is a cards to draw
    game.table.drawpile.add_card(card!(TinyStable));

    // User should draw the highest amount of cards allowed and possible (!)
    assert_eq!(game.draw().unwrap(), ());
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ TinyStable });

    // Make sure there is a cards to draw
    game.table.drawpile.add_card(card!(TwoForOne)); // (1)
    game.table.drawpile.add_card(card!(UnicornLasso)); // (2)
    game.table.drawpile.add_card(card!(BarbedWire)); // (3)

    // User should draw the highest amount of cards allowed and possible (!)
    assert_eq!(game.draw().unwrap(), ());
    // Keep in mind that first (1) and (3) are swapped, then (1) is drawn, then (3) and (2) are swapped and (3) is drawn
    // because of the way we do `swap_remove` from the drawpile
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ TinyStable, TwoForOne, BarbedWire });
}

#[test]
pub fn test_rainbow_mane() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    game.p_mut(uuid1).hand_mut().add_card(card!(RainbowMane));
    game.p_mut(uuid1).hand_mut().add_card(card!(Slowdown));
    game.table.drawpile.add_card(basic!(PoppedCollars));

    // May not be played
    assert_eq!(game.p_play_id(uuid1, RainbowMane).unwrap_err(), "game#p_play: Player may not play card");
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ RainbowMane, Slowdown });

    // Now it may be played
    game.p_mut(uuid1).stable_mut().add_card(basic!(PumpkinSpice));

    assert_eq!(game.p_play_id(uuid1, RainbowMane).unwrap(), ());
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ Basic, RainbowMane });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ Slowdown });

    // If the player doesn't have any basic unicorns in their hand, don't do anything
    assert_eq!(game.bot().unwrap(), ());
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ Basic, RainbowMane });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ Slowdown });

    // Make the player draw a basic unicorn
    assert_eq!(game.p_draw(uuid1).unwrap(), ());
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ Slowdown, Basic });

    // Now the player has a basic unicorn to play
    assert_eq!(game.bot().unwrap(), ());
    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ Basic, RainbowMane, Basic });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ Slowdown });
}


#[test]
pub fn test_reset_button() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");
    let uuid2 = game.ez_new_player("Tester");

    game.p_mut(uuid1).hand_mut().add_card(card!(ResetButton)); // card to play -> drawpile
    game.p_mut(uuid1).hand_mut().add_card(card!(Slowdown)); // downgrade in hand - ignore
    game.p_mut(uuid1).stable_mut().add_card(card!(BarbedWire)); // downgrade in stable -> drawpile
    game.p_mut(uuid1).stable_mut().add_card(card!(ExtraTail)); // upgrade in stable -> drawpile
    game.p_mut(uuid1).stable_mut().add_card(basic!(PumpkinSpice)); // unicorn in stable - ignore
    game.p_mut(uuid2).stable_mut().add_card(card!(TinyStable)); // downgrade in other stable -> drawpile
    game.table.discard.add_card(card!(GinormousUnicorn)); // any card in discard -> drawpile

    // Play it
    assert_eq!(game.p_play_id(uuid1, ResetButton).unwrap(), ());

    assert_eq_ids!(game.p(uuid1).stable().ids(), vec!{ Basic });
    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ Slowdown });
    assert_eq_ids!(game.table.discard.ids(), vec!{ });
    assert_eq_ids!(game.table.drawpile.ids(), vec!{ ResetButton, BarbedWire, ExtraTail, TinyStable, GinormousUnicorn });
}


#[test]
pub fn tests_unfair_bargain() {
    use CardID::*;

    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");
    let uuid2 = game.ez_new_player("Tester");

    game.p_mut(uuid1).hand_mut().add_card(card!(UnfairBargain));
    game.p_mut(uuid1).hand_mut().add_card(card!(TinyStable));
    game.p_mut(uuid2).hand_mut().add_card(card!(RainbowMane));

    assert_eq!(game.p_play_id(uuid1, UnfairBargain).unwrap(), ());

    assert_eq_ids!(game.p(uuid1).hand().ids(), vec!{ RainbowMane });
    assert_eq_ids!(game.p(uuid2).hand().ids(), vec!{ TinyStable });
    assert_eq_ids!(game.table.discard.ids(), vec!{ UnfairBargain });
}