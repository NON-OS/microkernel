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

//! The glyph cache's byte budget under real page text, and the page font
//! registry emptying it on navigation.

use capsule_browser_proofs::browser::fonts::{self, family_key, TextRun};
use nonos_toolkit::font::ttf::GLYPH_CACHE_BUDGET;
use nonos_toolkit::font::ttf::{builtin_face, draw_text_tracked, glyph_cache_bytes};
use nonos_toolkit::paint::PaintBuffer;

use super::serial::serial;

const SANS: &[u8] = include_bytes!("../../../toolkit/assets/fonts/NotoSans-Regular.ttf");

#[test]
fn page_text_stays_within_the_cache_budget() {
    let _one = serial();
    let f = builtin_face(false, false).expect("built-in face");
    let (w, h) = (1200u32, 900u32);
    let mut buf = vec![0xffff_ffffu32; (w * h) as usize];
    let text = "The quick brown fox jumps over the lazy dog 0123456789 AVfiW";
    let mut peak = 0;
    for step in 0..300i32 {
        let px = 9.0 + step as f32 * 0.37;
        let y = step * 7 % 800;
        draw_text_tracked(f, &mut buf, w as usize, w, h, 3, y, text, 0xff10_2030, px, 0.3);
        let bytes = glyph_cache_bytes();
        assert!(bytes <= GLYPH_CACHE_BUDGET, "{bytes} bytes after drawing at {px} px");
        peak = peak.max(bytes);
    }
    assert!(peak > GLYPH_CACHE_BUDGET / 2, "the run never filled the cache ({peak} bytes)");
}

#[test]
fn clearing_the_page_fonts_empties_the_glyph_cache() {
    let _one = serial();
    let key = family_key("Proof Sans");
    assert!(fonts::ingest_font(key, SANS.to_vec()));
    let (w, h) = (400u32, 60u32);
    let mut px = vec![0xffff_ffffu32; (w * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    let run = TextRun { key, mono: false, bold: false, x: 2, top_y: 4, px: 18.0, spacing: 0.0 };
    fonts::draw_text(&mut fb, run, "Hamburgefonstiv", 0xff00_0000);
    assert!(glyph_cache_bytes() > 0);
    fonts::clear();
    assert_eq!(glyph_cache_bytes(), 0);
}
