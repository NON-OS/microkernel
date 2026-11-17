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

use nonos_app_skeleton::EventOutcome;

use super::layout::{content_w, content_x};
use super::list_cols::cols;
use super::state::State;
use super::store_meta::save_meta;
use super::view::rebuild_view;

/// Select a sort directly, rather than cycling the way the header pill does.
/// Writes through to `prefs` for the same reason `event_head` does: the drawn
/// caret and the persisted preference must not be able to disagree. `col`
/// indexes the header strip's columns, as `browse_hit` names them.
pub fn sort_by(state: &mut State, col: usize) -> EventOutcome {
    let c = cols(content_x(), content_w(state.win_w));
    let Some(mode) = c.cols.get(col).map(|h| h.mode) else { return EventOutcome::Idle };
    state.sort_mode = mode;
    state.prefs.sort = mode;
    rebuild_view(state);
    save_meta(state);
    EventOutcome::Repaint
}
