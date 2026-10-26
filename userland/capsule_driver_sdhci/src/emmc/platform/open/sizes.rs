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

//! The descriptor table size and the page the broker maps in.

/// The descriptor table page.
pub const DESC_TABLE_BYTES: u64 = 4096;
pub(super) const PAGE: u64 = 4096;

/// The bytes of a slot BAR of `size` the driver maps. The broker maps whole
/// pages and hands back the first byte asked for, so a BAR of a page or more
/// is mapped in whole pages, and one under a page as it is: QEMU's sdhci-pci
/// BAR is 0x100, one slot's registers and no more. None below one slot.
pub(super) const fn slot_length(size: u64, slot_window: u64) -> Option<u64> {
    let length = if size >= PAGE { size & !(PAGE - 1) } else { size };
    if length < slot_window {
        None
    } else {
        Some(length)
    }
}
