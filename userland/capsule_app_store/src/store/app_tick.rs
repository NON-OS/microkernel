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

//! One tick of the store: the catalogue, a moving install, the fetcher, what is unasked.

use super::super::fill_step::Step;
use super::super::state::State;

pub(super) fn tick(state: &mut State) -> bool {
    /* A store opened before the market registered asks again, a tick
     * at a time, rather than holding "not running yet" until r. */
    if !state.loaded || state.market_not_up() {
        state.refresh();
        return true;
    }
    let moved = state.any_pending() && state.poll_pending();
    let fetched = state.poll_fetch() | state.watch_order() | state.watch_proof();
    if moved {
        // "install requested" is stale once the system has said more.
        state.asked = None;
    }
    let filled = state.fill() == Step::Shown;
    moved || fetched || filled
}
