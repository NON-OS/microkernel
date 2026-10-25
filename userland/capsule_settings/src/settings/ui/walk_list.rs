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

use super::metrics::ROW_H;
use super::station_row::station_list;
use super::walk::Item;
use super::walk_metrics::{bt_rows, network_rows, saved_rows};

/// How many entries a list has, the item the walker gives each one, and
/// each entry's height.
pub type List = (usize, fn(usize) -> Item, u32);

pub fn list_of(state: &State, r: &Row) -> Option<List> {
    match r {
        Row::Networks => Some((network_rows(state), Item::Network, ROW_H)),
        Row::Saved => Some((saved_rows(state), Item::Saved, ROW_H)),
        Row::Wallpapers => Some((WALLPAPER_LABELS.len(), |i| Item::Wallpaper(i as u8), ROW_H)),
        Row::BtDevices => Some((bt_rows(state), Item::BtDevice, ROW_H)),
        Row::Station => Some(station_list(state)),
        Row::Field(_) | Row::Live(..) | Row::Access(_) => None,
    }
}
