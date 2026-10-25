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
//! report descriptor, as any other HID interface is. That binds when the
//! descriptor names reports this driver reads, each short enough to come in
//! one interrupt read, or lays out a joystick or game pad, read as a pad
//! (`pad`). A UPS or a vendor channel was read as QEMU's tablet before, and
//! clicked and moved a pointer that was not there; a security key is
//! driver.fido0's.

use alloc::boxed::Box;

use crate::descriptors::report::parse;
use crate::descriptors::{pad_layout, HidBinding, HidKind};
use crate::hid::{Device, Keyboard, Mouse, Pad, ReportDevice};
use crate::protocol::REPORT_DESCRIPTOR_MAX;
use crate::xhci::{control_transfer, Setup};

use super::poll::read_len;
use super::report_descriptor::read_report;

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
        HidKind::Report | HidKind::Gamepad => None,
    };
    if let Some(dev) = boot {
        if set_boot_protocol(xhci_port, slot, binding.interface_number) {
            return Some((binding, dev));
        }
    }
    let mut buf = [0u8; REPORT_DESCRIPTOR_MAX];
    let n = read_report(xhci_port, slot, binding, &mut buf)?;
    let raw = &buf[..n];
    if let Some(layout) = parse(raw).ok().filter(|l| l.bindable(read_len(binding.max_packet_size)))
    {
        let report = HidBinding { kind: HidKind::Report, ..binding };
        return Some((report, Device::Report(Box::new(ReportDevice::new(layout)))));
    }
    let pad = pad_layout(raw)?;
    Some((HidBinding { kind: HidKind::Gamepad, ..binding }, Device::Pad(Box::new(Pad::new(pad)))))
}

/// SET_PROTOCOL(boot) (HID 1.11, 7.2.6); whether the interface took it.
fn set_boot_protocol(xhci_port: u32, slot: u8, iface: u8) -> bool {
    let setup = Setup::new(RT_CLASS_INTERFACE_OUT, SET_PROTOCOL, BOOT_PROTOCOL, iface.into(), 0);
    control_transfer(xhci_port, slot, setup, &mut []).is_ok()
}
