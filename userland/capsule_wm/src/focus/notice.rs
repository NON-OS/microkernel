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
//! When lifecycle subscribers hear that focus moved: whenever the focused
//! window after a request (or a sweep of dead processes) is not the one
//! before it, whatever moved it. The desktop shell names the focused app in
//! its menu bar; it heard only opens and closes, so a raise by a click, the
//! dock or a chord left it naming the app before. Pure; held in wm_proofs.

use super::model::FocusedRef;

/// The window focused now, as (pid, window id), when it is not `was`'s;
/// (0, 0) when nothing has focus now.
pub fn focus_notice(was: Option<FocusedRef>, now: Option<FocusedRef>) -> Option<(u32, u32)> {
    (was != now).then(|| now.map_or((0, 0), |f| (f.owner_pid, f.window_id)))
}
