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

//! The kernel's drawer in the takeover model (L79), drawing::drawable and
//! the Drop of what it hands out: the stop read, the draw counted, the stop
//! read again and the count backed out if it is set, the write, the count
//! dropped; `cfg.draws` tries a run. NoRecheck reads the stop only before.

use super::events::{BACKED_OUT, DREW};
use super::knob::Knob;
use super::world::END;
use super::world_state::World;

pub(super) fn drawer(w: &mut World, k: Knob) -> bool {
    let (tries, phase) = (w.pc[5] / 4, w.pc[5] % 4);
    let over = if tries + 1 < w.cfg.draws { (tries + 1) * 4 } else { END };
    w.pc[5] = match phase {
        0 if w.stopped() => over,
        0 => w.pc[5] + 1,
        1 => {
            w.draws += 1;
            w.pc[5] + if k == Knob::NoRecheck { 2 } else { 1 }
        }
        2 if w.stopped() => {
            (w.draws, w.events) = (w.draws - 1, w.events | BACKED_OUT);
            over
        }
        2 => w.pc[5] + 1,
        // The write ends: fenced, then the count dropped.
        _ => {
            (w.draws, w.events) = (w.draws - 1, w.events | DREW);
            over
        }
    };
    true
}
