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

//! The broker's record types as the kernel's `legacy::attach` names them,
//! with the kernel's own fixed-window flag by path.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bar {
    pub base: u64,
    pub size: u64,
    pub kind: u8,
    pub flags: u8,
    pub aux: u32,
    pub _pad: [u8; 2],
}

#[repr(u8)]
pub enum BarKind {
    None = 0,
    Mmio = 1,
    Pio = 2,
}

/// The fields of the kernel's record the windows are placed from and into.
pub struct DeviceRecord {
    pub pci_class: u8,
    pub pci_subclass: u8,
    pub pci_progif: u8,
    pub vendor: u16,
    pub device: u16,
    pub bar_count: u8,
    pub bars: [Bar; 6],
}

#[path = "../../../../../src/hardware/broker/device/flags.rs"]
mod flags;

// The kernel's record flags as its `device.rs` gives them: the device flags
// to everyone, the BAR's own two within the broker.
pub use flags::{
    BAR_FLAG_FIXED, DEVICE_FLAG_CLAIMED, DEVICE_FLAG_DEPS_UNMET, DEVICE_FLAG_DISABLED,
    DEVICE_FLAG_DMA_NONCOHERENT, DEVICE_FLAG_EXTERNAL, DEVICE_FLAG_IRQ_ACTIVE_LOW,
    DEVICE_FLAG_IRQ_EDGE,
};
pub(in crate::hardware::broker) use flags::{BAR_FLAG_MEM64, BAR_FLAG_PREFETCH};
