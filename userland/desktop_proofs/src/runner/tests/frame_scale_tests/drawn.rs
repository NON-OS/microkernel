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

//! What is drawn is where the geometry says, the title inside its bar.

use super::rig::{paint, title_rows, H, SCALES, W};
use nonos_toolkit::decorations::{
    content_rect_at, light_rect_at, HAIRLINE, LIGHT_CLOSE, LIGHT_MAXIMIZE, LIGHT_MINIMIZE,
};

#[test]
fn what_is_drawn_is_where_the_geometry_says() {
    let lights = [LIGHT_CLOSE, LIGHT_MINIMIZE, LIGHT_MAXIMIZE];
    for q in SCALES {
        let px = paint(q, b"");
        for (i, argb) in lights.iter().enumerate() {
            let r = light_rect_at(i as u32, W, H, false, q);
            let centre = px[((r.y + r.h / 2) * W + r.x + r.w / 2) as usize];
            assert_eq!(centre, *argb, "button {i} painted at its centre at {q}");
        }
        // The hairline under the bar is the content's top edge.
        let c = content_rect_at(W, H, false, q);
        assert_eq!(px[(c.y * W + W / 2) as usize], HAIRLINE, "the bar's hairline at {q}");
    }
}

#[test]
fn the_title_grows_with_the_bar_and_stays_inside_it() {
    let (t1, b1) = title_rows(4);
    let (t2, b2) = title_rows(8);
    let (h1, h2) = (b1 - t1 + 1, b2 - t2 + 1);
    assert!(h2 * 10 >= h1 * 17 && h2 * 10 <= h1 * 23, "type at 2x is about twice: {h1} then {h2}");
    for q in SCALES {
        title_rows(q);
    }
}
