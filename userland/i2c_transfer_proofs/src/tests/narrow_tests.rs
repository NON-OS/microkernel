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

//! The reach narrowing once the pad has answered. A controller bound while
//! its pad slept keeps every pad declared on it in reach; the first time a
//! client reads a valid HID descriptor back from one of them, at the
//! register that pad's declaration names, the reach becomes that address
//! alone for the rest of the session. Nothing else narrows it, and nothing
//! widens it again.

use nonos_i2cmodel::{Bus, Config, Designware};

use super::client::{transfer_errno, with_expander, EXPANDER};
use super::declared::{here, pad_at};
use crate::bench::{settled, GEMINI_LAKE_HZ};
use crate::protocol::{E_INVAL, E_NACK, E_OK};
use crate::regs;
use crate::transaction::Presence;

const PAD: u8 = 0x15;
const PHANTOM: u8 = 0x2C;
/// The HID descriptor register, as both declarations name it.
const DESC: [u8; 2] = [0x01, 0x00];

#[test]
fn a_descriptor_read_through_op_transfer_narrows_a_wide_reach_to_the_pad() {
    let _a = regs::attach(Designware::new(Config::LPSS, Bus::default().with(pad_at(PAD))));
    let (phantom, pad) = (here(PHANTOM), here(PAD));
    let d = settled(GEMINI_LAKE_HZ, &phantom, Presence::Absent, &[phantom, pad]).expect("bind");
    assert_eq!(transfer_errno(&d, PHANTOM, &DESC, 30), E_NACK, "the wide reach");
    // The input register is not the descriptor's.
    assert_eq!(transfer_errno(&d, PAD, &[0x03, 0x00], 2), E_OK);
    assert_eq!(transfer_errno(&d, PHANTOM, &DESC, 30), E_NACK, "narrowed on another read");
    assert_eq!(transfer_errno(&d, PAD, &DESC, 30), E_OK);
    assert_eq!(transfer_errno(&d, PHANTOM, &DESC, 30), E_INVAL, "the reach did not narrow");
    assert_eq!(transfer_errno(&d, PAD, &DESC, 30), E_OK, "the pad itself left the reach");
}

#[test]
fn a_malformed_descriptor_or_a_read_at_another_register_does_not_narrow() {
    // A Synaptics-style pad publishes its descriptor at 0x0020; firmware
    // declared 0x0001, where this pad has nothing, and an expander answers
    // the descriptor register with all ones.
    let (_pad, _expander, _a) = with_expander(pad_at(PAD).descriptor_at(0x0020));
    let declared = [here(PAD), here(EXPANDER)];
    let d = settled(GEMINI_LAKE_HZ, &declared[0], Presence::Acked, &declared).expect("bind");
    assert_eq!(transfer_errno(&d, EXPANDER, &DESC, 30), E_OK);
    assert_eq!(transfer_errno(&d, PAD, &DESC, 30), E_OK);
    assert_eq!(transfer_errno(&d, PAD, &[0x20, 0x00], 30), E_OK);
    assert_eq!(transfer_errno(&d, PAD, &[0x02, 0x00], 30), E_OK, "the report descriptor");
    assert_eq!(transfer_errno(&d, EXPANDER, &DESC, 30), E_OK, "narrowed without a descriptor");
}
