// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! The list pages' geometry: rows stack under the header band, scroll moves
//! them together, and pills sit apart. Where rows, pills and their buttons
//! hit is proven in `tests/page_controls`.

use crate::browser::ui::list_geom::{max_scroll, ListGeom};
use crate::browser::ui::rect::Rect;

fn overlap(a: Rect, b: Rect) -> bool {
    a.x < b.right() && b.x < a.right() && a.y < b.bottom() && b.y < a.bottom()
}

#[test]
fn rows_stack_in_the_frame_from_under_the_band() {
    for w in [400, 800, 1600] {
        let g = ListGeom::new(w, 900, 0);
        assert!(g.row_rect(0).y >= g.band, "{w}");
        for i in 0..10 {
            let (a, b) = (g.row_rect(i), g.row_rect(i + 1));
            assert_eq!(a.bottom(), b.y, "{w} row {i}");
            assert!(a.x == g.frame.x && a.right() == g.frame.right(), "{w} row {i}");
        }
    }
}

#[test]
fn scroll_shifts_rows_together() {
    let base = ListGeom::new(1000, 600, 0);
    for s in [0, 30, 56, 200, 1000] {
        let g = ListGeom::new(1000, 600, s);
        for i in 0..30 {
            assert_eq!(g.row_rect(i).y, base.row_rect(i).y - s);
        }
    }
    assert_eq!(ListGeom::new(1000, 600, -40), base);
}

#[test]
fn pills_sit_apart_left_to_right() {
    let g = ListGeom::new(1000, 600, 300);
    let widths = [60, 90, 75];
    for i in 0..2 {
        let (a, b) = (g.pill(&widths, i), g.pill(&widths, i + 1));
        assert!(!overlap(a, b) && a.x < b.x && a.y == g.pills_y, "{i}");
    }
    assert_eq!(g.pill(&widths, 0).x, g.frame.x);
}

#[test]
fn scroll_is_bounded() {
    let g = ListGeom::new(1000, 600, 0);
    assert_eq!(max_scroll(500, 600), 0);
    assert_eq!(max_scroll(g.list_h(20), 600), g.list_h(20) - 600);
}
