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

//! The photograph at the head of a section, full width under the bar, then
//! the fade the phones lay over it: ink at 35% at the top, clear from 30% to
//! 55%, solid ink at the foot, so it melts into the ground.

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::png::decoder::decode_png_argb8888;
use spin::Mutex;

use super::backdrop::Backdrop;
use super::layout::PHOTO_W;
use super::tokens::{BANNER_H, INK};

/// One decoded photograph at a time: the section on screen.
static DECODED: Mutex<Option<(Backdrop, Vec<u32>)>> = Mutex::new(None);

fn fade(row: u32, h: u32) -> u32 {
    let t = row * 1000 / h.max(1);
    let alpha = match t {
        0..=299 => 350 * (300 - t) / 300,
        300..=549 => 0,
        _ => 1000 * (t - 550) / 450,
    };
    alpha.min(1000) * 255 / 1000
}

/// Draw `which` `h` rows tall with its top-left at `x, top`, rows above the
/// screen left out, at the column's width: the photograph is scaled to it,
/// keeping its proportions. A band shorter than the scaled photograph shows
/// its middle, which is where each photograph's eruption sits.
pub fn band(fb: &mut PaintBuffer, x: u32, top: i64, which: Backdrop, h: u32) -> u32 {
    let column = super::current::now().column.max(1);
    let tall = BANNER_H * column / PHOTO_W;
    let h = h.min(tall);
    let skip = (tall - h) / 2;
    let mut slot = DECODED.lock();
    if slot.as_ref().map(|(b, _)| *b) != Some(which) {
        let mut px = alloc::vec![0u32; (PHOTO_W * BANNER_H) as usize];
        let ok = decode_png_argb8888(which.png(), &mut px).is_ok_and(|s| s.width == PHOTO_W);
        *slot = ok.then_some((which, px));
    }
    for row in 0..h {
        let Ok(y) = u32::try_from(top + i64::from(row)) else {
            continue;
        };
        if let Some((_, px)) = slot.as_ref() {
            /* The nearest photograph pixel to each window pixel. */
            let src_row = ((row + skip) * PHOTO_W / column).min(BANNER_H - 1);
            let from = (src_row * PHOTO_W) as usize;
            for col in 0..column {
                let src_col = (col * PHOTO_W / column).min(PHOTO_W - 1);
                fb.blend_px(x + col, y, px[from + src_col as usize] | 0xFF00_0000);
            }
        } else {
            fb.fill_rect(x, y, column, 1, INK);
        }
        fb.blend_rect(x, y, column, 1, (fade(row, h) << 24) | (INK & 0x00FF_FFFF));
    }
    h
}
