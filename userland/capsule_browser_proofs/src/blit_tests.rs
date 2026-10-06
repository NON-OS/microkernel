// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! A scroll shifts the pixels still in view and paints only the rows that
//! came into view, and a partial band repaint grows over boxes it would cut.

use crate::browser::omnibox::parts::Shift;
use crate::browser::omnibox::{blit_plan, extend_rows, guarded, Blit};

#[test]
fn a_small_scroll_copies_rows_and_exposes_the_rest() {
    let plan = blit_plan(0, 60, 619);
    assert_eq!(plan, Blit::Rows { src_y: 60, dst_y: 0, rows: 559, expose_y: 559, expose_h: 60 });
    let up = blit_plan(60, 0, 619);
    assert_eq!(up, Blit::Rows { src_y: 0, dst_y: 60, rows: 559, expose_y: 0, expose_h: 60 });
    assert_eq!(blit_plan(5, 5, 619), Blit::Same);
    assert_eq!(blit_plan(0, 619, 619), Blit::Full);
    assert_eq!(blit_plan(900, 0, 619), Blit::Full);
}

#[test]
fn the_bottom_band_is_never_copied() {
    let down = guarded(blit_plan(0, 60, 619), 619, 24);
    assert_eq!(down, Some(Shift { src_y: 60, dst_y: 0, rows: 535, bands: [(0, 0), (535, 619)] }));
    let up = guarded(blit_plan(60, 0, 619), 619, 24);
    assert_eq!(up, Some(Shift { src_y: 0, dst_y: 60, rows: 535, bands: [(0, 60), (595, 619)] }));
    assert_eq!(guarded(blit_plan(0, 600, 619), 619, 24), None);
    assert_eq!(guarded(Blit::Full, 619, 24), None);
}

#[test]
fn a_band_grows_over_boxes_it_would_cut() {
    let spans = [(90, 130), (125, 160), (400, 420)];
    assert_eq!(extend_rows(&spans, 100, 120, 619), (90, 160));
    assert_eq!(extend_rows(&spans, 200, 300, 619), (200, 300));
    assert_eq!(extend_rows(&[(-50, 30)], 10, 20, 619), (0, 30));
    assert_eq!(extend_rows(&[(600, 700)], 610, 619, 619), (600, 619));
}
