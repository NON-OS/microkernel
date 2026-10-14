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
//! One address declared twice with different HID descriptor registers, as
//! multi-SKU firmware does when two pads a chassis may carry share an
//! address. OP_ACPI_HID answers the bound declaration's register, so that
//! is the register the reach carries for the address: a client's
//! descriptor read there narrows the reach, and one at the other
//! declaration's register does not.

use nonos_i2cmodel::{Bus, Config, Designware, HidOverI2c};

use super::client::transfer_errno;
use super::declared::{here, pad_at};
use crate::bench::{settled, GEMINI_LAKE_HZ};
use crate::discover::AcpiTouchpad;
use crate::driver::Driver;
use crate::protocol::{E_INVAL, E_NACK, E_OK};
use crate::regs::{self, Attached};
use crate::transaction::Presence;

const PAD: u8 = 0x2C;
const OTHER: u8 = 0x15;

/// PAD declared first with its descriptor at 0x0020, then, as the bound
/// declaration, at 0x0001; another pad declared at OTHER.
fn declared() -> [AcpiTouchpad; 3] {
    [AcpiTouchpad { desc_reg: 0x0020, ..here(PAD) }, here(OTHER), here(PAD)]
}

/// `pad` alone on the bus, and the controller bound to PAD at 0x0001.
fn bound(pad: HidOverI2c, found: Presence) -> (Driver, Attached) {
    let a = regs::attach(Designware::new(Config::LPSS, Bus::default().with(pad)));
    let t = declared();
    (settled(GEMINI_LAKE_HZ, &t[2], found, &t).expect("bind"), a)
}

#[test]
fn the_reach_carries_the_bound_declarations_register() {
    for found in [Presence::Acked, Presence::Absent] {
        let (d, _a) = bound(pad_at(PAD), found);
        assert_eq!((d.bound_addr, d.bound_desc_reg), (PAD, 0x0001), "{found:?}");
        assert_eq!(d.reach.get().desc_reg(PAD), Some(0x0001), "{found:?}");
    }
}

#[test]
fn a_descriptor_read_at_the_bound_register_narrows_the_reach() {
    let (d, _a) = bound(pad_at(PAD), Presence::Acked);
    assert_eq!(transfer_errno(&d, PAD, &[0x01, 0x00], 30), E_OK);
    assert_eq!(transfer_errno(&d, OTHER, &[0x01, 0x00], 30), E_INVAL, "did not narrow");
}

#[test]
fn a_descriptor_read_at_the_other_declarations_register_does_not() {
    let (d, _a) = bound(pad_at(PAD).descriptor_at(0x0020), Presence::Acked);
    assert_eq!(transfer_errno(&d, PAD, &[0x20, 0x00], 30), E_OK);
    assert_eq!(transfer_errno(&d, OTHER, &[0x01, 0x00], 30), E_NACK, "narrowed at 0x0020");
}
