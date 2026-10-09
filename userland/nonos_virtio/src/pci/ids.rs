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

//! Virtio PCI identities.

pub const VIRTIO_VENDOR_ID: u16 = 0x1AF4;

/// A modern-only function's device id is 0x1040 plus its virtio device type
/// (0x1041 network, 0x1042 block, 0x1044 entropy, 0x1050 GPU). Transitional
/// functions keep the 0x1000 range and carry a legacy I/O BAR as well.
pub const MODERN_ID_FIRST: u16 = 0x1040;
pub const MODERN_ID_LAST: u16 = 0x107F;

pub const fn is_modern_id(device: u16) -> bool {
    device >= MODERN_ID_FIRST && device <= MODERN_ID_LAST
}
