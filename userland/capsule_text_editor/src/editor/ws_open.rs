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

//! Open a tree path into a document tab (reusing an existing tab for the same
//! file) and close tabs, always leaving at least one document open.

use super::app::Editor;
use super::ctrl_open::load;
use super::state::State;

fn doc_path(d: &State) -> &str {
    core::str::from_utf8(&d.path[..d.path_len]).unwrap_or("")
}

impl Editor {
    /// Open `path` in its own tab: the tab that already holds it, else the
    /// front tab when it is an untouched blank, else a new tab. Nothing that
    /// holds text is ever replaced, and a file that fails to load opens no tab.
    pub(super) fn open_path(&mut self, path: &str) -> bool {
        if let Some(i) = self.docs.iter().position(|d| doc_path(d) == path) {
            self.active = i;
            return true;
        }
        let front = self.active.min(self.docs.len().saturating_sub(1));
        let blank = self
            .docs
            .get(front)
            .map(|d| d.len == 0 && !d.dirty && d.path_len == 0)
            .unwrap_or(false);
        if blank {
            let ok = load(&mut self.docs[front], path.as_bytes());
            if ok {
                self.mru_note(path);
            }
            return ok;
        }
        let mut d = State::new();
        d.owner_pid = self.owner_pid;
        if !load(&mut d, path.as_bytes()) {
            // The reason belongs where the user is looking.
            let why = d.status;
            self.doc().status = why;
            return false;
        }
        self.mru_note(path);
        self.docs.push(d);
        self.active = self.docs.len() - 1;
        true
    }
}
