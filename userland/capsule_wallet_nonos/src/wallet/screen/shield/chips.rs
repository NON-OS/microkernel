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

/*
 * A row of choices where exactly one may be picked: the asset, or one of the
 * standard sizes. Four to a row, each a press the click table knows by its
 * index, so the size picked is the size that was drawn under the pointer.
 */

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::text::{draw_in, line, width};
use crate::wallet::etna::tokens::{CORNER, INK, OUTLINE, TEXT, TEXT_2, TIGHT};
use crate::wallet::etna::Role;
use crate::wallet::screen::hits::{self, Press};

const PER_ROW: u32 = 4;
const CHIP_H: u32 = 40;

fn chip(fb: &mut PaintBuffer, at: Rect, label: &str, picked: bool) {
    if picked {
        fb.fill_round(at.x, at.y, at.w, at.h, CORNER, TEXT);
    } else {
        fb.stroke_round(at.x, at.y, at.w, at.h, CORNER, 1, OUTLINE);
    }
    let ink = if picked { INK } else { TEXT_2 };
    let tx = at.x as i32 + (at.w as i32 - width(Role::RowValue, label)) / 2;
    let ty = at.y as i32 + (at.h as i32 - line(Role::RowValue)) / 2;
    draw_in(fb, tx, ty, Role::RowValue, label, ink);
}

/* Lays the labels out and returns the height used. */
pub fn chips(
    fb: &mut PaintBuffer,
    c: Rect,
    y: u32,
    labels: &[&str],
    picked: Option<u8>,
    press: fn(u8) -> Press,
) -> u32 {
    let per = PER_ROW.min(labels.len() as u32).max(1);
    let cw = (c.w - TIGHT * (per - 1)) / per;
    for (i, label) in labels.iter().enumerate() {
        let (col, row) = (i as u32 % per, i as u32 / per);
        let at = Rect::new(c.x + col * (cw + TIGHT), y + row * (CHIP_H + TIGHT), cw, CHIP_H);
        chip(fb, at, label, picked == Some(i as u8));
        hits::put(press(i as u8), at);
    }
    let rows = (labels.len() as u32).div_ceil(per);
    rows * CHIP_H + rows.saturating_sub(1) * TIGHT
}
