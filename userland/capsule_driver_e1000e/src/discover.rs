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

//! Find the first e1000e-family Ethernet function in the broker's device
//! list, with its family and the BAR0 window it decodes.

mod bar_command;
mod support;

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO};

use self::bar_command::command_bits;
use self::support::family_of;
use crate::constants::regs::BAR0_MIN_BYTES;
use crate::constants::Family;

/// The device list holds ACPI and fabricated records beside PCI functions;
/// at 32 a machine with more stopped short of the device behind a root port.
const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub pci_device: u16,
    pub family: Family,
    pub bar0_size: u64,
    pub command_bits: u16,
}

pub fn find_e1000e() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    // Interrupt routing is not asked for: the driver polls. UEFI firmware
    // often leaves Interrupt Line at 0xFF on exactly these machines.
    buf[..core::cmp::min(n as usize, MAX_DEVICES)].iter().find_map(found)
}

fn found(r: &DeviceRecord) -> Option<Found> {
    let family = family_of(r)?;
    if r.bar_count == 0 || r.bars[0].kind != BAR_KIND_MMIO || r.bars[0].size < BAR0_MIN_BYTES {
        return None;
    }
    Some(Found {
        device_id: r.device_id,
        pci_device: r.device,
        family,
        bar0_size: r.bars[0].size,
        command_bits: command_bits(r),
    })
}
