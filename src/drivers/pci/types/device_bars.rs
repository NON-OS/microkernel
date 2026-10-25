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

use super::bar::PciBar;
use super::capability::PciCapability;
use super::device::PciDevice;
use crate::memory::addr::PhysAddr;

impl PciDevice {
    pub fn get_bar(&self, index: usize) -> Option<&PciBar> {
        self.bars.get(index).filter(|b| b.is_present())
    }

    pub fn get_memory_bar(&self, index: usize) -> Option<PhysAddr> {
        self.bars.get(index).and_then(|b| b.address())
    }

    pub fn get_io_bar(&self, index: usize) -> Option<u16> {
        self.bars.get(index).and_then(|b| b.port())
    }

    pub fn find_capability(&self, id: u8) -> Option<&PciCapability> {
        self.capabilities.iter().find(|c| c.id == id)
    }

    pub fn has_capability(&self, id: u8) -> bool {
        self.capabilities.iter().any(|c| c.id == id)
    }
}
