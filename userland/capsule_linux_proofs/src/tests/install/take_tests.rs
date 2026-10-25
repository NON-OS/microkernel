// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Taking a market program out: its box goes only once it cannot start, and
//! a store that cannot say what is gone keeps it.

use alloc::vec::Vec;

use crate::install::market::minted::{beside, POLICY, POLICY_TRAILER, TRAILER};
use crate::install::market::take::take_out;

const AT: &[u8] = b"/usr/bin/age";

fn all_four() -> Vec<Vec<u8>> {
    [AT.to_vec(), beside(AT, TRAILER), beside(AT, POLICY), beside(AT, POLICY_TRAILER)].into()
}

/// Take AT out of a store holding it and all four beside it; `stays` would not go.
fn taken(stays: &[Vec<u8>]) -> (bool, Vec<Vec<u8>>) {
    let store = core::cell::RefCell::new(all_four());
    let mut unlink = |at: &[u8]| {
        if !stays.iter().any(|s| s == at) {
            store.borrow_mut().retain(|k| k != at);
        }
    };
    let mut found = |at: &[u8]| Ok::<bool, ()>(store.borrow().iter().any(|k| k == at));
    let file = take_out(AT, &mut unlink, &mut found);
    (file, store.into_inner())
}

#[test]
fn a_startable_program_that_will_not_go_keeps_its_box() {
    let stays = [AT.to_vec(), beside(AT, TRAILER)];
    let (file, left) = taken(&stays);
    assert!(!file);
    assert!(left.contains(&beside(AT, POLICY)) && left.contains(&beside(AT, POLICY_TRAILER)));
}

#[test]
fn once_the_program_cannot_start_everything_beside_it_goes() {
    for stays in [vec![], vec![AT.to_vec()], vec![beside(AT, TRAILER)]] {
        let (file, left) = taken(&stays);
        assert_eq!(file, !stays.contains(&AT.to_vec()));
        assert_eq!(left, stays, "only what would not go is left");
    }
}

/* A failing store answers neither the unlink nor the look: nothing reads as
 * gone, so the box stays beside a program that may still be there. */
#[test]
fn a_store_that_cannot_say_keeps_the_box() {
    let mut asked = Vec::new();
    let mut unlink = |at: &[u8]| asked.push(at.to_vec());
    let mut found = |_: &[u8]| Err::<bool, ()>(());
    assert!(!take_out(AT, &mut unlink, &mut found));
    assert!(!asked.contains(&beside(AT, POLICY)), "its policy is never even asked to go");
}
