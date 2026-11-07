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

//! What the toast panel's window is after each call to the window manager.
//! The window was counted as closed or opened before the call was answered,
//! so a close that went unanswered left the shell's popup window over the
//! empty panel, taking clicks meant for the window under it, and an open
//! that failed left the panel's presses going through to that window;
//! nothing tried again, as the rect asked for matched the one counted.
//! Pure, so the host proofs hold it.

/// A window's rect: x, y, width, height.
pub type Panel = (u32, u32, u32, u32);

/// What the window manager said to the window's close.
pub const CLOSE_REFUSED: &str = "wm rejected window_close";

/// The window held after a close of `held` was answered `r`: none once it
/// closed, or was refused (the window manager holds no such window); still
/// `held` when the call got no answer, so the next sync closes it again.
pub fn after_close(held: Option<Panel>, r: Result<(), &str>) -> Option<Panel> {
    match r {
        Ok(()) | Err(CLOSE_REFUSED) => None,
        Err(_) => held,
    }
}

/// The window held after an open of `want` was answered: `want` once it
/// opened, none when it did not, so the next sync opens it again.
pub fn after_open(want: Panel, opened: bool) -> Option<Panel> {
    opened.then_some(want)
}
