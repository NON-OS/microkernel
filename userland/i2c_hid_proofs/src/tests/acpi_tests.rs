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

//! What the controller answers to OP_ACPI_HID is a peer's word: an address
//! that is not a 7-bit one is refused, not cut to a byte, and a descriptor
//! register of zero is never written.

use i2c_transfer_proofs::protocol::OP_TRANSFER;
use nonos_i2cmodel::Command;

use super::fixture::{pad, PAD};
use super::recorder::recorded;
use crate::setup;

/// The reply body: address and register, little-endian, then pin and info.
fn answer(addr: u16, reg: u16) -> [u8; 8] {
    let (a, r) = (addr.to_le_bytes(), reg.to_le_bytes());
    [a[0], a[1], r[0], r[1], 0, 0, 0, 0]
}

#[test]
fn an_address_wider_than_seven_bits_is_refused_not_truncated() {
    // 0x0115 cut to a byte is 0x15, where the pad is.
    let r = recorded(pad(), Some(PAD), Some(answer(0x0100 | u16::from(PAD), 0x0001)));
    let state = setup::run().expect("setup");
    assert!(!state.found(), "0x0115 was bound as {:#04x}", state.addr);
    assert!(r.pad.lock().commands().is_empty(), "the pad was addressed");
}

#[test]
fn an_address_past_0x7f_or_zero_is_refused() {
    for bad in [0x0000, 0x0080, 0x00FF, 0xFFFF] {
        let r = recorded(pad(), Some(PAD), Some(answer(bad, 0x0001)));
        assert!(!setup::run().expect("setup").found(), "{bad:#06x} bound");
        assert!(r.pad.lock().commands().is_empty(), "{bad:#06x} addressed the pad");
        let asked = r.calls.borrow().iter().any(|c| c.0 == OP_TRANSFER);
        assert!(!asked, "a transfer was asked for at {bad:#06x}");
    }
}

#[test]
fn a_descriptor_register_of_zero_is_not_written_and_the_defaults_are_tried() {
    let r = recorded(pad(), Some(PAD), Some(answer(u16::from(PAD), 0x0000)));
    let state = setup::run().expect("setup");
    assert!(state.found(), "the pad at its default register was not bound");
    let cmds = r.pad.lock().take_commands();
    assert!(!cmds.contains(&Command::UnknownRegister(0)), "register 0 written: {cmds:?}");
}
