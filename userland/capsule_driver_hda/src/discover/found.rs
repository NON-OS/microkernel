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
//! One HD Audio controller as the device list reports it.

use nonos_libc::DeviceRecord;

use crate::constants::HDA_BAR_INDEX;
use crate::controller::intel::graphics_audio;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub vendor: u16,
    pub device: u16,
    pub subclass: u8,
    pub irq_pin: u8,
    pub irq_line: u8,
    pub bar_size: u64,
}

impl Found {
    pub fn graphics(&self) -> bool {
        graphics_audio(self.vendor, self.device)
    }
}

pub(super) fn found(r: &DeviceRecord) -> Found {
    Found {
        device_id: r.device_id,
        vendor: r.vendor,
        device: r.device,
        subclass: r.pci_subclass,
        irq_pin: r.irq_pin,
        irq_line: r.irq_line,
        bar_size: r.bars[HDA_BAR_INDEX as usize].size,
    }
}
