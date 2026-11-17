// NONOS Operating System (AGPL-3.0-or-later)
//! The header and the hint bar: the header keeps the end of a long working
//! directory, and each band paints only inside itself, the hint bar's keys
//! and labels each only in the box laid out for it, at 1x and 2x.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::Rect;

use crate::footer::{draw_footer, footer_h, hint_boxes, HINTS};
use crate::header::{cwd_shown, draw_header, header_h};
use crate::strip_boxes::serial;

const H: u32 = 200;
const LONG: &str = "/usr/share/doc/nonos/userland/capsule_terminal/src/paint/vt/body";

fn frame(w: u32, f: impl Fn(&mut PaintBuffer)) -> Vec<(i32, i32)> {
    let mut px = vec![0u32; (w * H) as usize];
    f(&mut PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: H });
    let at = |n: usize| ((n as u32 % w) as i32, (n as u32 / w) as i32);
    px.iter().enumerate().filter(|(_, &p)| p != 0).map(|(n, _)| at(n)).collect()
}

#[test]
fn the_header_keeps_the_end_of_the_directory_and_stays_in_its_band() {
    let _g = serial();
    for (q, w) in [(4, 600), (8, 900), (4, 1400)] {
        set_scale(q);
        let shown = cwd_shown(LONG, w);
        assert!(shown.ends_with("vt/body"), "{shown}");
        assert_eq!(shown.starts_with('\u{2026}'), shown != LONG, "{shown}");
        let lit = frame(w, |fb| draw_header(LONG, fb));
        let band = Rect { x: 0, y: 0, w, h: header_h() + scale().hair() };
        assert!(lit.iter().all(|&(x, y)| contains(band, x, y)));
    }
}

#[test]
fn each_hint_paints_only_in_its_boxes_and_none_is_cut() {
    let _g = serial();
    for (q, w) in [(4, 900), (4, 300), (8, 900), (8, 2000)] {
        set_scale(q);
        let at = Rect { x: 0, y: H - footer_h(), w, h: footer_h() };
        let boxes = hint_boxes(at);
        assert!(boxes.len() <= HINTS.len() && (w < 400) == (boxes.len() < HINTS.len()));
        for pair in boxes.windows(2) {
            assert!(pair[0].1.x + pair[0].1.w < pair[1].0.x);
        }
        assert!(boxes.iter().all(|(k, l)| k.x + k.w < l.x && l.x + l.w <= w));
        let full = frame(w, |fb| draw_footer(fb, at, true));
        assert!(full.iter().all(|&(x, y)| contains(at, x, y)));
        let lit = changed(&render(w, at, false), &render(w, at, true), w);
        let ink = |r: Rect| Rect { w: r.w + scale().hair(), ..r };
        let inside =
            |x, y| boxes.iter().any(|&(k, l)| contains(ink(k), x, y) || contains(ink(l), x, y));
        assert!(!lit.is_empty() && lit.iter().all(|&(x, y)| inside(x, y)), "{q} {w}");
    }
}

fn render(w: u32, at: Rect, hints: bool) -> Vec<u32> {
    let mut px = vec![0u32; (w * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: H };
    draw_footer(&mut fb, at, hints);
    px
}

fn changed(a: &[u32], b: &[u32], w: u32) -> Vec<(i32, i32)> {
    let at = |n: usize| ((n as u32 % w) as i32, (n as u32 / w) as i32);
    a.iter().zip(b).enumerate().filter(|(_, (p, q))| p != q).map(|(n, _)| at(n)).collect()
}
