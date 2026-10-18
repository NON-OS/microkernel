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

//! The registers the worst case counts (worst_case_tests.rs) are every one
//! a card's bring-up reaches through HAIMR, counted on the modelled reader:
//! on the RTS522A, which has the most, with a version 1.x card, whose CMD8
//! fails and has the engines stopped and the SD error cleared after it.

use nonos_libc::set_devices;

use super::rig::rts5227;
use super::worst_case_tests::REGISTERS;
use crate::card::bring_up;
use crate::init::init_hw;
use crate::model::{with, Card};
use crate::setup::{find, run};

/// The registers reached bringing a version 1.x card up on reader `device`.
fn registers(device: u16) -> u64 {
    let mut old = Card::sdsc(4095, 1);
    old.v2 = false;
    with(|r| r.insert(old));
    let mut reader = rts5227();
    reader.device = device;
    set_devices(vec![reader]);
    let mut drv = run(&find().expect("listed")).expect("the reader is taken");
    init_hw(&mut drv).expect("the chip comes up");
    with(|r| r.accesses = 0);
    bring_up(&mut drv).expect("the card comes up");
    u64::from(with(|r| r.accesses))
}

#[test]
fn every_register_a_version_1_card_reaches_on_the_rts522a_is_summed() {
    assert_eq!(registers(0x522A), REGISTERS);
}

#[test]
fn the_rts5227_reaches_no_more_than_the_sum_counts() {
    let reached = registers(0x5227);
    assert!(reached <= REGISTERS, "{reached} reached, {REGISTERS} summed");
}
