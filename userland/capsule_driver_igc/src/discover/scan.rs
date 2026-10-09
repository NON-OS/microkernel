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

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO};

use super::bar_command::command_bits;
use super::found::Found;
use super::support::is_match;

/// The device list holds ACPI and fabricated records beside PCI functions;
/// at 32 a machine with more stopped short of the device behind a root port.
const MAX_DEVICES: usize = 128;

pub fn find_igc() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let count = core::cmp::min(n as usize, MAX_DEVICES);
    for r in &buf[..count] {
        if !is_match(r) {
            continue;
        }
        // Interrupt routing is not asked for: the driver polls. UEFI firmware
        // often leaves Interrupt Line at 0xFF, and filtering on it would skip
        // a working NIC.
        if r.bar_count == 0 {
            continue;
        }
        let bar0 = r.bars[0];
        if bar0.kind != BAR_KIND_MMIO || bar0.size == 0 {
            continue;
        }
        return Some(Found {
            device_id: r.device_id,
            pci_device: r.device,
            bar0_size: bar0.size,
            command_bits: command_bits(r),
        });
    }
    None
}
