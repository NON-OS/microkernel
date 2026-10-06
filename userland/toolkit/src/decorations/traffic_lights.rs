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

use super::frame_rect::light_rect_at;
use super::palette::{LIGHT_CLOSE, LIGHT_GLYPH, LIGHT_MAXIMIZE, LIGHT_MINIMIZE};
use super::scale::{at, ONE};
use crate::paint::PaintBuffer;

const LIGHTS: [u32; 3] = [LIGHT_CLOSE, LIGHT_MINIMIZE, LIGHT_MAXIMIZE];

pub fn draw_traffic_lights(fb: &mut PaintBuffer, w: u32, h: u32, maximized: bool, hover: bool) {
    draw_traffic_lights_at(fb, w, h, maximized, hover, ONE);
}

pub fn draw_traffic_lights_at(
    fb: &mut PaintBuffer,
    w: u32,
    h: u32,
    maximized: bool,
    hover: bool,
    quarters: u32,
) {
    for (i, argb) in LIGHTS.iter().enumerate() {
        let r = light_rect_at(i as u32, w, h, maximized, quarters);
        let rad = r.w / 2;
        fb.circle(r.x + rad, r.y + rad, rad, *argb);
        if hover {
            draw_glyph(fb, i as u32, r.x + rad, r.y + rad, quarters);
        }
    }
}

/* The marks inside the buttons grow with them: arms by the scale, and the
 * stroke one pixel thicker per whole step, so a 2x button does not carry a
 * hairline mark half its weight. */
fn draw_glyph(fb: &mut PaintBuffer, i: u32, cx: u32, cy: u32, q: u32) {
    let (x, y) = (cx as i32, cy as i32);
    let a = at(2, q) as i32;
    let long = at(3, q) as i32;
    for k in 0..at(1, q) as i32 {
        match i {
            0 => {
                fb.line(x - a + k, y - a, x + a + k, y + a, LIGHT_GLYPH);
                fb.line(x + a + k, y - a, x - a + k, y + a, LIGHT_GLYPH);
            }
            1 => fb.line(x - long, y + k, x + long, y + k, LIGHT_GLYPH),
            _ => {
                fb.line(x - a, y - a + k, x + a, y - a + k, LIGHT_GLYPH);
                fb.line(x - a + k, y - a, x - a + k, y + a, LIGHT_GLYPH);
                fb.line(x - a, y + a - k, x + a, y + a - k, LIGHT_GLYPH);
                fb.line(x + a - k, y - a, x + a - k, y + a, LIGHT_GLYPH);
            }
        }
    }
}
