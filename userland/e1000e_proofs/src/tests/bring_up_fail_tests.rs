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

//! Bring-up attempts that fail: every grant goes back and the step is named.

use crate::constants::regs::*;
use crate::constants::rxtx::RCTL_EN;
use crate::constants::Family;
use crate::init::finish::finish;
use crate::model::live::live;
use crate::model::memory::Memory;
use crate::model::part::Behaviour;
use crate::model::window::{i82574, phy_at};
use nonos_libc::{entropy, given_back};
use std::sync::atomic::Ordering;

/// The setup grants in reverse, then the claim.
const EVERYTHING: [(&str, u64); 6] = [
    ("dma_unmap", 24),
    ("dma_unmap", 23),
    ("dma_unmap", 22),
    ("dma_unmap", 21),
    ("mmio_unmap", 20),
    ("device_release", 9),
];

#[test]
fn without_entropy_every_grant_goes_back_and_the_step_is_named() {
    let _turn = entropy(false);
    let bar = i82574();
    let _part = live(&bar, phy_at(1), Behaviour::default());
    let mut mem = Memory::new();
    let before = given_back().len();
    let err = finish(mem.driver(&bar, Family::I82574, 0x10D3)).err();
    assert_eq!(err, Some("no entropy for station address"));
    assert_eq!(given_back()[before..], EVERYTHING);
    assert_eq!(bar.wrote32(REG_RCTL) & RCTL_EN, 0, "never on the air");
}

#[test]
fn a_reset_that_never_clears_gives_every_grant_back() {
    let bar = i82574();
    let how = Behaviour::default();
    how.reset_stuck.store(true, Ordering::SeqCst);
    let _part = live(&bar, phy_at(1), how);
    let mut mem = Memory::new();
    let before = given_back().len();
    let err = finish(mem.driver(&bar, Family::I82574, 0x10D3)).err();
    assert_eq!(err, Some("reset did not clear in 50 ms"));
    assert_eq!(given_back()[before..], EVERYTHING);
}
