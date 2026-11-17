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

//! Every control the window shows, in the box its painter draws it in. The
//! click and the hover both search this one table, so a click lands on what
//! was drawn.

use alloc::vec::Vec;

use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::Rect;

use super::convert_geom::{chip, row, swap, CHIPS};
use super::convert_hit::ConvertHit;
use super::{bits_geom, history_geom, keypad_geom, nav_geom};
use crate::calc::convert::list;
use crate::calc::hit::Hit;
use crate::calc::mode::{Mode, MODES};
use crate::calc::prog::BITS;
use crate::calc::state::State;

/// The rail's modes, then the controls of the mode on show, each with its box.
/// The History page names only the rows that hold a calculation.
pub fn targets(state: &State) -> Vec<(Hit, Rect)> {
    let (w, h) = state.view;
    let mut out: Vec<(Hit, Rect)> =
        MODES.iter().enumerate().map(|(i, m)| (Hit::Rail(*m), nav_geom::row(i))).collect();
    match state.mode {
        Mode::Convert => {
            out.extend((0..CHIPS).map(|i| (Hit::Convert(ConvertHit::Chip(i)), chip(w, i))));
            out.push((Hit::Convert(ConvertHit::Swap), swap(w)));
            for i in 0..list(state.cat).len() {
                out.push((Hit::Convert(ConvertHit::From(i)), row(w, true, i)));
                out.push((Hit::Convert(ConvertHit::To(i)), row(w, false, i)));
            }
        }
        Mode::History => {
            let held = |i: &usize| {
                state.history.get(history_geom::entry_at(state.history_scroll, *i)).is_some()
            };
            let rows = (0..history_geom::capacity(h)).filter(held);
            out.extend(rows.map(|i| (Hit::Row(i), history_geom::row(w, i))));
        }
        mode => {
            if mode == Mode::Programmer {
                out.extend((0..BITS).map(|b| (Hit::Bit(b), bits_geom::cell(w, b))));
            }
            let keys = keypad_geom::keys(mode, w, h);
            out.extend(keys.map(|(r, i, _, at)| (Hit::Key(r, i), at)));
        }
    }
    out
}

/// The control at `(x, y)`, if any.
pub fn at(state: &State, x: i32, y: i32) -> Option<Hit> {
    targets(state).into_iter().find(|&(_, r)| contains(r, x, y)).map(|(t, _)| t)
}
