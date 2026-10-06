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

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::pci_match::{is_intel_wifi, is_supported_adapter, Candidate};
use crate::setup::announce;

const MAX_DEVICES: usize = 96;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    /// The legacy interrupt pin and line as firmware left them. Either may say
    /// "none" (pin 0, line 0xFF); the bind then goes straight to MSI-X.
    pub irq_pin: u8,
    pub irq_line: u8,
    pub bar0_size: u64,
    pub pci_device: u16,
}

pub fn find_iwlwifi() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        let c = candidate(r);
        if is_intel_wifi(&c) {
            announce(c.device);
        }
        if is_supported_adapter(&c) {
            return Some(Found {
                device_id: r.device_id,
                irq_pin: r.irq_pin,
                irq_line: r.irq_line,
                bar0_size: r.bars[0].size,
                pci_device: r.device,
            });
        }
    }
    None
}

// The fields of a broker record the adapter decision reads.
fn candidate(r: &DeviceRecord) -> Candidate {
    let bar0 = r.bars[0];
    Candidate {
        is_pci: r.bus_kind == BUS_KIND_PCI,
        vendor: r.vendor,
        device: r.device,
        class: r.pci_class,
        subclass: r.pci_subclass,
        bar0_mmio: r.bar_count != 0 && bar0.kind == BAR_KIND_MMIO && bar0.size != 0,
    }
}
