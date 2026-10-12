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
//! A device with no boot interface is a tablet only where an interface's
//! report descriptor says it is a pointer. Read once the device is
//! configured, as the class asks of an interface request. A security key,
//! a gamepad or a vendor channel is left, its slot given back for the
//! driver it belongs to; one whose descriptor cannot be read is left too.

use alloc::vec::Vec;

use crate::descriptors::{is_pointer, HidBinding, HidKind};
use crate::xhci::control_transfer;

const GET_DESCRIPTOR: u8 = 0x06;
/// Standard, device to host, to an interface.
const TO_INTERFACE_IN: u8 = 0x81;
const REPORT_DESCRIPTOR: u16 = 0x22 << 8;
/// The most the control pipe client reads; a pointer's is far shorter.
const REPORT_MAX: usize = 512;

pub(super) fn keep_pointers(xhci: u32, slot: u8, bindings: Vec<HidBinding>) -> Vec<HidBinding> {
    let pointer = |b: &HidBinding| {
        let mut report = [0u8; REPORT_MAX];
        let index = b.interface_number as u16;
        let len = REPORT_MAX as u16;
        control_transfer(
            xhci,
            slot,
            TO_INTERFACE_IN,
            GET_DESCRIPTOR,
            REPORT_DESCRIPTOR,
            index,
            len,
            &mut report,
        )
        .is_ok_and(|n| is_pointer(&report[..n]))
    };
    bindings.into_iter().filter(|b| b.kind != HidKind::Tablet || pointer(b)).collect()
}
