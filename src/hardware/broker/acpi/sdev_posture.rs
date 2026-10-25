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

//! One posture line for each device SDEV asks the OS to keep secured:
//! where it is, whether firmware lets it be handed off, and whether a
//! confining domain holds it now (and whose). A namespace device has no
//! requester id of its own for a domain to hold, and says so. Read-only:
//! what the IOMMU does about a listed device is not decided here.

use super::super::confine::held_by;
use super::kernel::platform::fw;
use crate::drivers::pci::config::access::read8;
use crate::memory::iommu::DeviceAddress;
use crate::sys::serial::Line;
use fw::parser::other::shared_tables::sdev::{resolve, SecureDevice};

/// A bridge's secondary bus, None for anything that is not one.
fn secondary(bus: u8, dev: u8, func: u8) -> Option<u8> {
    let header = read8(bus, dev, func, 0x0E).ok()?;
    let sec = read8(bus, dev, func, 0x19).ok()?;
    (header & 0x7F == 1 && sec != 0 && sec != 0xFF).then_some(sec)
}

pub fn sdev_posture() {
    for e in fw::parser::other::sdev_entries() {
        let mut line = Line::new();
        line.str(b"[SDEV] posture ");
        match &e.device {
            SecureDevice::Acpi(path) => {
                line.str(b"acpi ").str(path).str(b" handoff=").dec(u64::from(e.handoff));
                line.str(b" confined=n/a (no requester id)");
            }
            SecureDevice::Pci { segment, start_bus, path } => {
                line.str(b"pci seg=").dec(u64::from(*segment));
                match resolve(*start_bus, path, secondary) {
                    Some((b, d, f)) => {
                        line.str(b" bdf=").hex(u64::from(b)).str(b":").hex(u64::from(d));
                        line.str(b".").hex(u64::from(f));
                        line.str(b" handoff=").dec(u64::from(e.handoff));
                        match held_by(DeviceAddress::pci(b, d, f)) {
                            Some(pid) => line.str(b" confined=yes pid=").dec(u64::from(pid)),
                            None => line.str(b" confined=no"),
                        };
                    }
                    None => {
                        line.str(b" start_bus=").hex(u64::from(*start_bus));
                        line.str(b" path unresolved (a bridge did not answer)");
                    }
                }
            }
        }
        line.end();
    }
}
