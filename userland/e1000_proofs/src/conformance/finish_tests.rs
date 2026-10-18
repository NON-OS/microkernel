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

//! The end of one bring-up attempt: a part that will not come up leaves
//! nothing claimed behind it, so the next attempt can claim it again.

use nonos_libc::{entropy, given_back};

use super::live::live_part;
use super::memory::Memory;
use super::model::window;
use crate::init::finish;

/// The claim `Memory::driver` records.
const DEVICE: u64 = 1;

/// What this attempt released: the claim alone, which tears every grant
/// down after the broker has turned Bus Master off. A frame unmapped by hand
/// before it would be freed while the part could still write it.
const EVERYTHING: [(&str, u64); 1] = [("device_release", DEVICE)];

fn released_since(before: usize) -> Vec<(&'static str, u64)> {
    given_back()[before..].to_vec()
}

#[test]
fn a_part_that_cannot_get_an_address_gives_back_every_grant_and_the_claim() {
    let _turn = entropy(false);
    let bar = window();
    let _part = live_part(&bar);
    let mut mem = Memory::new();
    let before = given_back().len();
    let err = finish(mem.driver(&bar)).err();
    assert_eq!(err, Some("no entropy for station address"));
    assert_eq!(released_since(before), EVERYTHING);
}

#[test]
fn a_part_that_never_leaves_reset_gives_back_every_grant_and_the_claim() {
    let bar = window();
    let mut mem = Memory::new();
    let before = given_back().len();
    let err = finish(mem.driver(&bar)).err();
    assert_eq!(err, Some("CTRL.RST did not self-clear"));
    assert_eq!(released_since(before), EVERYTHING);
}

#[test]
fn a_part_that_comes_up_keeps_every_grant() {
    let _turn = entropy(true);
    let bar = window();
    let _part = live_part(&bar);
    let mut mem = Memory::new();
    let before = given_back().len();
    let driver = finish(mem.driver(&bar)).expect("brought up");
    assert!(released_since(before).is_empty(), "a working attempt releases nothing");
    assert_eq!(driver.device_id, DEVICE);
}
