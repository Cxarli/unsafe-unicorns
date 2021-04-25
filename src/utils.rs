use crate::{Card, CardType};

use rand::Rng;

/// Make a user choose n items from a list
pub fn user_choose_n<'a, T>(_question: &'static str, items: &'a Vec<T>, n: usize) -> Vec<&'a T> {
    assert!(items.len() >= n);

    // @TODO: Make user actually able to choose
    (0..n).into_iter().map(|i| &items[i]).collect()
}

/// Make a user choose n items from a list and copy value
pub fn user_choose_n_copy<T: Copy>(_question: &'static str, items: &Vec<T>, n: usize) -> Vec<T> {
    assert!(items.len() >= n);

    // @TODO: Make user actually able to choose
    (0..n).into_iter().map(|i| items[i]).collect()
}

/// Make a user choose from a list of items
pub fn user_choose<'a, T>(_question: &'static str, items: &'a Vec<T>) -> &'a T {
    assert!(!items.is_empty());

    // @TODO: Make user actually able to choose
    &items[0]
}

/// Make a user choose from a yes/no question
pub fn user_choose_bool(question: &'static str) -> bool {
    *user_choose(question, &vec!{true, false})
}

static mut NEXT_UUID: u64 = 0;

/// Generate a random uuid
/// WILL ALWAYS BE >= 1
pub fn uuid() -> u64 {
    // To prevent the "unused rand::Rng" message
    if unsafe { NEXT_UUID } > 10000 {
        ::rand::thread_rng().gen::<u64>()
    } else {
        unsafe {
            NEXT_UUID += 1;
            NEXT_UUID
        }
    }
}

/// Check if a card is a unicorn
pub fn is_unicorn(card: &dyn Card) -> bool {
    match card.cardtype() {
        CardType::Unicorn(_) => true,
        _ => false
    }
}