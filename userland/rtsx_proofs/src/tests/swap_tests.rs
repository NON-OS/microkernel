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

//! What the slot's latch and a mute card do to the medium: a swap between
//! two looks ends service, so does a card that cannot be identified where
//! the served one was, and a latch with the same card in the slot only
//! has it identified again.

use super::medium_tests::read;
use super::rig::{sdhc, serving};
use crate::model::{with, SD_INT};
use crate::protocol::*;

#[test]
fn a_swap_between_two_looks_is_caught_by_the_latch() {
    let (mut drv, mut medium) = serving(sdhc(1));
    with(|r| r.pull().map(|_| r.insert(sdhc(2))));
    assert_eq!(read(&mut drv, &mut medium), E_NODEV);
}

#[test]
fn a_card_that_cannot_be_identified_where_the_served_one_was_ends_service() {
    let (mut drv, mut medium) = serving(sdhc(1));
    let mut mute = with(|r| r.pull()).expect("a card");
    mute.busy_polls = u32::MAX;
    with(|r| r.insert(mute));
    assert_eq!(read(&mut drv, &mut medium), E_NODEV);
}

#[test]
fn a_latch_with_the_same_card_in_the_slot_keeps_it_served() {
    let (mut drv, mut medium) = serving(sdhc(1));
    with(|r| (r.pending |= SD_INT, r.sent.clear()));
    assert_eq!(read(&mut drv, &mut medium), E_OK);
    with(|r| assert!(r.sent.starts_with(&[0, 8]) && r.sent.contains(&2), "{:?}", r.sent));
}
