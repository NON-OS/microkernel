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

//! A face whose glyphs span more pixels than MAX_GLYPH_AREA, or than the
//! surface, draws them as missing glyphs, without allocating for them.

use capsule_browser_proofs::browser::fonts::{self, family_key, TextRun};
use nonos_toolkit::font::ttf::{clear_glyph_cache, draw_text_tracked, FontRef};
use nonos_toolkit::paint::PaintBuffer;

use super::counting::measure;
use super::serial::serial;
use super::sfnt::square_face;

const HEAP: usize = 4 << 20;
const WHITE: u32 = 0xffff_ffff;

/* Draw `text` at 16 px into a white w x h surface with a face that is only
borrowed here, so the cache is emptied first: returns inked pixels and the
heap peak of the draw. */
fn draw(face: &[u8], (w, h): (u32, u32), text: &str) -> (usize, usize) {
    clear_glyph_cache();
    let f = FontRef::try_from_slice(face).expect("the square face parses");
    let mut buf = vec![WHITE; (w * h) as usize];
    let s = w as usize;
    let (_, heap) =
        measure(|| draw_text_tracked(&f, &mut buf, s, w, h, 0, 0, text, 0xff00_0000, 16.0, 0.0));
    (buf.iter().filter(|&&p| p != WHITE).count(), heap.peak)
}

/* Past the cap nothing draws and nothing is allocated for the glyph; a
glyph at the cap, or at the surface's own area, still draws. */
#[test]
fn the_cap_and_the_surface_bound_what_draws_and_allocates() {
    let _one = serial();
    let (big, small) = ((1400, 1200), (100, 100));
    let cases = [(1100, big, false), (3000, big, false), (1000, big, true), (1025, big, false)];
    for (n, surface, drawn) in cases.into_iter().chain([(100, small, true), (101, small, false)]) {
        let (inked, peak) = draw(&square_face(n), surface, if drawn { "H" } else { "Hamburg" });
        assert_eq!(inked > 0, drawn, "a {n}-pixel glyph on a {surface:?} surface");
        assert!(drawn || peak <= HEAP, "{n}-pixel glyphs peaked at {peak} heap bytes");
    }
}

#[test]
fn a_hostile_page_face_draws_nothing_within_a_small_heap() {
    let _one = serial();
    let key = family_key("Proof Hostile");
    assert!(fonts::ingest_font(key, square_face(3000)));
    let mut px = vec![WHITE; 1400 * 1200];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: 1400, width: 1400, height: 1200 };
    let run = TextRun { key, mono: false, bold: false, x: 0, top_y: 0, px: 16.0, spacing: 0.0 };
    let (_, heap) = measure(|| fonts::draw_text(&mut fb, run, "Hamburgefonstiv", 0xff00_0000));
    let peak = heap.peak;
    fonts::clear();
    assert!(peak <= HEAP, "the page face peaked at {peak} heap bytes");
    assert!(px.iter().all(|&p| p == WHITE), "the page face drew");
}
