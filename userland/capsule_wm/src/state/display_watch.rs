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

//! The display size this window manager places windows on, kept to the
//! compositor's. The compositor moves the desktop onto a new primary while
//! it runs: from the boot framebuffer to the display driver, onto a driver
//! that restarted, or to a mode the user chose (compositor setup/adopt.rs).
//! The size read once at start went stale then. Every window opened, moved
//! or placed after was clamped to the old display while its app clamped to
//! the new one, so the rect the window manager hit-tests stopped being the
//! rect drawn: a click on a window's middle missed it, and a press on its
//! title bar landed in a resize band.

/// The longest a size read is trusted before a placement asks again. A drag
/// moves the window many times a second; it asks at most this often.
pub const RECHECK_MS: i64 = 100;

#[derive(Clone, Copy, Debug, Default)]
pub struct DisplayWatch {
    checked_ms: Option<i64>,
}

impl DisplayWatch {
    pub const fn new() -> Self {
        Self { checked_ms: None }
    }

    /// Whether the size is to be read again at uptime `now_ms`.
    pub fn due(&self, now_ms: i64) -> bool {
        self.checked_ms.is_none_or(|t| !(0..RECHECK_MS).contains(&now_ms.saturating_sub(t)))
    }

    /// A read was made at `now_ms`, answered or not: an unanswered one is
    /// asked again after the same wait, never on every request.
    pub fn read(&mut self, now_ms: i64) {
        self.checked_ms = Some(now_ms);
    }
}
