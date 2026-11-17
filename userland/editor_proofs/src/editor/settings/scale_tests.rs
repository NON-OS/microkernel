// NONOS Operating System (AGPL-3.0-or-later)
//! The Settings screen keeps logical metrics and draws them at the display
//! scale: at 1x to 2x every length is its 1x length through `scale().px`, the
//! title through `scale().font`, and a window grown by the same scale holds
//! the rail's rows under the title and every section's card.

use nonos_toolkit::design::scale::{scale, set_scale};

use super::card::card_rect;
use super::geom::{
    head_top, lh, nav_row, pane_pad, pane_x, rail_pad, rail_w, rail_x, title_px, title_top,
    NAV_LABELS,
};
use super::sect::section;
use crate::editor::manifest::{HEIGHT, WIDTH};
use crate::editor::square_tests::lock;

/// Every length, then the title size, the Settings screen lays out with.
fn sizes() -> (Vec<u32>, Vec<f32>) {
    let lengths = vec![rail_x(), rail_w(), rail_pad(), pane_pad(), title_top(), head_top()];
    (lengths, vec![title_px()])
}

#[test]
fn every_settings_size_grows_with_the_scale_and_the_window_still_fits() {
    let _g = lock();
    set_scale(4);
    let (lengths, fonts) = sizes();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let s = scale();
        let px = lengths.iter().map(|&n| s.px(n)).collect();
        let want = (px, fonts.iter().map(|&f| s.font(f)).collect());
        assert_eq!(sizes(), want, "q={q}: a size is not its 1x size at the scale");
        let (w, h) = (s.px(WIDTH), s.px(HEIGHT));
        for i in 0..NAV_LABELS.len() {
            let row = nav_row(i);
            assert!(row.y >= title_top() + lh(title_px()), "q={q}: row {i} sits on the title");
            assert!(row.x >= rail_x() && row.x + row.w <= pane_x(), "q={q}: row {i}");
        }
        for nav in 0..NAV_LABELS.len() {
            let sec = section(nav).unwrap();
            let at = card_rect(w, sec.head, sec.rows.len());
            assert_eq!(at.x, pane_x() + pane_pad(), "q={q}: section {nav}");
            assert!(at.x + at.w <= w && at.y + at.h <= h, "q={q}: section {nav} leaves the window");
        }
    }
    set_scale(4);
}
