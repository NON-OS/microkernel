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

//! How often the viewer asks the shell for a path to open.
//!
//! A viewer launched to open a file (a desktop icon, Files' Enter on an
//! image) was launched after the shell left the path, so while it has
//! nothing to show, in its first seconds, it asks on every tick. Once it shows
//! an image or the gallery, a path can only come from a file opened while the
//! viewer is up, and the shell raises this window for it. Then it asks about
//! once a second, as Music and Video do, rather than calling the shell every
//! 150 ms for as long as the window is open.

/// The window's tick (`App::tick_interval_ms`).
pub const TICK_MS: i64 = 150;
/// How long after start an empty window keeps asking on every tick.
pub const STARTUP_MS: i64 = 3_000;
/// The wait between asks after that, one tick short of a second: an ask comes
/// on the first tick at or past it, so a path left at any moment is taken
/// within a second.
pub const EVERY_MS: i64 = 1_000 - TICK_MS;

/// When to ask next, having just asked at `now` in a window that started at
/// `started`, `showing` an image or the gallery or not.
pub fn next_ask(now: i64, started: i64, showing: bool) -> i64 {
    if !showing && now.saturating_sub(started) < STARTUP_MS {
        now
    } else {
        now.saturating_add(EVERY_MS)
    }
}
