// NONOS Operating System (AGPL-3.0-or-later)
//! Which exit net.socks5 tries next. Rotation used to take the next place in
//! the directory whatever the session had seen there; it now skips exits
//! that went silent on this session, falls back to one that delivered, and
//! keeps what it remembers bounded.

use crate::choose::{Named, Record, LOOK_MAX, PROVEN_MAX, SILENT_MAX};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct E(u8);

impl Named for E {
    fn id(&self) -> [u8; 32] {
        [self.0; 32]
    }
}

/// A directory of `n` exits, place k holding exit k, wrapping.
fn dir(n: u8) -> impl FnMut(u32) -> Option<E> {
    move |place| Some(E((place % u32::from(n)) as u8))
}

#[test]
fn a_fresh_session_takes_the_place_it_is_at() {
    let r: Record<E> = Record::new();
    assert_eq!(r.pick(3, dir(10)), Some((E(3), 3)));
}

#[test]
fn exits_silent_this_session_are_skipped() {
    let mut r = Record::new();
    r.silent(&E(3));
    r.silent(&E(4));
    assert_eq!(r.pick(3, dir(10)), Some((E(5), 5)));
}

#[test]
fn with_every_one_looked_at_silent_an_exit_that_delivered_is_preferred() {
    let mut r = Record::new();
    r.proven(E(42));
    for k in 0..LOOK_MAX as u8 {
        r.silent(&E(k));
    }
    assert_eq!(r.pick(0, dir(LOOK_MAX as u8)), Some((E(42), 0)));
}

#[test]
fn with_nothing_proven_the_one_silent_longest_ago_is_tried_again() {
    let mut r = Record::new();
    for k in [2u8, 0, 1] {
        r.silent(&E(k));
    }
    assert_eq!(r.pick(0, dir(3)), Some((E(2), 2)));
}

#[test]
fn a_delivery_clears_an_exit_s_silence() {
    let mut r = Record::new();
    r.silent(&E(1));
    r.proven(E(1));
    assert!(!r.is_silent(&E(1)));
    assert_eq!(r.pick(1, dir(5)), Some((E(1), 1)));
}

#[test]
fn what_is_remembered_is_bounded() {
    let mut r = Record::new();
    for k in 0..=(SILENT_MAX as u8) {
        r.silent(&E(k));
    }
    assert!(!r.is_silent(&E(0)), "the oldest silent exit is forgotten past SILENT_MAX");
    assert!(r.is_silent(&E(SILENT_MAX as u8)));
    for k in 100..(100 + PROVEN_MAX as u8 + 3) {
        r.proven(E(k));
    }
    for k in 0..LOOK_MAX as u8 {
        r.silent(&E(k + 200));
    }
    let picked = r.pick(200, |p| Some(E((p % 256) as u8)));
    assert_eq!(picked.map(|p| p.0), Some(E(100 + PROVEN_MAX as u8 + 2)), "the latest proven");
}

#[test]
fn an_empty_directory_and_list_give_nothing() {
    let r: Record<E> = Record::new();
    assert_eq!(r.pick(0, |_| None), None);
}
