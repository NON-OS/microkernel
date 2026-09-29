// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Scripts the built-in face has no glyphs for (Arabic, Hebrew, Devanagari)
//! measure and draw in an embedded fallback face instead of .notdef boxes.

use nonos_app_skeleton::PaintBuffer;

use crate::browser::fonts::{draw_text, measure_text, TextRun, NO_CLIP};

/* A private-use code point: no face has a glyph, so it measures .notdef. */
const NOTDEF: &str = "\u{E000}";

fn ink(text: &str) -> Vec<u32> {
    let mut px = vec![0xff00_0000u32; 64 * 40];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: 64, width: 64, height: 40 };
    let run = TextRun {
        key: 0,
        mono: false,
        bold: false,
        italic: false,
        x: 4,
        top_y: 4,
        px: 24.0,
        spacing: 0.0,
        clip: NO_CLIP,
    };
    draw_text(&mut fb, run, text, 0xffff_ffff);
    px
}

#[test]
fn arabic_hebrew_and_devanagari_measure_in_a_fallback_face() {
    let notdef = measure_text(0, false, false, NOTDEF, 16.0, 0.0);
    for ch in ["\u{0628}", "\u{05D0}", "\u{0915}"] {
        let w = measure_text(0, false, false, ch, 16.0, 0.0);
        assert!(w > 0 && w != notdef, "{ch:?} is {w}px, .notdef is {notdef}px");
    }
}

#[test]
fn an_arabic_letter_draws_its_own_glyph_not_the_notdef_box() {
    let glyph = ink("\u{0628}");
    assert!(glyph.iter().any(|&p| p != 0xff00_0000), "something is drawn");
    assert_ne!(glyph, ink(NOTDEF), "and it is not the .notdef box");
}

#[test]
fn a_mixed_run_measures_as_the_sum_of_its_pieces() {
    let m = |s: &str| measure_text(0, false, false, s, 16.0, 0.0);
    let whole = m("ab\u{0628}\u{0629}cd");
    let parts = m("ab") + m("\u{0628}\u{0629}") + m("cd");
    assert!((whole - parts).abs() <= 2, "{whole} vs {parts}");
}
