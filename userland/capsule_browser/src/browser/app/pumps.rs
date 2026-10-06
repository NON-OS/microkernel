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

use crate::browser::state::State;

use crate::browser::omnibox::Change;

/* Every fetch steps, lands and starts here: the navigation's own slot and
 * the pool of sub-resource fetches side by side. A page that changed is
 * marked for a whole repaint. */
pub(super) fn fetch_tick(state: &mut State) {
    if crate::browser::fetch::tick(state) {
        state.mark(Change::Full);
    }
}
