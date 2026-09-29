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

use alloc::string::String;

use crate::browser::state::State;

/* The href of the link under viewport point (x, y), y = 0 at the first page
 * row, on the box-model page or the plain line document. */
pub(super) fn link_under(state: &State, x: i32, y: i32) -> Option<String> {
    let scroll = state.scroll as i32;
    match (state.box_doc.as_ref(), state.document.as_ref()) {
        (Some(b), _) => b.link_at(x, y, scroll).map(String::from),
        (None, Some(d)) => d.link_at(x, y.saturating_add(scroll)).map(String::from),
        (None, None) => None,
    }
}
