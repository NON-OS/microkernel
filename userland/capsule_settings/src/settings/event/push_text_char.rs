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

use nonos_policy_proto::{str_max_of, STR_MAX};

use crate::settings::state::{current_field, State};

pub fn push_text_char(state: &mut State, ch: u32) -> bool {
    if !state.editing {
        return false;
    }
    let b = match ch {
        c @ 0x20..=0x7E => c as u8,
        _ => return false,
    };
    /* The store refuses a value longer than its field takes, so stop typing there. */
    let cap = current_field(state).map_or(STR_MAX, str_max_of);
    if !allowed(b) || state.edit.len >= cap {
        return false;
    }
    state.edit.push(b)
}

fn allowed(b: u8) -> bool {
    matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'.' | b'_')
}
