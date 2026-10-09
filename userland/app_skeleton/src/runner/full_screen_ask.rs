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

//! An app asking for full screen (`App::wants_full_screen`): from its first
//! frame, as a window that opens full screen, or while it does something,
//! as Video while it plays. The runner takes the window full screen the way
//! the green button does, through one path (maximize.rs), so the dock hides
//! and green restores the rect the window had.
//!
//! The ask follows changes only, and gives back only what it took: a window
//! the person made full screen with green stays so when the app stops
//! asking, and green pressed while the app asks is the person's choice,
//! which the app's next change does not undo.

/// What the runner does with the window this frame.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Turn {
    Stay,
    Enter,
    Leave,
}

#[derive(Clone, Copy)]
pub struct FullScreenAsk {
    /// The app's ask as last seen.
    seen: bool,
    /// The window is full screen because the app asked, not by green.
    by_ask: bool,
}

impl FullScreenAsk {
    pub const fn new() -> Self {
        Self { seen: false, by_ask: false }
    }

    /// The app's ask this frame (`want`), with the window full screen or not
    /// (`full`).
    pub fn step(&mut self, want: bool, full: bool) -> Turn {
        if want == self.seen {
            return Turn::Stay;
        }
        self.seen = want;
        if want && !full {
            self.by_ask = true;
            return Turn::Enter;
        }
        if !want && full && self.by_ask {
            self.by_ask = false;
            return Turn::Leave;
        }
        self.by_ask = false;
        Turn::Stay
    }

    /// The person pressed green (or resized the window): what the window is
    /// now is theirs.
    pub fn person_chose(&mut self) {
        self.by_ask = false;
    }
}

impl Default for FullScreenAsk {
    fn default() -> Self {
        Self::new()
    }
}
