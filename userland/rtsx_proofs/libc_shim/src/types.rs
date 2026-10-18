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

//! The broker records and constants the driver reads, as nonos_libc lays
//! them out (broker/types and broker/pci.rs, broker/dma.rs).

pub const BAR_KIND_MMIO: u8 = 1;
pub const BUS_KIND_PCI: u8 = 1;
pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
pub const MK_PCI_CMD_INTX_DISABLE: u16 = 1 << 10;
pub const MK_DMA_MAP_DMA32: u32 = 1 << 1;
pub const MK_DMA_MAP_COHERENT: u32 = 1 << 2;

#[derive(Clone, Copy, Default)]
pub struct Bar {
    pub base: u64,
    pub size: u64,
    pub kind: u8,
}

#[derive(Clone, Copy, Default)]
pub struct DeviceRecord {
    pub device_id: u64,
    pub bus_kind: u8,
    pub pci_class: u8,
    pub vendor: u16,
    pub device: u16,
    pub bars: [Bar; 6],
}

impl DeviceRecord {
    pub const fn empty() -> Self {
        const NONE: Bar = Bar { base: 0, size: 0, kind: 0 };
        Self { device_id: 0, bus_kind: 0, pci_class: 0, vendor: 0, device: 0, bars: [NONE; 6] }
    }
}

#[derive(Clone, Copy)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

#[derive(Clone, Copy)]
pub struct MmioMapOut {
    pub user_va: u64,
    pub length: u64,
    pub grant_id: u64,
}
