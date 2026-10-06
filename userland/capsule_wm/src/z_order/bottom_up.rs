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

use crate::window::{Visibility, WindowTable};

/// Hand `lift` each process with a window on screen, bottom of the stack
/// first, each once, at the place of its highest window: lifting them in this
/// order gives the compositor, which stacks layers by process, the order
/// this table has. Stops early when `lift` returns false.
pub fn bottom_up(table: &WindowTable, mut lift: impl FnMut(u32) -> bool) {
    let shown = || table.windows().filter(|w| w.visibility == Visibility::Visible);
    let top_of = |pid: u32| shown().filter(|w| w.owner_pid == pid).map(|w| w.z).max();
    let mut last: Option<u32> = None;
    while let Some(w) = shown().filter(|w| last.is_none_or(|l| w.z > l)).min_by_key(|w| w.z) {
        last = Some(w.z);
        if top_of(w.owner_pid) == Some(w.z) && !lift(w.owner_pid) {
            return;
        }
    }
}
