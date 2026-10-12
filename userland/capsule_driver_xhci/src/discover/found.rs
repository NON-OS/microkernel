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

#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub bar0_size: u64,
    pub vendor: u16,
    pub device: u16,
    /// False for a device-tree controller: it has no config space to enable
    /// bus mastering in, and no MSI-X.
    pub on_pci: bool,
    /// A device-tree controller's GIC SPI, zero for none.
    pub spi: u32,
}
