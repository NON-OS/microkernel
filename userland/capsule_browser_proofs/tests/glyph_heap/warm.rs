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

//! Repainting text already on screen touches no heap: glyphs come from the
//! cache, and a page face was parsed once when it was installed.

use capsule_browser_proofs::browser::fonts::{self, family_key, TextRun, NO_CLIP};
use nonos_toolkit::font::ttf::{builtin_face, draw_text_tracked};
use nonos_toolkit::paint::PaintBuffer;

use super::counting::measure;
use super::serial::serial;

const SANS: &[u8] = include_bytes!("../../../toolkit/assets/fonts/NotoSans-Regular.ttf");
const TEXT: &str = "Warm text draws from the glyph cache 0123456789";

#[test]
fn warm_page_text_allocates_nothing() {
    let _one = serial();
    let f = builtin_face(false, false).expect("built-in face");
    let mut buf = vec![0xffff_ffffu32; 800 * 60];
    let mut draw =
        || draw_text_tracked(f, &mut buf, 800, 800, 60, 4, 6, TEXT, 0xff00_0000, 17.0, 0.0);
    draw();
    let (_, heap) = measure(&mut draw);
    assert_eq!(heap.allocs, 0, "a repaint of cached text allocated");
}

#[test]
fn a_page_face_is_parsed_once_not_per_call() {
    let _one = serial();
    let key = family_key("Proof Parsed Once");
    assert!(fonts::ingest_font(key, SANS.to_vec()));
    let mut px = vec![0xffff_ffffu32; 800 * 60];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: 800, width: 800, height: 60 };
    let mut repaint = || {
        let run = TextRun {
            key,
            mono: false,
            bold: false,
            italic: false,
            x: 4,
            top_y: 6,
            px: 17.0,
            spacing: 0.0,
            clip: NO_CLIP,
        };
        fonts::draw_text(&mut fb, run, TEXT, 0xff00_0000);
        fonts::measure_text(key, false, false, TEXT, 17.0, 0.0)
    };
    let width = repaint();
    let (same, heap) = measure(|| (0..50).all(|_| repaint() == width));
    fonts::clear();
    assert!(width > 0 && same, "the page face measured differently between calls");
    assert_eq!(heap.allocs, 0, "measuring and drawing with a page face allocated");
}
