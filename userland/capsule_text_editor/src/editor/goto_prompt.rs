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

//! Go to a line: move the caret to the line the prompt names.

use nonos_app_skeleton::EventOutcome;

use super::state::State;

/// Move the caret to the line the prompt names, and scroll so it is visible.
pub(super) fn goto(state: &mut State, len: usize) -> EventOutcome {
    let Some(n) = super::goto_line::parse_line_number(&state.prompt_path[..len]) else {
        state.status = b"not a line number";
        return EventOutcome::Repaint;
    };
    state.caret = super::goto_line::offset_of_line(&state.buf[..state.len], n);
    let rows = state.visible_rows;
    super::follow_caret::follow_caret(state, rows);
    state.status = b"jumped";
    EventOutcome::Repaint
}
