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
//! The no-IOMMU warning as drawn: red, inside its row, in the owner's words.

use crate::layout_fit::layout;
use crate::warning::{draw, RED, TEXT};
use nonos_toolkit::font::atlas::FontAtlas;
use nonos_toolkit::font::render::draw_text;

const MODES: [(u32, u32); 7] =
    [(640, 480), (800, 600), (1024, 768), (1280, 720), (1280, 800), (1920, 1080), (3840, 2160)];

#[test]
fn drawn_in_red_inside_its_row_only() {
    for (w, h) in MODES {
        let mut buf = vec![0u32; (w * h) as usize];
        let r = layout(w, h).warn;
        draw(&mut buf, w as usize, w, h, r);
        let mut lit = 0;
        for (i, px) in buf.iter().enumerate() {
            let (x, y) = ((i as u32) % w, (i as u32) / w);
            if *px != 0 {
                assert!(y >= r.1 && y < r.1 + r.3, "{w}x{h}: pixel at row {y}");
                assert!(x > 0 && x < w - 1, "{w}x{h}: pixel at column {x}");
                lit += (*px == RED) as u32;
            }
        }
        assert!(lit > 50, "{w}x{h}: {lit} red pixels");
    }
    assert_eq!(TEXT, b"device DMA not confined by hardware");
}

#[test]
fn the_same_pixels_as_the_text_centred_at_the_rows_top() {
    let text = b"device DMA not confined by hardware";
    for (w, h) in MODES {
        let r = layout(w, h).warn;
        let (mut got, mut want) = (vec![0u32; (w * h) as usize], vec![0u32; (w * h) as usize]);
        draw(&mut got, w as usize, w, h, r);
        let x = (w - FontAtlas::default().text_width(text)) / 2;
        draw_text(&mut want, w as usize, w, h, x, r.1, text, 0xFFFF_3B3B);
        assert!(got == want, "{w}x{h}");
    }
}
