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

use super::State;

impl State {
    /* Tell the page's document what the session history holds, which its
     * scripts read as history.length: after an entry is added, rewritten
     * or stepped to, and when a document is homed. */
    pub fn note_history(&mut self) {
        let h = &self.ui.history;
        let (n, at) = (h.entries.len() as u32, h.index as u32);
        if let Some(dom) = self.page_dom.as_mut() {
            dom.history = (n, at);
        }
    }
}
