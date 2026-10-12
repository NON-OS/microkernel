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

//! The state a binding is driven with. A boot keyboard or mouse that takes
//! SET_PROTOCOL(boot) is read in its fixed boot layout. One that refuses it
//! stays in report protocol, where a report may start with a report ID the
//! boot layout would take for the modifier keys; it is read through its
//! report descriptor, as any other HID interface is. That binds only when
//! the descriptor names reports this driver reads, each short enough to
//! come in one interrupt read: a gamepad, a UPS or a vendor channel was
//! read as QEMU's tablet before, and clicked and moved a pointer that was
//! not there.

use alloc::boxed::Box;

use crate::descriptors::{HidBinding, HidKind};
use crate::hid::{Device, Keyboard, Mouse, ReportDevice};
use crate::xhci::{control_transfer, Setup};

use super::poll::read_len;
use super::report_descriptor::read_layout;

/// A class request to an interface, host to device (HID 1.11, 7.2).
const RT_CLASS_INTERFACE_OUT: u8 = 0x21;
const SET_PROTOCOL: u8 = 0x0B;
const BOOT_PROTOCOL: u16 = 0;

/// The binding, its kind now the layout its reports are read in, and the
/// state that reads them; None when nothing here reads them.
pub fn device_for(xhci_port: u32, slot: u8, binding: HidBinding) -> Option<(HidBinding, Device)> {
    let boot = match binding.kind {
        HidKind::Keyboard => Some(Device::Keyboard(Keyboard::new())),
        HidKind::Mouse => Some(Device::Mouse(Mouse::new())),
        HidKind::Report => None,
    };
    if let Some(dev) = boot {
        if set_boot_protocol(xhci_port, slot, binding.interface_number) {
            return Some((binding, dev));
        }
    }
    let layout = read_layout(xhci_port, slot, binding)?;
    let report = HidBinding { kind: HidKind::Report, ..binding };
    let dev = || Device::Report(Box::new(ReportDevice::new(layout)));
    layout.bindable(read_len(binding.max_packet_size)).then(|| (report, dev()))
}

/// SET_PROTOCOL(boot) (HID 1.11, 7.2.6); whether the interface took it.
fn set_boot_protocol(xhci_port: u32, slot: u8, iface: u8) -> bool {
    let setup = Setup::new(RT_CLASS_INTERFACE_OUT, SET_PROTOCOL, BOOT_PROTOCOL, iface.into(), 0);
    control_transfer(xhci_port, slot, setup, &mut []).is_ok()
}
