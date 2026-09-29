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

//! Faces that agree on glyph ids, sizes and metrics never share a cached
//! raster: a regular and a bold cut, and a page face loaded after another
//! was freed.

use capsule_browser_proofs::browser::fonts::{self, family_key, TextRun};
use nonos_toolkit::font::em::em_scale;
use nonos_toolkit::font::ttf::{builtin_face, clear_glyph_cache, draw_text_tracked, FontRef};
use nonos_toolkit::paint::PaintBuffer;

use super::serial::serial;

const SANS: &[u8] = include_bytes!("../../../toolkit/assets/fonts/NotoSans-Regular.ttf");
const BOLD: &[u8] = include_bytes!("../../../toolkit/assets/fonts/NotoSans-Bold.ttf");
const TEXT: &str = "Hamburgefonstiv 0123 AVfi";

/* Page text is drawn at its em, so the reference is too. */
fn draw(f: &FontRef) -> Vec<u32> {
    let mut buf = vec![0xffff_ffffu32; 420 * 40];
    draw_text_tracked(f, &mut buf, 420, 420, 40, 3, 5, TEXT, 0xff00_0000, em_scale(f, 21.0), 0.0);
    buf
}

#[test]
fn bold_never_draws_with_regular_glyphs() {
    let _one = serial();
    let (regular, bold) = (builtin_face(false, false).unwrap(), builtin_face(false, true).unwrap());
    clear_glyph_cache();
    let bold_alone = draw(bold);
    clear_glyph_cache();
    let regular_alone = draw(regular);
    assert_ne!(bold_alone, regular_alone);
    assert_eq!(draw(bold), bold_alone, "bold drew with the regular cut's glyphs");
}

/* Both cuts padded to one length, so a face installed after the other was
freed can sit at its address with its length. */
fn page_draw(face: &[u8], len: usize) -> Vec<u32> {
    let key = family_key("Proof Reuse");
    let mut bytes = face.to_vec();
    bytes.resize(len, 0);
    assert!(fonts::ingest_font(key, bytes));
    let mut px = vec![0xffff_ffffu32; 420 * 40];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: 420, width: 420, height: 40 };
    let run = TextRun {
        key,
        mono: false,
        bold: false,
        italic: false,
        x: 3,
        top_y: 5,
        px: 21.0,
        spacing: 0.0,
    };
    fonts::draw_text(&mut fb, run, TEXT, 0xff00_0000);
    px
}

#[test]
fn a_page_face_loaded_where_a_freed_one_was_draws_its_own_glyphs() {
    let _one = serial();
    let len = SANS.len().max(BOLD.len());
    let regular = page_draw(SANS, len);
    fonts::clear();
    let bold = page_draw(BOLD, len);
    fonts::clear();
    assert_ne!(regular, bold);
    assert_eq!(bold, draw(builtin_face(false, true).unwrap()));
}
