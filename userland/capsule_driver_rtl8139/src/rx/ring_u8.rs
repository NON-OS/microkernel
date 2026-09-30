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

use crate::constants::dma::RX_BUF_BYTES;

/*
 * Linear, not modulo the ring. RCR.WRAP is set, so a frame that crosses the
 * end of the ring is written on past it into the slack after, not back at
 * the start: taking its tail from offset 0 handed up stale bytes once a lap.
 * Offsets stay below the allocation (the header is inside the ring and the
 * frame is length-checked first), and anything that would not reads as zero.
 */
pub(super) fn ring_u8(base: u64, off: usize) -> u8 {
    if off >= RX_BUF_BYTES {
        return 0;
    }
    unsafe { core::ptr::read_volatile((base + off as u64) as *const u8) }
}
