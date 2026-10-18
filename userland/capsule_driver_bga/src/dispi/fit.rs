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

//! The adapter's video memory, which its VIDEO_MEMORY_64K register reports:
//! a mode must fit it as well as the BAR0 mapping the broker made. QEMU
//! sizes BAR0 to its video memory, but an adapter is free to decode more
//! than it holds, and a mode past the memory scans out what is not there.

use super::dispi_off::dispi_off;
use crate::constants::DISPI_INDEX_VIDEO_MEMORY_64K;
use crate::regs::Regs;

/// The adapter's video memory in bytes.
pub fn video_memory(regs: Regs) -> u64 {
    // SAFETY: VIDEO_MEMORY_64K is the last index below `regs_len`, which the
    // caller has checked the register mapping covers.
    let units = unsafe { regs.r16(dispi_off(DISPI_INDEX_VIDEO_MEMORY_64K)) };
    units as u64 * 0x1_0000
}
