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

//! No blind scan: the driver addresses only the device the controller
//! named, and a bus with no pad is asked a bounded number of times.

use i2c_transfer_proofs::protocol::OP_TRANSFER;
use nonos_i2cmodel::touchpad::report_descriptor;
use nonos_i2cmodel::HidOverI2c;

use super::fixture::MAX_INPUT;
use super::recorder::recorded;
use crate::setup;

/// A pad at an address neither the firmware names nor the sweep tries.
fn stray() -> HidOverI2c {
    HidOverI2c::new(0x3A, report_descriptor(), MAX_INPUT, 0x04F3, 0x3028)
}

#[test]
fn with_no_address_from_the_controller_no_transfer_is_asked_for() {
    let r = recorded(stray(), None, None);
    let state = setup::run().expect("setup");
    assert!(!state.found());
    let calls = r.calls.borrow();
    let asked: Vec<_> = calls.iter().filter(|c| c.0 == OP_TRANSFER).collect();
    assert!(asked.is_empty(), "transfers to addresses nobody named: {asked:?}");
}

#[test]
fn a_bus_with_no_pad_is_never_asked_for_an_address_in_the_whole_window() {
    let r = recorded(stray(), None, None);
    let mut state = setup::run().expect("setup");
    (0..200).for_each(|_| setup::reprobe(&mut state));
    nonos_libc::mk_idle_ms(30_000);
    (0..20).for_each(|_| setup::reprobe(&mut state));
    assert!(state.done, "the driver did not give up on a bus with no pad");
    let asked = r.calls.borrow().iter().filter(|c| c.0 == OP_TRANSFER).count();
    assert_eq!(asked, 0, "a transfer was asked for on a bus with no pad");
}

#[test]
fn a_bus_with_no_pad_is_asked_again_every_half_second() {
    let r = recorded(stray(), None, None);
    let mut state = setup::run().expect("setup");
    let before = r.calls.borrow().len();
    (1..=2_000).for_each(|now_ms| crate::drive::pass(&mut state, now_ms));
    let asked = r.calls.borrow().len() - before;
    assert!((3..=5).contains(&asked), "asked {asked} times in two seconds");
}
