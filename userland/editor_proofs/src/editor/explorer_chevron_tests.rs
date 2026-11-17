// NONOS Operating System (AGPL-3.0-or-later)
//! The explorer's chevrons: each folder row draws a ">" while closed and a
//! "v" while open in its indent, at 1x to 2x. A file row draws none, and
//! opening folders changes no pixel outside their indents.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{Controls, Rect};

use super::square_tests::lock;
use super::strip_tests::{changed, W};
use crate::editor::explorer::{Entry, Explorer};

const H: u32 = 240;
const TREE: [(&str, u16, bool); 3] = [("src", 0, true), ("editor", 1, true), ("main.rs", 2, false)];

fn paint(q: u32, open: bool) -> (Vec<u32>, Vec<Rect>) {
    let tree = TREE.map(|(name, depth, dir)| Entry { name, depth, dir, open: dir && open });
    let band = Rect { x: 46 * q / 4, y: 2 * q, w: 220 * q / 4, h: 24 * q };
    let ex = Explorer { entries: &tree, first: 0, selected: 9, band, row_h: 24 * q / 4 };
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    ex.paint(&mut fb, &mut Controls::acting_on_press());
    let step = scale().px(u32::from(SPACE.md));
    let indent = |(i, at): (usize, Rect)| Rect { w: step * (u32::from(TREE[i].1) + 1), ..at };
    (px, ex.rows().map(indent).collect())
}

fn ink(px: &[u32], r: Rect) -> Vec<(u32, u32)> {
    let dots = (r.y..r.y + r.h).flat_map(|y| (r.x..r.x + r.w).map(move |x| (x, y)));
    dots.filter(|&(x, y)| px[(y * W + x) as usize] != 0).collect()
}

fn span(dots: &[(u32, u32)]) -> (u32, u32) {
    let (xs, ys) = (dots.iter().map(|d| d.0), dots.iter().map(|d| d.1));
    (xs.clone().max().unwrap() - xs.min().unwrap(), ys.clone().max().unwrap() - ys.min().unwrap())
}

#[test]
fn a_folder_draws_the_toolkit_chevron_in_its_indent() {
    let _g = lock();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let ((shut, rows), (open, _)) = (paint(q, false), paint(q, true));
        for (i, r) in rows.iter().enumerate() {
            let (s, o) = (ink(&shut, *r), ink(&open, *r));
            if !TREE[i].2 {
                assert!(s.is_empty() && o.is_empty(), "file row {i} at {q}");
                continue;
            }
            let ((sw, sh), (ow, oh)) = (span(&s), span(&o));
            assert!(sh > sw && ow > oh, "row {i} at {q}: > {sw}x{sh}, v {ow}x{oh}");
        }
        let moved = changed(&shut, &open);
        let inside = |&(x, y): &(i32, i32)| rows[..2].iter().any(|&r| contains(r, x, y));
        assert!(!moved.is_empty() && moved.iter().all(inside), "opening moved pixels at {q}");
    }
    set_scale(4);
}
