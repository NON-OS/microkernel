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

use super::device::PciDevice;

impl PciDevice {
    pub fn bus(&self) -> u8 {
        self.address.bus
    }

    pub fn device(&self) -> u8 {
        self.address.device
    }

    pub fn function(&self) -> u8 {
        self.address.function
    }

    pub fn vendor_id(&self) -> u16 {
        self.device_id_info.vendor_id
    }

    pub fn device_id_value(&self) -> u16 {
        self.device_id_info.device_id
    }

    pub fn class(&self) -> u8 {
        self.class_code.class
    }

    pub fn subclass(&self) -> u8 {
        self.class_code.subclass
    }

    pub fn prog_if(&self) -> u8 {
        self.class_code.prog_if
    }

    pub fn revision(&self) -> u8 {
        self.device_id_info.revision
    }
}
