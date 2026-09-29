// NONOS Operating System (AGPL-3.0-or-later)
//! Text is measured and drawn at its CSS em: a font-size is the em square,
//! not the ascent-to-descent height the rasterizer scales by, and page text
//! has no readability floor.

use crate::browser::fonts::measure_text;
use crate::render::{render, texts};

#[test]
fn a_17px_word_measures_at_its_true_em() {
    let w = measure_text(0, false, false, "Nothing", 17.0, 0.0);
    assert!((64..=66).contains(&w), "'Nothing' at 17px is {w}px wide");
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
