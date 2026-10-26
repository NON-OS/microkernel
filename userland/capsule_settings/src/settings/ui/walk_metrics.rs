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

use crate::settings::schema::rows::{Block, Live, Row};
use crate::settings::state::State;

use super::field_note::note_of;
use super::metrics::{CARD_HEAD_H, ROW_H, ROW_NOTE_H};
use super::note_wrap;
use super::walk_list::list_of;

pub fn head_h(state: &State, b: &Block) -> u32 {
    b.note.map_or(CARD_HEAD_H, |n| note_wrap::head_h(n, state.win_w))
}

pub fn row_h(r: &Row) -> u32 {
    match r {
        Row::Field(f) if note_of(*f).is_some() => ROW_NOTE_H,
        Row::Live(_, Live::AudioOutput) => ROW_NOTE_H,
        _ => ROW_H,
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

/// The bonded devices, or one row saying there are none or why the list
/// could not be read.
pub fn bt_rows(state: &State) -> usize {
    state.bt.bond_count.max(1)
}

pub fn block_h(state: &State, b: &Block) -> u32 {
    let rows = b.rows.iter().map(|r| list_of(state, r).map_or(row_h(r), |l| l.2 * l.0 as u32));
    head_h(state, b) + rows.sum::<u32>()
}
