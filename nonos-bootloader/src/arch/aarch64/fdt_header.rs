// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! A flattened device tree's header (Devicetree Specification, 5.2), big
//! endian: magic 0xd00dfeed at 0, totalsize at 4. Pure, so it is held on the
//! host (loader_arm_proofs).

const FDT_MAGIC: u32 = 0xd00d_feed;
/// The header's own size; a tree claiming less is not one.
const FDT_HEADER: u32 = 40;

/// The tree's total size from its first 8 bytes; None without the magic or
/// with a size below the header's.
pub fn fdt_size(head: [u8; 8]) -> Option<u64> {
    let magic = u32::from_be_bytes([head[0], head[1], head[2], head[3]]);
    let size = u32::from_be_bytes([head[4], head[5], head[6], head[7]]);
    (magic == FDT_MAGIC && size >= FDT_HEADER).then_some(u64::from(size))
}
