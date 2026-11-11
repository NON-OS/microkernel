// NONOS Operating System (AGPL-3.0-or-later)
//! The Special Character panel as the shared card of rows, at 1x to 2x: a
//! row lights and presses only where its press acts and picks the character
//! its label shows, and a pointer that left lights nothing.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::{Controls, Step};

use super::square_tests::{centre, lock};
use super::strip_tests::{changed, W};
use crate::editor::panel::{panel_note, panel_title, special_labels, Panel};
use crate::editor::panel_view::PanelView;
use crate::editor::specials::SPECIALS;

pub(super) const H: u32 = 1600;

pub(super) fn view<'a>(panel: Panel, rows: &'a [(&'a str, &'a str)]) -> PanelView<'a> {
    let (title, note) = (panel_title(panel), panel_note(panel));
    PanelView { title, note, rows, picks: panel == Panel::Special }
}

pub(super) fn paint(v: &PanelView, c: &mut Controls<usize>) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    v.paint(&mut fb, v.rect(W, H, 26), c);
    px
}

#[test]
fn a_pick_lights_presses_and_inserts_only_where_its_press_acts() {
    let _g = lock();
    let rows: Vec<(&str, &str)> = special_labels().into_iter().map(|l| (l, "")).collect();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let v = view(Panel::Special, &rows);
        assert_eq!(v.rows_at(v.rect(W, H, 26)).count(), SPECIALS.len(), "all shown at {q}");
        for (i, at) in v.rows_at(v.rect(W, H, 26)) {
            let (c, (x, y)) = (&mut Controls::acting_on_press(), centre(at));
            let unlit = paint(&v, c);
            assert_eq!(c.pressed(x, y), Step::Act(i), "row {i} at {q}");
            assert!(rows[i].0.starts_with(SPECIALS[i].1), "row {i} inserts what it shows");
            (c.released(), c.moved(x, y));
            let lit = paint(&v, c);
            c.pressed(x, y);
            for diff in [changed(&unlit, &lit), changed(&unlit, &paint(&v, c))] {
                assert!(!diff.is_empty() && diff.iter().all(|&(x, y)| c.at(x, y) == Some(i)));
            }
            (c.released(), c.left());
            assert_eq!(paint(&v, c), unlit, "a pointer that left, row {i} at {q}");
        }
    }
    set_scale(4);
}
