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

/* A site that splits a family into unicode-range subsets must get the Latin
 * one, and Latin text in a page face with no Latin glyphs must still draw. */

use capsule_browser_proofs::browser::fonts::{self, collect_font_faces, family_key, TextRun};
use nonos_toolkit::font::fallback::has_glyph;
use nonos_toolkit::font::ttf::FontRef;
use nonos_toolkit::paint::PaintBuffer;

const HEBREW: &[u8] = include_bytes!("../../toolkit/assets/fonts/NotoSansHebrew-Subset.ttf");

fn face(url: &str, range: &str) -> String {
    format!("@font-face{{font-family:Fira;src:url({url}) format('woff');unicode-range:{range}}}")
}

#[test]
fn the_latin_subset_wins_in_either_order() {
    let viet = face("viet.woff", "U+0102-0103, U+0110-0111, U+1EA0-1EF9");
    let latin = face("latin.woff", "U+0000-00FF, U+0131, U+2000-206F");
    for css in [viet.clone() + &latin, latin + &viet] {
        assert_eq!(collect_font_faces(&css), vec![(family_key("Fira"), "latin.woff".into())]);
    }
}

#[test]
fn a_wildcard_range_and_no_range_both_cover_latin() {
    let wild = face("w.woff", "U+00??");
    assert_eq!(collect_font_faces(&wild)[0].1, "w.woff");
    let whole = "@font-face{font-family:Fira;src:url(a.woff)}";
    assert_eq!(collect_font_faces(whole)[0].1, "a.woff");
}

#[test]
fn latin_text_in_a_face_without_latin_still_draws() {
    let hebrew = FontRef::try_from_slice(HEBREW).unwrap();
    assert!(!has_glyph(&hebrew, 'A'), "the stand-in subset must lack Latin");
    let key = family_key("Subset Only");
    assert!(fonts::ingest_font(key, HEBREW.to_vec()));
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
    fonts::draw_text(&mut fb, run, "Hamburgefonstiv", 0xff00_0000);
    let ink = px.iter().filter(|&&p| p != 0xffff_ffff).count();
    assert!(ink > 500, "Latin text drew {ink} pixels");
}
