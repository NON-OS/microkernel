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

//! When the test counts a DDC/CI reply read as taken off the bus.

use super::sink::Try;
use crate::engine::gmbus_xfer::deadline::XFER_US;

/// A reply read of `len` bytes taken: every word, no NAK, the wait phase
/// shown while its deadline (set 50 us after the pins) still ran.
pub fn taken(t: &Try, len: usize) -> bool {
    let words = len.div_ceil(4) * 4;
    let end = t.t0 + 50 + XFER_US;
    t.read == Some((0x37, 0, len))
        && !t.nak
        && t.got.len() == words
        && t.phase
        && t.phase_at + 50 < end
}
