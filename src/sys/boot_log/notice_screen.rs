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
 * A notice band across the top of the framebuffer, for a boot that stops on
 * purpose and must say why on the panel: the on-screen log is off by default
 * and a machine may have no serial console, so this is the one line a person
 * is sure to see. No allocation, no locks, every pixel write bounds-checked.
 */

use super::draw::{fill_rect, scale};
use super::panic_screen::draw_text;
use super::screen::screen;

const TOP: u32 = 20;
const PITCH: u32 = 28;
const BAND_COLOR: u32 = 0x0024_2C48;
const TITLE_COLOR: u32 = 0x00FF_E08A;
const TEXT_COLOR: u32 = 0x00FF_FFFF;

pub fn show_notice(title: &[u8], lines: &[&[u8]]) {
    let Some(fb) = screen() else {
        return;
    };
    let fb = &fb;
    let s = scale(fb);
    let (top, pitch) = (TOP * s, PITCH * s);
    let band = top * 2 + pitch * (lines.len() as u32 + 1);
    fill_rect(fb, 0, 0, fb.width, band.min(fb.height), BAND_COLOR);
    draw_text(fb, 24 * s, top, title, TITLE_COLOR);
    for (i, line) in lines.iter().enumerate() {
        draw_text(fb, 24 * s, top + pitch * (i as u32 + 1), line, TEXT_COLOR);
    }
}
