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

//! An interface's report descriptor (HID 1.11, 7.1.1: GET_DESCRIPTOR of
//! type Report to the interface), read whole in one control transfer and
//! walked into the layout of the reports this driver reads.

use crate::descriptors::report::{parse, Layout};
use crate::descriptors::HidBinding;
use crate::protocol::REPORT_DESCRIPTOR_MAX;
use crate::xhci::{control_transfer, Setup};

const GET_DESCRIPTOR: u8 = 0x06;
/// A standard request to an interface, device to host.
const RT_INTERFACE_IN: u8 = 0x81;
const DT_REPORT: u16 = 0x22;

/// None when the interface gives no descriptor length, the read fails or
/// comes back short, or the walk refuses what came back. A descriptor longer
/// than one read is refused, not walked cut: a cut that falls between two
/// collections parses, and the reports of every collection past it would go
/// unread, or babble when longer than the layout says.
pub fn read_layout(xhci_port: u32, slot: u8, binding: HidBinding) -> Option<Layout> {
    let len = binding.report_len;
    if len == 0 || usize::from(len) > REPORT_DESCRIPTOR_MAX {
        return None;
    }
    let mut buf = [0u8; REPORT_DESCRIPTOR_MAX];
    let iface = u16::from(binding.interface_number);
    let setup = Setup::new(RT_INTERFACE_IN, GET_DESCRIPTOR, DT_REPORT << 8, iface, len);
    let n = control_transfer(xhci_port, slot, setup, &mut buf).ok()?;
    if n != usize::from(len) {
        return None;
    }
    parse(buf.get(..n)?).ok()
}
