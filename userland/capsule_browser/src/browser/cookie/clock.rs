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

//! The time cookie expiry is judged at.

/// 2026-01-01T00:00:00Z. No cookie is judged at an earlier time.
pub const FLOOR: i64 = 1_767_225_600;

/*
 * The wall clock in milliseconds, as the fetch machine reads it. A machine
 * whose clock was never set reads near zero, and then every Expires date
 * looks far in the future, including the 1970 one a site writes to log
 * the reader out. Holding the clock at no earlier than this build's year
 * keeps such a deletion a deletion; it costs only that a session-length
 * Max-Age counts from that floor until the clock is set.
 */
/// Unix seconds for cookie expiry from the wall clock `now_ms`.
pub fn unix_secs(now_ms: i64) -> i64 {
    (now_ms / 1000).max(FLOOR)
}
