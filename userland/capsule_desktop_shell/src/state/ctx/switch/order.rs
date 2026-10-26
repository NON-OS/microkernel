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

//! The application windows, most recently focused first, as the window
//! manager's notices told them: a focused window goes to the front, an
//! opened one behind the focused one, a closed one out.

use alloc::vec::Vec;

use super::session::Session;

/// A window by its owner's pid and its window id. An app's window id is
/// fixed in its manifest, so only the pid tells two instances apart.
pub type Win = (u32, u32);

/// Cap on the windows held, as the dock's own; the oldest goes first.
pub const ORDER_MAX: usize = 64;

#[derive(Default)]
pub struct SwitchOrder {
    pub(super) recent: Vec<Win>,
    /// The run of presses in progress, if any.
    pub session: Option<Session>,
}

impl SwitchOrder {
    pub fn focused(&mut self, w: Win) {
        self.recent.retain(|&r| r != w);
        self.recent.insert(0, w);
        self.recent.truncate(ORDER_MAX);
    }

    /// A new window is not the focused one until the window manager says so.
    pub fn opened(&mut self, w: Win) {
        if !self.recent.contains(&w) {
            self.recent.insert(self.recent.len().min(1), w);
            self.recent.truncate(ORDER_MAX);
        }
    }

    /// Forget a window, here and in the run in progress.
    pub fn closed(&mut self, w: Win) {
        self.recent.retain(|&r| r != w);
        if let Some(s) = self.session.as_mut() {
            s.forget(w);
        }
    }
}
