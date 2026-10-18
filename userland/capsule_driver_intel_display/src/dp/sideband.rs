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

//! Multi-stream sideband messages are refused outright, before the bus is
//! touched. This driver drives one stream to one sink; a branch device's
//! message buffers would be a whole protocol of untrusted input, and
//! turning MST on would hand the topology to the sink. The addresses are
//! DisplayPort 1.2's as recalled (unverified here): MSTM_CTRL at 0x111,
//! and the DOWN_REQ, UP_REP, DOWN_REP and UP_REQ buffers at 0x1000-0x17FF.

use core::ops::Range;

const MSTM_CTRL: u32 = 0x111;
const MESSAGES: Range<u32> = 0x1000..0x1800;

/// Whether a native transaction on `len` bytes from `addr` reaches either.
pub fn refused(addr: u32, len: usize) -> bool {
    let end = addr.saturating_add(len.min(u32::MAX as usize) as u32);
    let reaches = |r: Range<u32>| addr < r.end && r.start < end;
    reaches(MSTM_CTRL..MSTM_CTRL + 1) || reaches(MESSAGES)
}
