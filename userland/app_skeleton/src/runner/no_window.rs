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

//! What an app does when it was asked to open and has no window to show:
//! its peers were not found, or its window could not be opened. Pure, so a
//! host proof holds it.
//!
//! An on-demand instance whose window failed to open went back to idle like
//! a base app. Nothing sends it another open: the kernel boots an instance
//! with one frame. It stayed alive with no window, holding its instance slot
//! and its name, and once every slot was held so, the kernel handed each
//! later click to one of them as the app's "running" window, which opened
//! nothing. Now such an instance exits, so its slot is free for the next
//! click; a base app still waits for the next open, as before.

/// After an open that brought no window.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NoWindow {
    /// End the process: the kernel frees its slot and its name.
    Exit,
    /// Wait in idle for the next open.
    Idle,
}

pub fn no_window(ephemeral: bool) -> NoWindow {
    if ephemeral {
        NoWindow::Exit
    } else {
        NoWindow::Idle
    }
}
