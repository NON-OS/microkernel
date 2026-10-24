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

//! The per-claim line the broker's confinement says.

use crate::memory::iommu::DeviceAddress;

pub(super) fn say(what: &[u8], pid: u32, device: DeviceAddress) {
    let serial = crate::sys::serial::print;
    serial(b"[VT-D] pid=");
    crate::sys::serial::print_dec(pid as u64);
    // Segment above the requester id, so segment 0 prints as it always has.
    match device {
        DeviceAddress::Pci { segment, rid } => {
            serial(b" device=");
            crate::sys::serial::print_hex((segment as u64) << 16 | rid as u64);
        }
        DeviceAddress::Stream(sid) => {
            serial(b" stream=");
            crate::sys::serial::print_hex(sid as u64);
        }
    }
    serial(b" ");
    crate::sys::serial::println(what);
}

/// `say` for a device with no DMA requester to name it by.
pub(super) fn say_device(what: &[u8], pid: u32, device_id: u64) {
    let serial = crate::sys::serial::print;
    serial(b"[VT-D] pid=");
    crate::sys::serial::print_dec(pid as u64);
    serial(b" described device=");
    crate::sys::serial::print_dec(device_id);
    serial(b" ");
    crate::sys::serial::println(what);
}
