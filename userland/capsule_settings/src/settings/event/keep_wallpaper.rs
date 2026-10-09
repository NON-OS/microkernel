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

//! Keeping or dropping wallpapers from Settings.

use nonos_policy_proto::wallpaper_labels::WALLPAPER_LABELS;
use nonos_policy_proto::wallpapers_kept::{kept, ALL};
use nonos_policy_proto::Field;

use crate::settings::ipc::op_set_u64;
use crate::settings::state::status::StatusKind;
use crate::settings::state::{store_value, FieldValue, State};
use crate::settings::ui::wallpaper_row::{desktop, kept_set};

use super::report::report;

/// Keep wallpaper `index`, or drop it. The desktop's is never dropped, since
/// the desktop would then show one the session no longer reads.
pub(super) fn toggle(state: &mut State, index: u8) {
    if desktop(state) == Some(index) {
        state.status.set(StatusKind::Idle, b"the desktop's wallpaper is always kept");
        return;
    }
    let Some(set) = kept_set(state) else { return unread(state) };
    let set = set ^ (1u64 << index);
    put(state, set, if kept(set, index) { b"kept" } else { b"dropped" });
}

/// From the summary row: every wallpaper kept, or, when every one already
/// is, only the desktop's.
pub(super) fn keep_all_or_one(state: &mut State) {
    let Some(set) = kept_set(state) else { return unread(state) };
    if set != ALL {
        return put(state, ALL, b"every wallpaper kept");
    }
    let Some(d) = desktop(state).filter(|&d| (d as usize) < WALLPAPER_LABELS.len()) else {
        return;
    };
    put(state, 1u64 << d, b"only the desktop's wallpaper kept");
}

fn unread(state: &mut State) {
    state.status.set(StatusKind::Error, b"the kept wallpapers were not read; nothing changed");
}

fn put(state: &mut State, set: u64, said: &[u8]) {
    match op_set_u64(state.policy_port, Field::WallpapersKept, set) {
        Ok(()) => {
            store_value(state, Field::WallpapersKept, FieldValue::U64(set));
            state.status.set(StatusKind::Ok, said);
        }
        Err(e) => report(state, e),
    }
}
