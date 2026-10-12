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

//! What the context answers of itself: the next request id, and where the
//! pointer is drawn.

use super::Context;

impl Context {
    pub fn issue_request_id(&mut self) -> u32 {
        let id = self.next_request_id;
        self.next_request_id = id.wrapping_add(1).max(1);
        id
    }

    /// The pointer is on the wire driver's cursor plane, not composited.
    pub fn hw_cursor(&self) -> bool {
        // Extended, the pointer can be on any output: composited.
        // Magnified, the arrow is drawn into the canvas and magnified with it.
        let lens = self.looks.access[0].magnified();
        self.wire.is_some_and(|w| w.cursor_offset != 0) && self.plan.is_none() && !lens
    }
}
