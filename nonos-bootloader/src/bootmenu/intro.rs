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

//! The menu's opening: the Ø assembles from particles, then its frame draws
//! itself, about two thirds of a second. It runs before the countdown starts
//! and after every check the menu reports, so it delays no check.

use uefi::table::boot::BootServices;

use super::brand::draw_emblem;
use super::layout::layout;
use crate::display::fx::clear_region;
use crate::display::ink::draw_particles;
use crate::display::ink::palette::CYAN;

const GATHER: u32 = 18;
const DRAW: u32 = 12;
const FRAME_US: usize = 16_000;

pub(super) fn intro(bs: &BootServices) {
    let l = &layout();
    let (x, y, w, h) = l.frame;
    let pad = w / 2 + l.u;
    let area = (x.saturating_sub(pad), y.saturating_sub(pad), w + 2 * pad, h + 2 * pad);
    let clear = || clear_region(area.0, area.1, area.2, area.3);
    for i in 0..=GATHER {
        clear();
        draw_particles(x + w / 2, y + h / 2, w / 2, i * 1000 / GATHER, CYAN);
        bs.stall(FRAME_US);
    }
    for i in 1..=DRAW {
        clear();
        draw_emblem(l, i * 1000 / DRAW);
        if i < DRAW {
            draw_particles(x + w / 2, y + h / 2, w / 2, 1000, CYAN);
        }
        bs.stall(FRAME_US);
    }
}
