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

//! Whether the document matches what was last read or written.
//!
//! Each edit names the state it leads to, and the save point records the name
//! of the state on disk. The document is clean exactly when the top of the undo
//! stack names that state, so undoing back to the saved text is clean again and
//! undoing past it is not.

use super::state::State;

/// Ids of document states: the next edit's, the one on disk, and the one
/// history bottoms out at once old steps are dropped.
#[derive(Default)]
pub struct SavePoint {
    pub next_op: u64,
    pub on_disk: u64,
    pub floor: u64,
}

impl State {
    /// The state the buffer is in now: the top of the undo stack, or the floor
    /// history bottoms out at.
    pub(super) fn top_id(&self) -> u64 {
        self.undo.last().map_or(self.saved.floor, |op| op.id)
    }

    pub(super) fn refresh_dirty(&mut self) {
        self.dirty = self.top_id() != self.saved.on_disk;
    }

    /// The buffer now matches the file on disk.
    pub(super) fn mark_saved(&mut self) {
        self.saved.on_disk = self.top_id();
        self.dirty = false;
    }

    /// A fresh load: the old history describes a different text, so replaying
    /// it here would corrupt this one. Selection and search go with it.
    pub(super) fn reset_history(&mut self) {
        self.undo.clear();
        self.redo.clear();
        self.saved.floor = self.saved.next_op;
        self.saved.on_disk = self.saved.next_op;
        self.dirty = false;
        self.sel_anchor = None;
        self.selecting = false;
        self.find_active = false;
        self.replace_active = false;
    }
}
