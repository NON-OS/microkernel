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

//! What the clock says when the system clock cannot be read or set. The
//! kernel answers a wall-clock read with an error until it has a time, and a
//! clock face drawn from no reading shows midnight on a day that never was.

/// The Clock tab's line while the system clock has no time.
pub const NOT_SET: &[u8] = b"The system clock is not set";

/// The Set tab's line while there is no date to set the time on.
pub const NO_DATE: &[u8] = b"Not set: the system clock has no date yet";

/// The Set tab's line after the kernel took the new time.
pub const DONE: &[u8] = b"Time set";

/// The wall-clock time in milliseconds from the kernel's answer, or `None`
/// while it has none (it answers with a negative errno).
pub fn wall_ms(answer: i64) -> Option<u64> {
    u64::try_from(answer).ok()
}

/// The Set tab's line for the kernel's answer to a time correction.
pub fn adjust_outcome(rc: i64) -> &'static [u8] {
    match rc {
        0.. => DONE,
        -22 => b"Not set: the time must fall between 2025 and 2100",
        -1 | -13 => b"Not set: this app may not change the clock",
        _ => b"Not set: the clock refused the change",
    }
}
