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

//! Whether a DMA grant the broker made can carry what the driver asked of it.

const PAGE_MASK: u64 = 0xFFF;

/// A grant of at least `want` bytes, page aligned on both sides, at non-zero
/// addresses whose ends do not wrap. The broker hands out nothing else; a
/// grant that breaks this is refused before any queue is built over it, so
/// the device is never given an address the driver cannot stand behind.
pub fn usable(user_va: u64, device_addr: u64, length: u64, want: usize) -> bool {
    let Ok(want) = u64::try_from(want) else {
        return false;
    };
    user_va != 0
        && device_addr != 0
        && user_va & PAGE_MASK == 0
        && device_addr & PAGE_MASK == 0
        && length >= want
        && user_va.checked_add(want).is_some()
        && device_addr.checked_add(want).is_some()
}
