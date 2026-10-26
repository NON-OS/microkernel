// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Text is measured and drawn at its CSS em: a font-size is the em square,
//! not the ascent-to-descent height the rasterizer scales by, and page text
//! has no readability floor.

use crate::browser::fonts::measure_text;
use crate::render::{render, texts};

/// Page text with no font of its own is drawn in the system face, as the
/// Terminal is: Geist (toolkit font/ttf/face.rs). Its em is 1000 units and
/// its ascent to descent 1300, so at a 1000 px em the word is its own
/// advances, 3708 units less 68 of kerning; scaled by the ascent-to-descent
/// height instead, it would be 1000/1300 of that, 2800, and 47 px at 17.
#[test]
fn a_17px_word_measures_at_its_true_em() {
    let em = measure_text(0, false, false, "Nothing", 1000.0, 0.0);
    assert!((3638..=3642).contains(&em), "'Nothing' at a 1000px em is {em}px wide");
    let w = measure_text(0, false, false, "Nothing", 17.0, 0.0);
    assert!((61..=62).contains(&w), "'Nothing' at 17px is {w}px wide");
}

#[test]
fn small_text_has_no_floor() {
    let small = measure_text(0, false, false, "Sphinx", 12.0, 0.0);
    let big = measure_text(0, false, false, "Sphinx", 24.0, 0.0);
    assert!((big - 2 * small).abs() <= 2, "12px {small} vs 24px {big}");
}

#[test]
fn word_gaps_in_small_mono_text_are_uniform() {
    let html = "<p style=\"font-family:monospace;font-size:12px;margin:0\">ab cde f ghij kl</p>";
    let doc = render(html, 600);
    let t = texts(&doc);
    let gaps: Vec<i32> = t.windows(2).map(|w| w[1].0 - (w[0].0 + w[0].2)).collect();
    assert_eq!(gaps.len(), 4);
    assert!(gaps.iter().all(|g| (g - gaps[0]).abs() <= 1), "gaps {gaps:?}");
    assert!(gaps[0] > 0 && gaps[0] < 12, "one space, not a hole: {gaps:?}");
}
