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

//! Where a paste goes: the field that has the keyboard (the search, a
//! value being edited, or the Wi-Fi passphrase), and the line put into it
//! by that field's own rule.

use super::on_search_key::QUERY_MAX;
use crate::settings::section::Section;
use crate::settings::state::cache::STRING_CAP;
use crate::settings::state::edit_paste::{value_char, Pasted};
use crate::settings::state::{current_field, State};
use nonos_policy_proto::{str_max_of, STR_MAX};

#[derive(Clone, Copy)]
pub(super) enum Target {
    Search,
    Value,
    Passphrase,
}

pub(super) fn target_of(state: &State) -> Option<Target> {
    if state.search.focused {
        Some(Target::Search)
    } else if state.editing {
        Some(Target::Value)
    } else if state.section == Section::Wifi && state.wifi_pass.active {
        Some(Target::Passphrase)
    } else {
        None
    }
}

pub(super) fn into(state: &mut State, target: Target, line: &str) -> Pasted {
    match target {
        Target::Search => {
            let out = state.search.text.paste(line, STRING_CAP, QUERY_MAX, |_| true);
            state.search.cursor = 0;
            state.search.scroll = 0;
            out
        }
        Target::Value => {
            // The store refuses a value longer than its field takes.
            let cap = current_field(state).map_or(STR_MAX, str_max_of);
            state.edit.paste(line, cap, usize::MAX, value_char)
        }
        Target::Passphrase => {
            state.wifi_pass.text.paste(line, STRING_CAP, usize::MAX, |c| matches!(c, ' '..='~'))
        }
    }
}
