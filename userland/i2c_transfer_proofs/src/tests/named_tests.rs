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

//! Which pad the shipping OP_ACPI_HID handler names. Multi-SKU firmware
//! declares every pad a chassis may carry; with the fitted one asleep at
//! setup's probe, all of them stay in reach. Each ask names the next pad in
//! reach, the bound one first, until a client has read one's HID
//! descriptor; from then on, that pad alone.

use super::acpi::named_once;
use super::client::transfer_errno;
use super::multi_sku::{bound, named, FITTED, FITTED_REG, PADS};
use crate::protocol::E_OK;
use crate::transaction::Presence;

/// Where the HID driver's blind scan looks (capsule_driver_i2c_hid
/// hid/probe/scan.rs, as of beeee464c).
const SCAN: [u8; 8] = [0x10, 0x15, 0x2C, 0x38, 0x4B, 0x4C, 0x20, 0x24];
const SCAN_REGS: [u16; 2] = [0x0001, 0x0020];

#[test]
fn of_three_pads_declared_the_fitted_second_is_named_on_the_second_ask() {
    for found in [Presence::Absent, Presence::Acked] {
        let (d, _a) = bound(found);
        assert_eq!(named(&d, 3), PADS, "each pad in reach in turn, the bound one first");
    }
}

#[test]
fn the_hid_drivers_reprobe_finds_a_pad_the_blind_scan_cannot_and_keeps_it() {
    let (d, _a) = bound(Presence::Absent);
    for (addr, reg) in SCAN.into_iter().flat_map(|a| SCAN_REGS.map(|r| (a, r))) {
        assert_ne!(transfer_errno(&d, addr, &reg.to_le_bytes(), 30), E_OK, "{addr:#x}");
    }
    // driver.i2c_hid0's reprobe: one ask, then the HID descriptor read at
    // the address and register it was named.
    let mut probes = 0;
    let found = loop {
        probes += 1;
        let (addr, reg, ..) = named_once(&d).expect("a pad is named");
        if transfer_errno(&d, addr, &reg.to_le_bytes(), 30) == E_OK {
            break (addr, reg);
        }
        assert!(probes < PADS.len(), "no ask named the fitted pad");
    };
    assert_eq!((found, probes), ((FITTED, FITTED_REG), 2));
    assert_eq!(named(&d, 7), [FITTED; 7], "once bound, only the bound pad is named");
}

#[test]
fn a_pad_that_returned_its_descriptor_to_setup_is_the_only_one_named() {
    let (d, _a) = bound(Presence::HidDescriptor(FITTED_REG));
    assert_eq!(named(&d, 5), [FITTED; 5]);
}
