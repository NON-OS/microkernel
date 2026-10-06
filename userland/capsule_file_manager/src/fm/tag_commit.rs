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

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::selection_acting::acting;
use super::state::State;
use super::store_meta::save_meta;
use super::tags_toggle::toggle_tag;

/// The tag prompt's Enter: the selection's band offers Tag, so the tag goes
/// on every selected entry (or the one under the cursor), not only the row
/// the cursor sits on as it did. When all of them carry it already, it comes
/// off all of them.
pub fn tag_commit(state: &mut State, name: &str) {
    let paths: Vec<String> = acting(state).into_iter().map(|(p, _)| p).collect();
    if paths.is_empty() {
        state.status = b"no selection";
        return;
    }
    state.status = toggle_tag(&mut state.tags, &paths, name);
    save_meta(state);
}
