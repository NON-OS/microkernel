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

//! The bytes the controller reads: the descriptor list and the SET FEATURES
//! dwords (NVMe 1.4, figures 5.21.1.13-1 and -2).

pub const DESCRIPTOR_BYTES: usize = 16;

/// One descriptor: the piece's device address (page aligned) and its size
/// in pages, then four reserved bytes.
pub fn descriptor(device_addr: u64, pages: u32) -> [u8; DESCRIPTOR_BYTES] {
    let mut d = [0u8; DESCRIPTOR_BYTES];
    d[0..8].copy_from_slice(&device_addr.to_le_bytes());
    d[8..12].copy_from_slice(&pages.to_le_bytes());
    d
}

/// Command dwords 11 to 15 of SET FEATURES Host Memory Buffer: enable, the
/// size in pages, the descriptor list's address, and its entry count.
pub const fn enable_dwords(pages: u32, list_addr: u64, entries: u32) -> [u32; 5] {
    [1, pages, list_addr as u32, (list_addr >> 32) as u32, entries]
}

/// Number of Queues dword 11: one I/O submission and one completion queue,
/// both counted from zero (NVMe 1.4, 5.21.1.7).
pub const ONE_QUEUE_PAIR: u32 = 0;
