// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! The reader settings card's geometry: every control sits inside the
//! card and apart from the others, and a card too narrow is not drawn.
//! Where each control lights and acts is proven in `tests/reader_controls`.

use crate::browser::ui::reader_geom::{panel, ReaderGeom, MARGIN, MIN_W, PANEL_W};
use crate::browser::ui::rect::Rect;

fn controls(g: &ReaderGeom) -> Vec<Rect> {
    let mut v = vec![g.close, g.minus, g.plus, g.time];
    v.extend(g.swatches);
    v.extend(g.widths);
    v
}

fn inside(r: Rect, o: Rect) -> bool {
    r.x >= o.x && r.y >= o.y && r.right() <= o.right() && r.bottom() <= o.bottom()
}

#[test]
fn controls_sit_in_the_panel_and_do_not_overlap() {
    for w in (MIN_W + 2 * MARGIN..1600).step_by(17) {
        let g = panel(w);
        assert!(g.drawn(), "{w}");
        assert!(g.panel.right() <= w - MARGIN && g.panel.x >= 0 && g.panel.w <= PANEL_W);
        let c = controls(&g);
        for (i, a) in c.iter().enumerate() {
            assert!(inside(*a, g.panel), "{w}: {a:?}");
            for b in c.iter().skip(i + 1).chain([&g.dots]) {
                let apart =
                    a.right() <= b.x || b.right() <= a.x || a.bottom() <= b.y || b.bottom() <= a.y;
                assert!(apart, "{w}: {a:?} {b:?}");
            }
        }
    }
}

#[test]
fn a_card_too_narrow_is_not_drawn() {
    for w in [0, -10, 100, MIN_W + 2 * MARGIN - 1] {
        assert!(!panel(w).drawn(), "{w}");
    }
}
