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

// The arithmetic of placing a self-relocating image inside an aligned
// block. Pure, so the rule is held on the host (loader_arm_proofs).

/// Bytes to allocate so that a whole `align` block, plus the image's offset
/// in it (less than `align`), plus `total` bytes fit wherever the allocation
/// starts: the block may begin up to `align` in, and the offset adds up to
/// `align` more. None when `align` is not a power of two or it overflows.
pub fn span(total: u64, align: u64) -> Option<u64> {
    if !align.is_power_of_two() {
        return None;
    }
    total.checked_add(align.checked_mul(2)?)
}

/// The image's base for an allocation starting at `at`: the first whole
/// `align` block at or after `at`, plus `virt_min`'s offset inside a block.
pub fn base(at: u64, virt_min: u64, align: u64) -> Option<u64> {
    if !align.is_power_of_two() {
        return None;
    }
    let block = at.checked_add(align - 1)? & !(align - 1);
    block.checked_add(virt_min & (align - 1))
}
