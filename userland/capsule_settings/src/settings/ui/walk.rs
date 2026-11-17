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

use crate::settings::schema::blocks_for;
use crate::settings::schema::rows::Row;
use crate::settings::state::State;

use super::metrics::metrics;
use super::walk_metrics::{block_h, head_h, network_rows, row_h, saved_rows};

#[derive(Clone, Copy)]
pub enum Item {
    Head,
    Card(usize),
    CardHead(usize),
    Row(usize, usize),
    Network(usize),
    Saved(usize),
    Wallpaper(u8),
}

/// The one place section geometry is defined. The painter and the hit test both
/// walk it, so a click can never land on pixels a different formula drew.
pub fn walk<F: FnMut(u32, u32, Item)>(state: &State, mut f: F) {
    let m = metrics();
    let (row, mut y) = (m.row_h, m.pane_pad_top);
    f(y, m.head_h, Item::Head);
    y += m.head_h;
    for (bi, b) in blocks_for(state.section).iter().enumerate() {
        let bh = block_h(state, b);
        f(y, bh, Item::Card(bi));
        f(y, head_h(b), Item::CardHead(bi));
        let mut ry = y + head_h(b);
        for (ri, r) in b.rows.iter().enumerate() {
            match r {
                Row::Networks => {
                    for i in 0..network_rows(state) {
                        f(ry, row, Item::Network(i));
                        ry += row;
                    }
                }
                Row::Saved => {
                    for i in 0..saved_rows(state) {
                        f(ry, row, Item::Saved(i));
                        ry += row;
                    }
                }
                Row::Wallpapers => {
                    for i in 0..WALLPAPER_LABELS.len() {
                        f(ry, row, Item::Wallpaper(i as u8));
                        ry += row;
                    }
                }
                other => {
                    f(ry, row_h(other), Item::Row(bi, ri));
                    ry += row_h(other);
                }
            }
        }
        y += bh + m.card_gap;
    }
}

pub fn content_h(state: &State) -> u32 {
    let m = metrics();
    let mut h = m.pane_pad_top + m.head_h;
    for b in blocks_for(state.section) {
        h += block_h(state, b) + m.card_gap;
    }
    h + m.pane_pad_top
}
