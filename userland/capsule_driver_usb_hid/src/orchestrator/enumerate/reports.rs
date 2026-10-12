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
//! A device with no boot interface is bound by what each interface's report
//! descriptor says, read once the device is configured, as the class asks
//! of an interface request: a pointer as a tablet, a joystick or game pad
//! as a gamepad laid out by its descriptor. A security key or a vendor
//! channel is left, its slot given back for the driver it belongs to; one
//! whose descriptor cannot be read is left too.

use alloc::vec::Vec;

use crate::descriptors::{is_pointer, pad_layout, HidBinding, HidKind, PadLayout};
use crate::xhci::control_transfer;

const GET_DESCRIPTOR: u8 = 0x06;
/// Standard, device to host, to an interface.
const TO_INTERFACE_IN: u8 = 0x81;
const REPORT_DESCRIPTOR: u16 = 0x22 << 8;
/// The most the control pipe client reads; a gamepad's fits in it.
const REPORT_MAX: usize = 512;

pub(super) fn sort_by_report(
    xhci: u32,
    slot: u8,
    bindings: Vec<HidBinding>,
) -> Vec<(HidBinding, Option<PadLayout>)> {
    let sort = |mut b: HidBinding| {
        if b.kind != HidKind::Tablet {
            return Some((b, None));
        }
        let mut report = [0u8; REPORT_MAX];
        let (index, len) = (b.interface_number as u16, REPORT_MAX as u16);
        let n = control_transfer(
            xhci,
            slot,
            TO_INTERFACE_IN,
            GET_DESCRIPTOR,
            REPORT_DESCRIPTOR,
            index,
            len,
            &mut report,
        )
        .ok()?;
        if is_pointer(&report[..n]) {
            return Some((b, None));
        }
        let pad = pad_layout(&report[..n])?;
        b.kind = HidKind::Gamepad;
        Some((b, Some(pad)))
    };
    bindings.into_iter().filter_map(sort).collect()
}
