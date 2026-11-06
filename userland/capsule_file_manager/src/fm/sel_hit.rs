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

use super::clipboard::yank;
use super::duplicate::duplicate;
use super::prompt_start::start_prompt;
use super::sel_model::{SelAction, ACTIONS};
use super::selection_clear::clear;
use super::state::{PromptKind, State};

// Move is a cut awaiting a paste, which is the only move this crate has: the
// vfs client exposes rename within a directory and copy across one, and the
// cut/paste pair is what already drives a cross-directory move.
pub fn run(state: &mut State, index: usize) -> EventOutcome {
    let Some(&(action, _, _)) = ACTIONS.get(index) else { return EventOutcome::Idle };
    match action {
        SelAction::Move => yank(state, true),
        SelAction::Duplicate => duplicate(state),
        SelAction::Tag => return start_prompt(state, PromptKind::Tag, b"tag: "),
        SelAction::Delete => {
            return start_prompt(state, PromptKind::Delete, b"delete? type y + Enter: ")
        }
        SelAction::Clear => {
            clear(state);
            state.status = b"selection cleared";
        }
    }
    EventOutcome::Repaint
}
