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

//! The rows that are lists, a row an entry: how many each has now, and the
//! item each entry walks as. The walker, the heights and the hit test all
//! read this, so a list's length is counted in one place.

use nonos_policy_proto::wallpaper_labels::WALLPAPER_LABELS;

use crate::settings::schema::rows::Row;
use crate::settings::state::State;

use super::walk::Item;
use super::walk_metrics::{bt_rows, network_rows, saved_rows};

/// How many entries a list has, and the item the walker gives each one.
pub type List = (usize, fn(usize) -> Item);

pub fn list_of(state: &State, r: &Row) -> Option<List> {
    match r {
        Row::Networks => Some((network_rows(state), Item::Network)),
        Row::Saved => Some((saved_rows(state), Item::Saved)),
        Row::Wallpapers => Some((WALLPAPER_LABELS.len(), |i| Item::Wallpaper(i as u8))),
        Row::BtDevices => Some((bt_rows(state), Item::BtDevice)),
        Row::Field(_) | Row::Live(..) | Row::Access(_) => None,
    }
}
