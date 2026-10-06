// NONOS Operating System (AGPL-3.0-or-later)
/*
 * A guard given up on is not drawn again. On the old manager the redraw
 * avoided nothing, so `a_given_up_guard_is_never_redrawn` would have drawn
 * the dead guard back at the weight that chose it first.
 */

use super::path_pool::{flat, pool, relay};
use crate::path::guard_pick::{draw_guard, give_up, GIVEN_UP_MAX};
use crate::path::Relay;

/// Every roll a 64-bit draw can land on, sampled across the range.
fn rolls() -> impl Iterator<Item = u64> {
    (0..4096u64).map(|i| i.wrapping_mul(0x9E37_79B9_7F4A_7C15))
}

#[test]
fn a_given_up_guard_is_never_redrawn() {
    let relays = pool();
    let dead = relays[3].clone();
    let mut given_up = Vec::new();
    give_up(&mut given_up, &dead);
    for roll in rolls() {
        let got = draw_guard(&relays, &flat(), &mut given_up, roll).expect("a guard");
        assert_ne!(got.rsa_identity, dead.rsa_identity, "roll {roll:#x}");
        assert_ne!(got.address[..2], dead.address[..2], "its /16, roll {roll:#x}");
        assert_eq!(given_up.len(), 1, "the list holds while guards remain");
    }
}

#[test]
fn two_dead_guards_do_not_take_turns() {
    let mut relays: Vec<Relay> = vec![relay([10, 1, 0, 1], 1), relay([10, 2, 0, 1], 2), relay([10, 3, 0, 1], 3)];
    relays[0].weight = 1_000_000;
    relays[1].weight = 1_000_000;
    let mut given_up = Vec::new();
    give_up(&mut given_up, &relays[0]);
    give_up(&mut given_up, &relays[1]);
    for roll in rolls() {
        let got = draw_guard(&relays, &flat(), &mut given_up, roll).expect("the third");
        assert_eq!(got.rsa_identity, [3; 20], "the heavy dead guards stay out");
    }
}

#[test]
fn when_every_guard_failed_the_draw_starts_over() {
    let relays = vec![relay([10, 1, 0, 1], 1), relay([10, 2, 0, 1], 2)];
    let mut given_up = Vec::new();
    give_up(&mut given_up, &relays[0]);
    give_up(&mut given_up, &relays[1]);
    let got = draw_guard(&relays, &flat(), &mut given_up, 7);
    assert!(got.is_some(), "no guard left is not a reason to stop for the boot");
    assert!(given_up.is_empty(), "the list said nothing about these guards any more");
}

#[test]
fn an_empty_pool_draws_nothing_and_keeps_its_list() {
    let mut given_up = Vec::new();
    assert!(draw_guard(&[], &flat(), &mut given_up, 1).is_none());
    give_up(&mut given_up, &relay([10, 1, 0, 1], 1));
    assert!(draw_guard(&[], &flat(), &mut given_up, 1).is_none());
}

#[test]
fn the_list_forgets_its_oldest_past_the_cap() {
    let mut given_up = Vec::new();
    for i in 0..(GIVEN_UP_MAX as u8 + 3) {
        give_up(&mut given_up, &relay([10, i, 0, 1], i + 1));
    }
    assert_eq!(given_up.len(), GIVEN_UP_MAX);
    assert_eq!(given_up[0].identity, [4; 20], "the first three are forgotten");
    assert_eq!(given_up[GIVEN_UP_MAX - 1].identity, [GIVEN_UP_MAX as u8 + 3; 20]);
}

/// A link that breaks soon after it opened counts against the guard; one
/// that served a while does not. Three young breaks reach the same limit as
/// three refused dials (manager/guard.rs, GUARD_ATTEMPTS).
#[test]
fn only_a_young_link_counts_against_the_guard() {
    use crate::path::guard_pick::{died_young, LINK_YOUNG_SECONDS};
    let opened = 1_000;
    assert!(died_young(opened, opened));
    assert!(died_young(opened, opened + LINK_YOUNG_SECONDS - 1));
    assert!(!died_young(opened, opened + LINK_YOUNG_SECONDS));
    assert!(!died_young(opened, opened + 3_600), "an hour of service is not the guard failing");
    assert!(!died_young(u64::MAX - 5, u64::MAX), "no overflow at the top of the clock");
    assert!(died_young(u64::MAX - 5, u64::MAX - 1));
}
