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

use super::config;
use super::error::Result;

pub fn pci_read_config32(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    config::read32_unchecked(bus, device, function, offset)
}

pub fn pci_write_config32(bus: u8, device: u8, function: u8, offset: u8, value: u32) {
    config::write32_unchecked(bus, device, function, offset, value)
}

pub fn pci_read_config32_safe(bus: u8, device: u8, function: u8, offset: u8) -> Result<u32> {
    config::read32(bus, device, function, offset as u16)
}

pub fn pci_write_config32_safe(
    bus: u8,
    device: u8,
    function: u8,
    offset: u8,
    value: u32,
) -> Result<()> {
    config::write32(bus, device, function, offset as u16, value)
}
