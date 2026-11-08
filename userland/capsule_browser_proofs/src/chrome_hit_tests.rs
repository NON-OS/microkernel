// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! The toolbar and start page are hit-tested at the width they are painted.
//! A fixed 1360 once put the menu at x >= 1316 and the badges 90 px off at 1202.
//! The toolbar's are the boxes its controls were laid out with.

use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::Controls;

use crate::browser::omnibox::geometry::pill_rect;
use crate::browser::shell::route::Route;
use crate::browser::toolbar::buttons;
use crate::browser::toolbar::look::Shows;
use crate::browser::ui::limits::TOOLBAR_H;
use crate::browser::ui::rect::Rect;
use crate::browser::ui::start_geom::{fav_at, fav_rect, pill_text, scrolled};
use crate::browser::ui::toolbar_geom::{self, Part, ToolbarGeom};

/* Every toolbar control live, laid out as a paint of a `w` wide toolbar
 * lays it out. */
pub(crate) fn placed(w: i32) -> (ToolbarGeom, Controls<Part>) {
    let g = toolbar_geom::buttons(Rect::new(0, 0, w, TOOLBAR_H));
    let s = Shows {
        back: true,
        fwd: true,
        loading: false,
        page: true,
        marked: false,
        ready: true,
        reading: false,
        route: Route::Direct,
    };
    let (mut px, mut c) = (vec![0u32; (w * TOOLBAR_H) as usize], Controls::acting_on_press());
    let (pw, ph) = (w as u32, TOOLBAR_H as u32);
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: pw, width: pw, height: ph };
    buttons::paint(&mut fb, &g, &s, &mut c);
    (g, c)
}

#[test]
fn toolbar_buttons_follow_the_width() {
    let y = TOOLBAR_H / 2;
    for w in [1180, 1338, 1918] {
        let bar = Rect::new(0, 0, w, TOOLBAR_H);
        let (_, c) = placed(w);
        assert_eq!(c.at(w - 20, y), Some(Part::Sidebar), "sidebar at {w}");
        assert_eq!(c.at(w - 70, y), Some(Part::Tabs), "tabs at {w}");
        assert_eq!(c.at(w - 120, y), Some(Part::Star), "star at {w}");
        assert_eq!(c.at(w - 165, y), Some(Part::Reader), "reader at {w}");
        assert_eq!(c.at(w - 250, y), Some(Part::Route), "badge at the pill end at {w}");
        assert_eq!(c.at(w - 340, y), Some(Part::Pill), "pill left of the badge at {w}");
        assert_eq!(c.at(w + 5, y), None, "past the edge at {w}");
        assert_eq!(c.at(20, y), Some(Part::Back));
        assert_eq!(c.at(70, y), Some(Part::Fwd));
        assert_eq!(c.at(110, y), Some(Part::Reload));
        assert_eq!(c.at(160, y), Some(Part::Home));
        let p = pill_rect(bar);
        assert_eq!(p.x + p.w, w as u32 - 198);
    }
    let (g, c) = placed(1918);
    assert_eq!(c.at(1320, y), Some(Part::Pill), "not the star");
    assert_eq!(c.at(20, g.back.y - 1), None, "above the button");
    assert_eq!(c.at(20, g.back.bottom() - 1), Some(Part::Back), "its last row");
    assert_eq!(c.at(20, g.back.bottom()), None, "below the button");
}

#[test]
fn favourites_and_search_pill_follow_the_width() {
    for w in [480, 948, 1104, 1686] {
        let g = scrolled(w, 600, 8, 0);
        for i in 0..8 {
            let r = fav_rect(&g, i);
            assert_eq!(fav_at(&g, r.x + r.w / 2, r.y + r.h / 2), Some(i), "tile {i} at {w}");
            assert!(r.x >= 0 && r.right() <= w, "tile {i} inside {w}");
        }
        assert_eq!(fav_at(&g, fav_rect(&g, 0).x - 1, fav_rect(&g, 0).y + 10), None);
        let t = pill_text(&g).expect("pill text");
        let p = g.pill;
        assert!(t.x > p.x && t.right() < p.right() && t.y == p.y && t.h == p.h, "{w}");
    }
}
