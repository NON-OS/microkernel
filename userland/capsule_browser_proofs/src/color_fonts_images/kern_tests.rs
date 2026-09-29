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

/* GPOS pair kerning: a face whose kerning lives only in GPOS (Geist) draws
 * a kerned pair tighter than its two glyphs side by side, and a monospace
 * face with no kerning measures as the sum of its glyphs. */
use super::woff2_tests::font;
use crate::browser::fonts::{family_key, ingest_font, measure_text};

fn width(key: u32, s: &str) -> i32 {
    measure_text(key, false, false, s, 200.0, 0.0)
}

#[test]
fn a_gpos_kerned_pair_is_tighter_than_its_glyphs() {
    let key = family_key("kern test geist");
    assert!(ingest_font(key, font("GeistVariable.ttf")));
    for pair in ["To", "Ty", "AV", "Yo"] {
        let (a, b) = pair.split_at(1);
        let (whole, parts) = (width(key, pair), width(key, a) + width(key, b));
        assert!(whole < parts - 2, "{pair}: {whole} against {parts}");
    }
}

#[test]
fn a_face_without_kerning_measures_as_its_glyphs() {
    let key = family_key("kern test mono");
    assert!(ingest_font(key, font("JetBrainsMono-Regular.ttf")));
    let (whole, parts) = (width(key, "To"), width(key, "T") + width(key, "o"));
    assert!((whole - parts).abs() <= 1, "{whole} against {parts}");
}
