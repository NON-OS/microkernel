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

use nonos_app_skeleton::EventOutcome;

use super::clipboard::yank;
use super::duplicate::duplicate;
use super::event_open::open_selected;
use super::favorite_toggle::toggle_favorite;
use super::info_quick::{Quick, QUICKS};
use super::perms::toggle_readonly;
use super::prompt_start::start_prompt;
use super::state::{PromptKind, State};
use super::store_meta::save_meta;
use super::target::Target;

/// Acts on an info panel target: a tile, a tag's remove box, the add chip or
/// the permissions toggle. A tag is named by its index, and its name is read
/// back here because the removal mutates the map the name lives in.
pub fn run_info(state: &mut State, t: Target) -> EventOutcome {
    match t {
        Target::Quick(i) => match QUICKS.get(i) {
            Some(&(action, _)) => run_quick(state, action),
            None => EventOutcome::Idle,
        },
        Target::Untag(i) => {
            let Some(entry) = state.entries.get(state.cursor) else {
                return EventOutcome::Idle;
            };
            let names = state.tags.tags_for(entry.full_path.as_str());
            let Some(name) = names.get(i).map(|n| String::from(*n)) else {
                return EventOutcome::Idle;
            };
            remove_tag(state, name.as_str())
        }
        Target::AddTag => start_prompt(state, PromptKind::Tag, b"tag: "),
        Target::Perms => {
            toggle_readonly(state);
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}

/// Every tile lands on a handler that already exists; the tiles carry no labels,
/// so each handler's own status line is what names the action after the fact.
pub fn run_quick(state: &mut State, action: Quick) -> EventOutcome {
    match action {
        Quick::Open => return open_selected(state),
        Quick::Copy => yank(state, false),
        Quick::Duplicate => duplicate(state),
        Quick::Pin => toggle_favorite(state),
        Quick::Tag => return start_prompt(state, PromptKind::Tag, b"tag: "),
        Quick::Delete => {
            return start_prompt(state, PromptKind::Delete, b"delete? type y + Enter: ")
        }
    }
    EventOutcome::Repaint
}

/// Drops one tag from the cursor path and writes the sidecar back, the same way
/// `tag_commit` does when the typed name was already there.
pub fn remove_tag(state: &mut State, name: &str) -> EventOutcome {
    let Some(entry) = state.entries.get(state.cursor) else {
        return EventOutcome::Idle;
    };
    let path = entry.full_path.clone();
    state.tags.remove(path.as_str(), name);
    save_meta(state);
    state.status = b"untagged";
    EventOutcome::Repaint
}
