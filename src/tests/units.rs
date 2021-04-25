use crate::*;

#[test]
pub fn test_count_unicorns_in_stable() {
    let mut stable = Stable::new();

    stable.add_card(basic!(Narwhal));
    stable.add_card(basic!(Narwhal));  // Not actually twice in the game but alright for testing
	stable.add_card(baby!(White));
	stable.add_card(baby!(Skeleton));
    stable.add_card(baby!(Narwhal));
    stable.add_card(card!(GinormousUnicorn));  // Counts for 2 unicorns

    // Add non-unicorns just to be sure
    stable.add_card(card!(BrokenStable));
    stable.add_card(card!(RainbowAura));
    stable.add_card(card!(Slowdown));

    assert_eq!(stable.count_unicorns(), 7);

    // Should still be able to add more cards
    stable.add_card(card!(BrokenStable));
}

#[test]
pub fn test_may_play() {
    let mut game = Game::new();
    let uuid1 = game.ez_new_player("Tester");

    let brokenstable = card!(BrokenStable);
    assert!(game.p_may_play(uuid1, &*brokenstable).unwrap());
    game.p_mut(uuid1).stable_mut().add_card(brokenstable);

    // Broken Stable prevents Upgrades (like Rainbow Aura) from being played
    assert!(!game.p_may_play(uuid1, &*card!(RainbowAura)).unwrap());
}

#[test]
pub fn test_stable() {
    let mut stable = Stable::new();

    // Test count and adding a card
    assert_eq!(stable.len(), 0);
    stable.add_card(card!(RainbowAura));
    assert_eq!(stable.len(), 1);

    // Test getting IDs
    let ids = stable.ids();
    assert_eq!(ids, [CardID::RainbowAura]);
    let id: Option<&CardID> = ids.get(0);
    assert_eq!(id, Some(&CardID::RainbowAura));

    // Test getting UUIDs
    let uuids = stable.uuids();
    let uuid: Option<&u64> = uuids.get(0);
    assert!(uuid.is_some());
    let uuid = *uuid.unwrap();

    // Test getting a reference to a card without stealing it yet
    let boxcard: Option<&Box<dyn Card>> = stable.get_card(uuid);
    assert!(boxcard.is_some());
    let boxcard: &Box<dyn Card> = boxcard.unwrap();
    assert_eq!(boxcard.id(), CardID::RainbowAura);
    assert_eq!(stable.len(), 1);

    // Test taking the card from the stable
    let card: Option<Box<dyn Card>> = stable.take_card(uuid);
    assert!(card.is_some());
    let card: Box<dyn Card> = card.unwrap();
    assert_eq!(card.id(), CardID::RainbowAura);
    assert_eq!(stable.len(), 0);

    // Test adding the card back again
    stable.add_card(card);
    assert_eq!(stable.len(), 1);
    assert_eq!(stable.ids(), [CardID::RainbowAura]);
}

#[test]
pub fn test_next_card() {
    use CardID::*;

    let mut stable = Stable::new();
    stable.add_card(basic!(Narwhal));
    stable.add_card(card!(NarwhalTorpedo));

    for _ in 0..2 {
        stable.reset_cards();


        let next = stable.next_card();
        assert!(next.is_some());
        let next = next.unwrap();
        assert!(next.id() == Basic);
        stable.readd_card(next); // push back


        let next = stable.next_card();
        assert!(next.is_some());
        let next = next.unwrap();
        assert!(next.id() == NarwhalTorpedo);
        stable.readd_card(next); // push back


        let next = stable.next_card();
        assert!(next.is_none());
    }
}

#[test]
pub fn test_uuid() {
    let card1: Box<dyn Card> = card!(RainbowAura);
    let card2: Box<dyn Card> = card!(RainbowAura);

    // The card IDs should be the same
    assert_eq!(card1.id(), card2.id());

    // But the hashes shouldn't
    assert_ne!(card1.uuid(), card2.uuid());
}