// NONOS Operating System (AGPL-3.0-or-later)
//! The Home screen in shared controls, at 1x to 2x: every rail row, document
//! row and the "View all" button lights and presses only where its press
//! acts, and "View all" is unavailable while every document is on show.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::{Card, Controls, Rect, Step};

use super::doc_ref::{section_title, DocRef};
use super::metrics::nav_row;
use super::metrics_pane::{doc_row, list_card, view_all};
use super::pick::HomePick;
use super::screen::paint_screen;
use super::state::{HomeState, NAV_LABELS};
use crate::editor::square_tests::{centre, lock};
use crate::editor::strip_tests::{changed, W};

/// The window's 1x height; each scale grows it by the same factor.
const H: u32 = 720;
const DOCS: [(&str, &str); 3] =
    [("notes.txt", "/notes.txt"), ("a-much-longer-name.md", "/docs/a-much-longer-name.md"), ("main.rs", "/src/main.rs")];

fn paint(nav: usize, c: &mut Controls<HomePick>) -> Vec<u32> {
    let list: Vec<DocRef> = DOCS.iter().map(|&(n, p)| DocRef { name: n.into(), path: p.into() }).collect();
    let h = scale().px(H);
    let mut px = vec![0u32; (W * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: h };
    paint_screen(&mut fb, nav, &list, c);
    px
}

fn picks(nav: usize) -> Vec<(HomePick, Rect)> {
    let title = section_title(nav);
    let (card, shown) = list_card(W, scale().px(H), title, DOCS.len());
    assert_eq!(shown, DOCS.len(), "every document fits");
    let content = Card::new().title(title).content(card);
    let navs = (0..NAV_LABELS.len()).map(|i| (HomePick::Nav(i), nav_row(i)));
    let docs = (0..shown).map(|i| (HomePick::Doc(i), doc_row(content, i)));
    navs.chain([(HomePick::ViewAll, view_all(card, title).1)]).chain(docs).collect()
}

#[test]
fn every_control_lights_and_presses_only_where_its_press_acts() {
    let _g = lock();
    for (q, nav) in [4, 5, 6, 8].into_iter().flat_map(|q| [(q, 0), (q, 1)]) {
        (set_scale(q), HomeState::select(nav));
        assert_eq!(HomeState::load().nav, nav);
        for (pick, at) in picks(nav) {
            let mut c = Controls::acting_on_press();
            let unlit = paint(nav, &mut c);
            let (x, y) = centre(at);
            c.moved(x, y);
            let lit = paint(nav, &mut c);
            if pick == HomePick::ViewAll && nav == 0 {
                assert_eq!((c.pressed(x, y), paint(nav, &mut c)), (Step::Idle, unlit), "at {q}");
                continue;
            }
            assert_eq!(c.pressed(x, y), Step::Act(pick), "{pick:?} on {nav} at {q}");
            let pressed = paint(nav, &mut c);
            for diff in [changed(&unlit, &lit), changed(&unlit, &pressed)] {
                let here = diff.iter().all(|&(x, y)| c.at(x, y) == Some(pick));
                assert!(here && (pick == HomePick::Nav(nav) || !diff.is_empty()), "{pick:?} at {q}");
            }
            (c.released(), c.left());
            assert_eq!(paint(nav, &mut c), unlit, "a pointer that left, {pick:?} at {q}");
        }
    }
    (HomeState::select(0), set_scale(4));
}
