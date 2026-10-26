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

use crate::settings::schema::blocks_for;
use crate::settings::state::State;

use super::metrics::{CARD_GAP, HEAD_H, PANE_PAD_TOP};
use super::walk_list::list_of;
use super::walk_metrics::{block_h, head_h, row_h};

#[derive(Clone, Copy)]
pub enum Item {
    Head,
    Card,
    CardHead(usize),
    Row(usize, usize),
    Network(usize),
    Saved(usize),
    Wallpaper(u8),
    BtDevice(usize),
    Station,
}

/// The one place section geometry is defined. The painter and the hit test both
/// walk it, so a click can never land on pixels a different formula drew.
pub fn walk<F: FnMut(u32, u32, Item)>(state: &State, mut f: F) {
    let mut y = PANE_PAD_TOP;
    f(y, HEAD_H, Item::Head);
    y += HEAD_H;
    for (bi, b) in blocks_for(state.section).iter().enumerate() {
        let bh = block_h(state, b);
        f(y, bh, Item::Card);
        f(y, head_h(state, b), Item::CardHead(bi));
        let mut ry = y + head_h(state, b);
        for (ri, r) in b.rows.iter().enumerate() {
            let Some((n, item, h)) = list_of(state, r) else {
                f(ry, row_h(r), Item::Row(bi, ri));
                ry += row_h(r);
                continue;
            };
            for i in 0..n {
                f(ry, h, item(i));
                ry += h;
            }
        }
        y += bh + CARD_GAP;
    }
}
