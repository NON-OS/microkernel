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

use super::bar::Bar;

pub const DEVICE_FLAG_CLAIMED: u32 = 1 << 0;
pub const DEVICE_FLAG_DISABLED: u32 = 1 << 1;
/// Behind a PCIe port firmware calls external-facing: whoever can reach the
/// connector chose what is plugged in, so it gets no untranslated DMA.
pub const DEVICE_FLAG_EXTERNAL: u32 = 1 << 2;
/// `numa_node` of a device no firmware table places, or past domain 254.
pub const NUMA_UNKNOWN: u8 = 0xFF;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DeviceRecord {
    pub device_id: u64,
    pub bus_kind: u8,
    pub pci_class: u8,
    pub pci_subclass: u8,
    pub pci_progif: u8,
    pub class: u32,
    pub vendor: u16,
    pub device: u16,
    pub flags: u32,
    pub bar_count: u8,
    pub irq_line: u8,
    pub irq_pin: u8,
    /// Proximity domain, `NUMA_UNKNOWN` when no firmware table places it.
    pub numa_node: u8,
    pub irq_source: u32,
    pub bars: [Bar; 6],
}

impl DeviceRecord {
    pub const fn empty() -> Self {
        Self {
            device_id: 0,
            bus_kind: 0,
            pci_class: 0,
            pci_subclass: 0,
            pci_progif: 0,
            class: 0,
            vendor: 0,
            device: 0,
            flags: 0,
            bar_count: 0,
            irq_line: 0xFF,
            irq_pin: 0,
            numa_node: NUMA_UNKNOWN,
            irq_source: 0,
            bars: [Bar::empty(); 6],
        }
    }
}

const _: () = assert!(core::mem::size_of::<DeviceRecord>() == 176);
