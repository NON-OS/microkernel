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
use super::tokens::{BANNER_H, COLUMN, INK};

/// One decoded photograph at a time: the section on screen.
static DECODED: Mutex<Option<(Backdrop, Vec<u32>)>> = Mutex::new(None);

fn fade(row: u32) -> u32 {
    let t = row * 1000 / BANNER_H;
    let alpha = match t {
        0..=299 => 350 * (300 - t) / 300,
        300..=549 => 0,
        _ => 1000 * (t - 550) / 450,
    };
    alpha.min(1000) * 255 / 1000
}

/// Draw `which` with its top-left at `x, y`; returns the band's height.
pub fn band(fb: &mut PaintBuffer, x: u32, y: u32, which: Backdrop) -> u32 {
    let mut slot = DECODED.lock();
    if slot.as_ref().map(|(b, _)| *b) != Some(which) {
        let mut px = alloc::vec![0u32; (COLUMN * BANNER_H) as usize];
        let ok = decode_png_argb8888(which.png(), &mut px).is_ok_and(|s| s.width == COLUMN);
        *slot = ok.then_some((which, px));
    }
    for row in 0..BANNER_H {
        if let Some((_, px)) = slot.as_ref() {
            let from = (row * COLUMN) as usize;
            for col in 0..COLUMN {
                fb.blend_px(x + col, y + row, px[from + col as usize] | 0xFF00_0000);
            }
        } else {
            fb.fill_rect(x, y + row, COLUMN, 1, INK);
        }
        fb.blend_rect(x, y + row, COLUMN, 1, (fade(row) << 24) | (INK & 0x00FF_FFFF));
    }
    BANNER_H
}
