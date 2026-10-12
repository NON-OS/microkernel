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

//! Who may take a screenshot: only a process holding the ScreenCapture
//! capability bit, asked of the kernel per request (MkCapCheck). The bit is
//! not ambient and no capsule may hold it until the owner names a holder
//! under Decision 9, so until then no screenshot can be taken at all.

/// The kernel's ScreenCapture bit (the lead's ruling after G-27).
pub const SCREEN_CAPTURE: Option<u64> = Some(1 << 39);
/// How long the band shows that the screen was captured.
pub const NOTICE_MS: i64 = 3000;

/// Whether `sender` may capture, given how the kernel answers a capability
/// check on it.
pub fn may_capture(sender: u32, holds: impl Fn(u32, u64) -> bool) -> bool {
    sender != 0 && SCREEN_CAPTURE.is_some_and(|mask| holds(sender, mask))
}
