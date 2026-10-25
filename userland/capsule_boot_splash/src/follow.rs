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
//! Following the compositor's canvas while the splash is up. The compositor
//! can move from the firmware framebuffer to a GPU driver at another size
//! after the splash has drawn (seen on QEMU: 1280x800 to 1920x1080), and a
//! splash that kept its first size would sit in one corner of the new
//! screen. So it asks the size again now and then, and when the answer is
//! a different one it is drawn afresh at that size. The clock it keeps
//! (when it went up, when the desktop was seen) carries across a redraw, so
//! a move never lengthens its stay.

/// How often to ask, in milliseconds of uptime.
pub const CHECK_MS: i64 = 250;
/// Redraws at most: a canvas that keeps changing leaves the splash as it is.
pub const MAX_REDRAWS: u32 = 4;

/// A canvas as DISPLAY_INFO gives it: width, height, stride in bytes.
pub type Size = (u32, u32, u32);

pub struct Follow {
    pub size: Size,
    pub start: i64,
    pub desktop_up_at: i64,
    pub iters: u32,
    pub redraws: u32,
    last: i64,
}

impl Follow {
    pub fn new(size: Size, now: i64) -> Self {
        Follow { size, start: now, desktop_up_at: -1, iters: 0, redraws: 0, last: now }
    }

    /// Whether to ask the size now: once `CHECK_MS` has passed since the
    /// last time, at once if the clock has gone back, never once the
    /// redraws are spent.
    pub fn due(&self, now: i64) -> bool {
        self.redraws < MAX_REDRAWS && (now < self.last || now - self.last >= CHECK_MS)
    }

    /// Takes an answer asked at `now`. A new size is returned, and counted,
    /// when it differs from the one drawn; a failed answer or the same size
    /// changes nothing.
    pub fn seen(&mut self, now: i64, answer: Result<Size, i32>) -> Option<Size> {
        self.last = now;
        match answer {
            Ok(s) if s != self.size && self.redraws < MAX_REDRAWS => {
                self.redraws += 1;
                self.size = s;
                Some(s)
            }
            _ => None,
        }
    }
}
