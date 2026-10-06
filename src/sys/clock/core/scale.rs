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

//! Counter ticks as milliseconds.

/// Milliseconds in `ticks` of a counter running at `hz`, the product widened
/// before it is divided. Taken in 64 bits, `ticks * 1000` overflows once the
/// counter passes 2^64 / 1000 ticks, about 71 days of uptime at 3 GHz, and a
/// kernel built with overflow checks then panicked on the next clock read,
/// which any capsule could make with MkTimeMillis or MkTimeMonotonic. A result
/// too large for 64 bits saturates rather than wrapping; `hz` of zero reads as
/// no time at all.
pub(super) fn ticks_to_ms(ticks: u64, hz: u64) -> u64 {
    if hz == 0 {
        return 0;
    }
    let ms = (ticks as u128 * 1000) / hz as u128;
    u64::try_from(ms).unwrap_or(u64::MAX)
}
