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
use nonos_virtio::Bars;

#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub register_bar: u8,
    pub register_kind: u8,
    pub register_size: u64,
    /// The PCI device id: transitional 0x1005 or modern-only 0x1044.
    pub pci_device: u16,
    /// Every BAR, for the transport choice and the capability checks.
    pub bars: Bars,
}
