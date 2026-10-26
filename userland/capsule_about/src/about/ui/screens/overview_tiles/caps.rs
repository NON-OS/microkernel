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

//! The overview's capabilities and display tiles.

use nonos_app_skeleton::PaintBuffer;

use crate::about::data::caps::{granted, ALL_CAPS};
use crate::about::data::display::primary_dimensions;
use crate::about::ui::gauge::{self, Reading};

use super::super::tile_text::{dims, ratio};

// The word the kernel recorded for this pid, counted against every named
// capability. A table that did not answer says so rather than counting the
// bits the build declared.
pub fn caps_tile(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, held: Option<u64>) {
    let total = ALL_CAPS.len() as u64;
    let label = b"Capabilities";
    let Some(mask) = held else {
        let r = Reading { label, value: b"unavailable", sub: b"process table", num: 0, den: 1 };
        gauge::tile(fb, x, y, w, &r);
        return;
    };
    let count = granted(mask);
    let mut value = [0u8; 24];
    let v = ratio(count, total, &mut value);
    let r = Reading { label, value: v, sub: b"held by this window", num: count, den: total };
    gauge::tile(fb, x, y, w, &r);
}

pub fn display_tile(fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let mut value = [0u8; 24];
    let sub = b"primary display";
    let r = match primary_dimensions() {
        Some((dw, dh)) => {
            let v = dims(dw, dh, &mut value);
            Reading { label: b"Display", value: v, sub, num: 1, den: 1 }
        }
        None => Reading { label: b"Display", value: b"unavailable", sub, num: 0, den: 1 },
    };
    gauge::tile(fb, x, y, w, &r);
}
