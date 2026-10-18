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
//! A board's other devices beside the touchpad: a GPIO expander at 0x20,
//! whose interrupt latch a read clears, and a thermal sensor at 0x4C, whose
//! configuration register an HID register pointer would write. The sweep
//! tries only 0x15, 0x2C and 0x10, so wherever the pad is, and when none
//! answers at all, neither is addressed, nor read, nor put in reach.

use super::bystanders::{board, SENSOR};
use super::client::{transfer_errno, EXPANDER};
use super::declared::{here, pad_at, reached, this};
use crate::bench::{driver, unaddressed, GEMINI_LAKE_HZ};
use crate::protocol::E_INVAL;
use crate::setup::settle::settle;
use crate::transaction::Presence;

#[test]
fn with_the_pad_at_0x10_or_0x2c_the_expander_and_sensor_are_never_addressed() {
    for pad in [0x10, 0x2C] {
        let b = board(Some(pad_at(pad)));
        let d = driver(GEMINI_LAKE_HZ, 0).expect("controller bring-up");
        let tpd0 = here(0);
        let s = settle(&d, &this(), &tpd0, Presence::Absent, &[tpd0]);
        assert_eq!((s.addr, reached(&s.reach)), (pad, vec![pad]), "pad at {pad:#04x}");
        b.assert_untouched();
    }
}

#[test]
fn with_no_pad_answering_the_sweep_binds_neither_and_never_addresses_them() {
    let b = board(None);
    let d = driver(GEMINI_LAKE_HZ, 0).expect("controller bring-up");
    let tpd0 = here(0);
    let s = settle(&d, &this(), &tpd0, Presence::Absent, &[tpd0]);
    assert_eq!((s.addr, reached(&s.reach), s.by_probe), (0, vec![], false));
    b.assert_untouched();
}

#[test]
fn a_client_transfer_to_either_is_refused_before_the_bus() {
    for pad in [None, Some(pad_at(0x2C))] {
        let b = board(pad);
        let d = unaddressed(GEMINI_LAKE_HZ).expect("controller bring-up");
        // Port 0's configuration register, every line an output.
        assert_eq!(transfer_errno(&d, EXPANDER, &[0x06, 0x00], 0), E_INVAL);
        // The sensor's configuration register, as i2c_hid points at 0x0001.
        assert_eq!(transfer_errno(&d, SENSOR, &[0x01, 0x00], 30), E_INVAL);
        b.assert_untouched();
    }
}
