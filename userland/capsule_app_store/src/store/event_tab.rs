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
//! Walking the tab strip with the arrow keys.

use super::state::{State, TABS};

pub(super) fn step_tab(state: &mut State, delta: isize) -> bool {
    let at = TABS.iter().position(|t| *t == state.tab).unwrap_or(0) as isize;
    let want = (at + delta).clamp(0, TABS.len() as isize - 1) as usize;
    state.set_tab(TABS[want])
}
