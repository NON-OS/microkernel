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

//! WOFF2 web fonts (fixtures/fonts, OFL.txt there) unpack to faces that
//! measure and draw exactly as the TrueType files they were made from.

use nonos_app_skeleton::PaintBuffer;

use crate::browser::fonts::{draw_text, family_key, ingest_font, measure_text, TextRun, NO_CLIP};

/// A font file from fixtures/fonts.
pub(super) fn font(file: &str) -> Vec<u8> {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/fonts/");
    std::fs::read(format!("{dir}{file}")).expect("font fixture")
}

const TEXT: &str = "The quick brown fox jumps over the lazy dog, 0123456789 {}[]@&%? \
                    \u{c0}\u{c9}\u{ce}\u{d5}\u{dc} \u{e7}\u{f1}\u{df} \u{2014}\u{201c}\u{2026}";

fn ink(key: u32, mono: bool) -> Vec<u32> {
    let mut px = vec![0u32; 900 * 60];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: 900, width: 900, height: 60 };
    let (x, top_y, px_size, spacing) = (4, 8, 19.0, 0.0);
    let (bold, italic) = (false, false);
    let run = TextRun { key, mono, bold, italic, x, top_y, px: px_size, spacing, clip: NO_CLIP };
    draw_text(&mut fb, run, TEXT, 0xff20_3040);
    px
}

/// Install both files of a font under keys of their own and hold every
/// measure and every drawn pixel of the WOFF2 face to the TrueType one.
fn same_as_truetype(name: &str, stem: &str, mono: bool) {
    let (woff2, ttf) = (font(&format!("{stem}.woff2")), font(&format!("{stem}.ttf")));
    let (k2, kt) = (family_key(&format!("{name} woff2")), family_key(&format!("{name} ttf")));
    assert!(ingest_font(k2, woff2), "{name}.woff2 did not install");
    assert!(ingest_font(kt, ttf), "{name}.ttf did not install");
    for px in [11.0, 16.0, 37.5] {
        let w2 = measure_text(k2, mono, false, TEXT, px, 0.0);
        assert_eq!(w2, measure_text(kt, mono, false, TEXT, px, 0.0), "{name} at {px}px");
        assert!(w2 > 0);
    }
    for c in (0x20u8..0x7f).map(char::from) {
        let s = c.to_string();
        assert_eq!(
            measure_text(k2, mono, false, &s, 16.0, 0.0),
            measure_text(kt, mono, false, &s, 16.0, 0.0)
        );
    }
    let drawn = ink(k2, mono);
    assert!(drawn.iter().any(|&p| p != 0), "{name} drew nothing");
    assert!(drawn == ink(kt, mono), "{name} draws differently from its TrueType file");
}

#[test]
fn geist_woff2_matches_its_truetype_file() {
    same_as_truetype("geist proof", "GeistVariable", false);
}

#[test]
fn jetbrains_mono_woff2_matches_its_truetype_file() {
    same_as_truetype("jetbrains proof", "JetBrainsMono-Regular", true);
}
