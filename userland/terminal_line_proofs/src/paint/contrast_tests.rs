// NONOS Operating System (AGPL-3.0-or-later)
//! WCAG AA on every theme: each colour text is drawn in, the dim hints and
//! times included, reaches 4.5:1 against the background and every lift of it
//! the bars and blocks are drawn on; the running mark, a shape and not text,
//! reaches 3:1. Dim text was 2.4:1 on abyss and 2.7:1 on dark.

use super::shade::elevate;
use crate::theme::profiles::{by_index, COUNT};

/// The lifts text sits on: blocks (5, 13), the header and the hint bar (10),
/// the input and find bars (12), and the side rail's rows (14).
const LIFTS: [u8; 6] = [0, 5, 10, 12, 13, 14];

fn luminance(c: u32) -> f64 {
    let ch = |s: u32| {
        let v = ((c >> s) & 0xFF) as f64 / 255.0;
        if v <= 0.03928 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * ch(16) + 0.7152 * ch(8) + 0.0722 * ch(0)
}

fn ratio(a: u32, b: u32) -> f64 {
    let (x, y) = (luminance(a), luminance(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

fn worst(colour: u32, bg: u32) -> f64 {
    LIFTS.iter().map(|&l| ratio(colour, elevate(bg, l))).fold(f64::MAX, f64::min)
}

#[test]
fn every_text_colour_reaches_4_5_to_1_on_every_theme() {
    for i in 0..COUNT {
        let t = by_index(i);
        let text = [("fg", t.fg), ("accent", t.accent), ("path", t.path), ("dim", t.dim)];
        let marks = [("ok", t.ok), ("err", t.err)];
        for (name, c) in text.into_iter().chain(marks) {
            let r = worst(c, t.bg);
            assert!(r >= 4.5, "theme {i}: {name} {c:#010x} is {r:.2}:1");
        }
    }
}

#[test]
fn the_running_mark_reaches_3_to_1() {
    for i in 0..COUNT {
        let t = by_index(i);
        let r = worst(t.run, t.bg);
        assert!(r >= 3.0, "theme {i}: run {:#010x} is {r:.2}:1", t.run);
    }
}
