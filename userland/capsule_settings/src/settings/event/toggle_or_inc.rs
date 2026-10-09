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

use nonos_policy_proto::{kind_of, Field, KIND_BOOL, KIND_STR};

use crate::settings::schema::{read_only, Slot};
use crate::settings::state::status::StatusKind;
use crate::settings::state::{cached_value, current_field, current_slot, edit_start, FieldValue, State};

use super::adjust::adjust;
use super::adjust_tier::adjust_tier;
use super::commit_bool::commit_bool;
use super::keep_wallpaper;

pub fn toggle_or_inc(state: &mut State) {
    if let Some(Slot::Wallpaper(i)) = current_slot(state) {
        return keep_wallpaper::toggle(state, i);
    }
    let field = match current_field(state) {
        Some(f) => f,
        None => return,
    };
    if field == Field::WallpapersKept {
        return keep_wallpaper::keep_all_or_one(state);
    }
    // Status fields report what the system did; asserting one from a settings
    // row would state something untrue about the machine.
    if read_only(field) {
        state.status.set(StatusKind::Idle, b"reported by the system, not editable");
        return;
    }
    /* A choice among the pinned tiers, not text to type. */
    if field == Field::QwenTier {
        return adjust_tier(state, 1);
    }
    match kind_of(field) {
        KIND_BOOL => {
            let current = matches!(cached_value(state, field), FieldValue::Bool(true));
            commit_bool(state, field, !current);
        }
        KIND_STR => edit_start(state),
        _ => adjust(state, 1),
    }
}
