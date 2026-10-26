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

//! The row itself, its three columns laid out, and its uptime tile.

use nonos_app_skeleton::PaintBuffer;

use crate::about::data::uptime::read_millis;
use crate::about::ui::gauge::{self, Reading};
use crate::about::ui::metrics::TILE_GAP;

use super::super::tile_text::uptime;
use super::caps::{caps_tile, display_tile};

const DAY_S: u64 = 86_400;

// The third column absorbs the rounding remainder so the row ends flush with the
// cards above and below it rather than a pixel short.
pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32, held: Option<u64>) {
    if y < 0 || y + gauge::HEIGHT as i32 > fb.height as i32 {
        return;
    }
    let y = y as u32;
    let col = (w.saturating_sub(TILE_GAP * 2)) / 3;
    let last = w.saturating_sub((col + TILE_GAP) * 2);
    uptime_tile(fb, 0, y, col);
    caps_tile(fb, col + TILE_GAP, y, col, held);
    display_tile(fb, (col + TILE_GAP) * 2, y, last);
}

// The ring is the position inside the current day, which is the only fraction the
// clock can honestly offer: there is no total to divide an uptime by.
fn uptime_tile(fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let mut value = [0u8; 24];
    let r = match read_millis() {
        Some(ms) => {
            let v = uptime(ms, &mut value);
            Reading { label: b"Uptime", value: v, sub: b"since this boot", num: (ms / 1000) % DAY_S, den: DAY_S }
        }
        None => Reading {
            label: b"Uptime",
            value: b"unavailable",
            sub: b"monotonic clock",
            num: 0,
            den: 1,
        },
    };
    gauge::tile(fb, x, y, w, &r);
}
