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

//! Whether a window has room, and spending from it.

use super::super::state::Manager;

/*
 * Both windows have to have room. A cell sent past either is one the relay is
 * entitled to drop, and a dropped cell desynchronises the digest chain for
 * everything after it on that hop.
 */
pub(super) fn room(state: &Manager, index: usize, id: u16) -> bool {
    let Some(target) = state.circuits[index].hops.len().checked_sub(1) else {
        return false;
    };
    if state.circuits[index].hops[target].package_window <= 0 {
        return false;
    }
    state.streams.iter().any(|s| s.id == id && s.package_window > 0)
}

pub(super) fn spend(state: &mut Manager, index: usize, id: u16) {
    if let Some(target) = state.circuits[index].hops.len().checked_sub(1) {
        state.circuits[index].hops[target].package_window -= 1;
    }
    if let Some(stream) = state.streams.iter_mut().find(|s| s.id == id) {
        stream.package_window -= 1;
    }
}
