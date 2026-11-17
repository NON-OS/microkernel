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

use nonos_policy_proto::wallpaper_labels::WALLPAPER_LABELS;

use crate::settings::schema::rows::{Block, Live, Row};
use crate::settings::state::State;

use super::field_note::note_of;
use super::metrics::metrics;

pub fn head_h(b: &Block) -> u32 {
    let m = metrics();
    if b.note.is_some() {
        m.card_head_note_h
    } else {
        m.card_head_h
    }
}

pub fn row_h(r: &Row) -> u32 {
    let m = metrics();
    match r {
        Row::Field(f) if note_of(*f).is_some() => m.row_note_h,
        Row::Live(_, Live::AudioOutput) => m.row_note_h,
        _ => m.row_h,
    }
}

pub fn network_rows(state: &State) -> usize {
    state.wifi_network_count.max(1)
}

/// The saved networks, or one row saying there are none or why they could
/// not be read.
pub fn saved_rows(state: &State) -> usize {
    state.wifi.saved_count.max(1)
}

pub fn block_h(state: &State, b: &Block) -> u32 {
    let (mut h, row) = (head_h(b), metrics().row_h);
    for r in b.rows {
        h += match r {
            Row::Networks => row * network_rows(state) as u32,
            Row::Saved => row * saved_rows(state) as u32,
            Row::Wallpapers => row * WALLPAPER_LABELS.len() as u32,
            other => row_h(other),
        };
    }
    h
}
