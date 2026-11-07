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

//! Which key `latency` times: the time from a key's press to the frame that
//! shows it. A press was timed whether or not it asked for a frame, so a key
//! that changes nothing on screen (a modifier alone, a key no binding takes)
//! was timed to whatever frame came next, seconds later on an idle window,
//! and its wait was counted as the key's. Only a press that asks for a frame
//! starts the clock now; any other leaves the one already running.

/// When the key being timed was pressed, after an event at `now`: `now` for
/// a key-down that asked for a frame, else `was` unchanged (0 when nothing
/// is timed).
pub fn timed_from(was: i64, now: i64, key_down: bool, repaint: bool) -> i64 {
    if key_down && repaint {
        now
    } else {
        was
    }
}
