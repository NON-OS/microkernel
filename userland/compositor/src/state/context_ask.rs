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

//! What the context answers of itself: where the pointer is drawn, and
//! whether mirrors show their own colours.

use super::Context;

impl Context {
    /// The pointer is on the wire driver's cursor plane, not composited.
    pub fn hw_cursor(&self) -> bool {
        // Extended, the pointer can be on any output: composited.
        // Magnified, the arrow is drawn into the canvas and magnified with it.
        let lens = self.looks.access[0].magnified();
        self.wire.is_some_and(|w| w.cursor_offset != 0) && self.plan.is_none() && !lens
    }

    /// Mirrors show their own colours over a true-colour canvas.
    pub fn own_colours(&self) -> bool {
        let extended = self.want_extend || self.plan.is_some();
        let heads = self.heads.len();
        crate::display::mirror_rule::own_colours(extended, &self.looks.access, heads)
    }
}
