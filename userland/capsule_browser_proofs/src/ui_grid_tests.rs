// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! The Tabs page grid: cards never overlap and keep their close button and
//! group chip inside them (where each hits is proven in
//! `tests/page_controls`); and the pill row,
//! which keeps its end pills and fits what it can between them.

use crate::browser::ui::grid_geom::{chip_rect, close_rect};
use crate::browser::ui::list_geom::{middle_pills, ListGeom};
use crate::browser::ui::rect::Rect;

fn overlap(a: Rect, b: Rect) -> bool {
    a.x < b.right() && b.x < a.right() && a.y < b.bottom() && b.y < a.bottom()
}

#[test]
fn grid_cards_never_overlap_and_hold_their_parts() {
    for w in [300, 600, 900, 1300] {
        for s in [0, 120] {
            let g = ListGeom::new(w, 800, s);
            let cols = g.cols();
            assert!((1..=4).contains(&cols));
            for i in 0..9 {
                let r = g.grid_rect(i, cols);
                assert!(r.x >= g.frame.x && r.right() <= g.frame.right());
                for j in i + 1..9 {
                    assert!(!overlap(r, g.grid_rect(j, cols)));
                }
                assert_eq!(r.y, g.band - s + (i as i32 / cols) * (r.h + 20));
                let c = close_rect(r);
                assert!(c.x >= r.x && c.right() <= r.right() && c.y >= r.y);
                let chip = chip_rect(r, 1000, 30);
                assert!(chip.x >= r.x && chip.right() <= r.right() && chip.bottom() <= r.bottom());
            }
        }
    }
    assert!(ListGeom::new(2000, 800, 0).cols() >= ListGeom::new(500, 800, 0).cols());
}

#[test]
fn the_end_pills_stay_and_the_middle_ones_fill_what_is_left() {
    let w = [40, 100, 100, 100, 120];
    let all = w.iter().sum::<i32>() + 8 * (w.len() as i32 - 1);
    assert_eq!(middle_pills(&w, all), 3);
    assert_eq!(middle_pills(&w, all - 1), 2);
    assert_eq!(middle_pills(&w, 40 + 8 + 120 + 108), 1);
    assert_eq!(middle_pills(&w, 0), 0);
    assert_eq!(middle_pills(&[40, 120], 1000), 0);
    assert_eq!(middle_pills(&[40], 1000), 0);
    let g = ListGeom::new(900, 600, 0);
    let kept = [40, 100, 120];
    assert!(g.pill(&kept, 2).right() <= g.frame.x + all - 2 * 108);
}
