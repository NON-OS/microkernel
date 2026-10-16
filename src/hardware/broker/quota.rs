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

//! How much one holder may have granted at once, pure. Without a
//! bound one driver could map until the kernel ran out of frames or page
//! tables for everyone. Each is checked as a grant is entered, under its
//! table's lock. kernel_proofs::va_window.

/// MMIO grants a pid holds at once: BAR windows, a few per device.
pub(crate) const MAX_MMIO_GRANTS: usize = 64;
/// DMA grants a pid holds at once: rings, buffers and their descriptors.
pub(crate) const MAX_DMA_GRANTS: usize = 1024;
/// DMA bytes a pid holds at once: four of the largest map a class allows,
/// the display's 8192 pages.
pub(crate) const MAX_DMA_BYTES: u64 = 4 * 8192 * 4096;

/// Whether one more grant of `bytes` fits beside `held` grants of
/// `held_bytes`.
pub(crate) fn admits(held: usize, held_bytes: u64, bytes: u64, max: usize, max_bytes: u64) -> bool {
    held < max && held_bytes.checked_add(bytes).is_some_and(|total| total <= max_bytes)
}
