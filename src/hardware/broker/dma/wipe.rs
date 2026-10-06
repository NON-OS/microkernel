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

//! The shutdown wipe's share of DMA: every frame a live grant still holds.
//!
//! Packet buffers carry the traffic this machine sent and received, a block
//! driver's staging buffers carry file contents, and the GPU's primary surface
//! is the last frame on screen. Those frames belong to no process VMA and to
//! no heap, so neither the process wipe nor the heap erase reaches them, and a
//! warm reset leaves DRAM rows intact. Runs after every device has stopped
//! mastering the bus (`claim::quiesce_all`), so nothing writes behind it.
//!
//! The grant table lock is only tried: the other CPUs were stopped by IPI at
//! an arbitrary instruction, and one of them may hold it. Missing the wipe of
//! these frames costs coverage; waiting on a stopped CPU costs the shutdown.

use super::records::try_each;
use super::scrub::scrub;

/// Zero every frame a live grant holds. The grants stay recorded; the machine
/// is about to stop. The number of grants wiped, or `None` if the table was
/// held by a stopped CPU.
pub fn wipe_live_grants() -> Option<usize> {
    let mut wiped = 0usize;
    let seen = try_each(|g| {
        scrub(g.physical_start, g.length);
        wiped += 1;
    });
    seen.then_some(wiped)
}
