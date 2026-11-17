// NONOS Operating System (AGPL-3.0-or-later)
//! The Settings screen in shared controls, at 1x to 2x: every rail row and
//! setting row lights and presses only where its press acts.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::{Card, Controls, Rect, Step};

use super::card::{card_rect, row_rect};
use super::geom::{nav_row, NAV_LABELS};
use super::paint::paint_settings;
use super::pick::Pick;
use super::sect::section;
use super::state::select_nav;
use crate::editor::square_tests::{centre, lock};
use crate::editor::strip_tests::{changed, W};

pub(super) const H: u32 = 480;

pub(super) fn paint(c: &mut Controls<Pick>) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    paint_settings(&mut fb, c);
    px
}

pub(super) fn row_at(nav: usize, i: usize) -> Rect {
    let sec = section(nav).unwrap();
    let at = card_rect(W, sec.head, sec.rows.len());
    row_rect(Card::new().title(sec.head).content(at), i)
}

fn picks(nav: usize) -> impl Iterator<Item = (Pick, Rect)> {
    let rows = section(nav).map_or(0, |s| s.rows.len());
    let navs = (0..NAV_LABELS.len()).map(|i| (Pick::Nav(i), nav_row(i)));
    navs.chain((0..rows).map(move |i| (Pick::Row(i), row_at(nav, i))))
}

#[test]
fn every_control_lights_and_presses_only_where_its_press_acts() {
    let _g = lock();
    for (q, nav) in [4, 5, 6, 8].into_iter().flat_map(|q| [(q, 0), (q, 1)]) {
        (set_scale(q), select_nav(nav));
        for (pick, at) in picks(nav) {
            let mut c = Controls::acting_on_press();
            let unlit = paint(&mut c);
            let (x, y) = centre(at);
            c.moved(x, y);
            let lit = paint(&mut c);
            assert_eq!(c.pressed(x, y), Step::Act(pick), "{pick:?} at {q}");
            let pressed = paint(&mut c);
            for diff in [changed(&unlit, &lit), changed(&unlit, &pressed)] {
                let here = diff.iter().all(|&(x, y)| c.at(x, y) == Some(pick));
                assert!(here && (pick == Pick::Nav(nav) || !diff.is_empty()), "{pick:?} at {q}");
            }
            (c.released(), c.left());
            assert_eq!(paint(&mut c), unlit, "a pointer that left, {pick:?} at {q}");
        }
    }
    (select_nav(0), set_scale(4));
}

pub(super) fn crop(px: &[u32], r: Rect) -> Vec<u32> {
    let rows = r.y..r.y + r.h;
    rows.flat_map(|y| px[(y * W + r.x) as usize..(y * W + r.x + r.w) as usize].to_vec()).collect()
}
