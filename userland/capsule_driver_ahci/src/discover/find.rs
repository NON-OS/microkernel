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

//! The walk of the device list for AHCI controllers.

use alloc::vec::Vec;

use nonos_libc::{mk_device_list, DeviceRecord};

use super::candidate::is_candidate;
use super::found::Found;
use crate::constants::{AHCI_ABAR_BAR, CLASS_BLOCK};
use crate::log::Line;

/// The device list holds ACPI and fabricated records beside PCI functions;
/// at 32 a machine with more stopped short of the device behind a root port.
const MAX_DEVICES: usize = 128;

/// Every AHCI controller the device list reports, in device list order. The
/// driver walks all of them: the NONOS disk need not sit on the first one.
pub fn find_ahci() -> Vec<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(CLASS_BLOCK, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return Vec::new();
    }
    buf[..core::cmp::min(n as usize, MAX_DEVICES)]
        .iter()
        .filter(|r| is_candidate(r))
        .inspect(|r| {
            Line::new()
                .text(b"controller ")
                .hex(u32::from(r.vendor) << 16 | u32::from(r.device))
                .text(b" class ")
                .hex(
                    u32::from(r.pci_class) << 16
                        | u32::from(r.pci_subclass) << 8
                        | u32::from(r.pci_progif),
                )
                .text(b" ABAR ")
                .num(r.bars[AHCI_ABAR_BAR as usize].size)
                .text(b" bytes")
                .send();
        })
        .map(|r| Found {
            device_id: r.device_id,
            irq_line: r.irq_line,
            abar_size: r.bars[AHCI_ABAR_BAR as usize].size,
        })
        .collect()
}
