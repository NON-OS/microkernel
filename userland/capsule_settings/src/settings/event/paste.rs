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

//! Ctrl+V and Shift+Insert: the clipboard's first line into the field that
//! has the keyboard, by that field's own rule.

use nonos_app_skeleton::{clipboard_paste_line, EventOutcome};
use nonos_policy_proto::{str_max_of, STR_MAX};
use nonos_wifi_client::wipe;

use crate::settings::section::Section;
use crate::settings::state::cache::STRING_CAP;
use crate::settings::state::edit_paste::{value_char, Pasted};
use crate::settings::state::status::StatusKind;
use crate::settings::state::{current_field, State};

use super::on_search_key::QUERY_MAX;

/// More than any field here holds, so a longer line is seen to be cut.
const CLIP_MAX: usize = 256;

#[derive(Clone, Copy)]
enum Target {
    Search,
    Value,
    Passphrase,
}

fn target_of(state: &State) -> Option<Target> {
    if state.search_focused {
        Some(Target::Search)
    } else if state.editing {
        Some(Target::Value)
    } else if state.section == Section::Wifi && state.wifi_pass_active {
        Some(Target::Passphrase)
    } else {
        None
    }
}

/// None when no field has the keyboard, so the key goes on as any other.
pub(super) fn on_paste(state: &mut State) -> Option<EventOutcome> {
    let target = target_of(state)?;
    let mut buf = [0u8; CLIP_MAX];
    let pasted = match clipboard_paste_line(&mut buf) {
        Ok(Some(line)) => Ok(into(state, target, line.text)),
        Ok(None) => Ok(Pasted::Empty),
        Err(_) => Err(()),
    };
    // The line may be a passphrase; it does not stay on the stack.
    wipe(&mut buf);
    let note: &[u8] = match (pasted, target) {
        (Err(()), _) => b"paste: the clipboard is not available",
        (Ok(Pasted::Whole), _) => return Some(EventOutcome::Repaint),
        (Ok(Pasted::Empty), _) => b"paste: the clipboard holds no text",
        (Ok(Pasted::Cut), _) => b"paste: cut to what the field holds",
        (Ok(Pasted::Refused), Target::Passphrase) => {
            b"paste refused: a passphrase is printable ASCII"
        }
        (Ok(Pasted::Refused), _) => {
            b"paste refused: letters, digits, dot, dash and underscore only"
        }
    };
    state.status.set(StatusKind::Error, note);
    Some(EventOutcome::Repaint)
}

fn into(state: &mut State, target: Target, line: &str) -> Pasted {
    match target {
        Target::Search => {
            let out = state.search.paste(line, STRING_CAP, QUERY_MAX, |_| true);
            state.search_cursor = 0;
            state.search_scroll = 0;
            out
        }
        Target::Value => {
            // The store refuses a value longer than its field takes.
            let cap = current_field(state).map_or(STR_MAX, str_max_of);
            state.edit.paste(line, cap, usize::MAX, value_char)
        }
        Target::Passphrase => {
            state.wifi_pass.paste(line, STRING_CAP, usize::MAX, |c| matches!(c, ' '..='~'))
        }
    }
}
