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
    /* A load, or anything it pulls in, is still on its way. */
    pub fn loading(&self) -> bool {
        self.fetch.is_some()
            || self.pending_nav.is_some()
            || !self.css_queue.is_empty()
            || !self.script_queue.is_empty()
            || !self.image_queue.is_empty()
            || !self.font_queue.is_empty()
            || self.pool.busy()
    }
}
