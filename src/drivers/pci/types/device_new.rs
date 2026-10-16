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

extern crate alloc;

use super::address::PciAddress;
use super::bar::PciBar;
use super::class_code::ClassCode;
use super::device::PciDevice;
use super::device_id::DeviceId;
use super::header::HeaderType;
use alloc::vec::Vec;

impl PciDevice {
    pub fn new(address: PciAddress) -> Self {
        Self {
            address,
            device_id_info: DeviceId::new(0xFFFF, 0xFFFF),
            class_code: ClassCode::new(0, 0, 0),
            header_type: HeaderType::Standard,
            multifunction: false,
            bars: [PciBar::NotPresent; 6],
            capabilities: Vec::new(),
            pcie_capabilities: Vec::new(),
            interrupt_line: 0xFF,
            interrupt_pin: 0,
            msi: None,
            msix: None,
            power_management: None,
            pcie: None,
            bus: address.bus,
            device: address.device,
            function: address.function,
            vendor_id: 0xFFFF,
            device_id: 0xFFFF,
            class: 0,
            subclass: 0,
            progif: 0,
        }
    }

    pub fn sync_compat_fields(&mut self) {
        self.bus = self.address.bus;
        self.device = self.address.device;
        self.function = self.address.function;
        self.vendor_id = self.device_id_info.vendor_id;
        self.device_id = self.device_id_info.device_id;
        self.class = self.class_code.class;
        self.subclass = self.class_code.subclass;
        self.progif = self.class_code.prog_if;
    }
}
