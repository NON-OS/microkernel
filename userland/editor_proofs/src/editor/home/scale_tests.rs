// NONOS Operating System (AGPL-3.0-or-later)
//! The Home screen keeps logical metrics and draws them at the display scale:
//! at 1x to 2x every length is its 1x length through `scale().px`, every text
//! size through `scale().font`, and a window grown by the same scale holds the
//! rail's rows, the pane head and the document card.

use nonos_toolkit::design::scale::{scale, set_scale};

use super::metrics::{
    body, brand_side, brand_y, head, nav_row, pane_pad, pane_x, rail_pad, rail_w, rail_x, subhead,
};
use super::metrics_pane::{cols_y, list_card, pane_content, strap_y};
use super::state::NAV_LABELS;
use crate::editor::manifest::{HEIGHT, WIDTH};
use crate::editor::square_tests::lock;

/// Every length, then every text size, the Home screen lays out with.
fn sizes() -> (Vec<u32>, Vec<f32>) {
    (
        vec![rail_w(), rail_pad(), pane_pad(), brand_side(), brand_y()],
        vec![body(), subhead(), head()],
    )
}

#[test]
fn every_home_size_grows_with_the_scale_and_the_window_still_fits() {
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
        assert_eq!(pane_x(), rail_x() + rail_w(), "q={q}: the pane does not start at the rail");
        for i in 0..NAV_LABELS.len() {
            let row = nav_row(i);
            assert!(row.y >= brand_y() + brand_side(), "q={q}: row {i} sits on the brand");
            assert!(row.x >= rail_x() && row.x + row.w <= rail_x() + rail_w(), "q={q}: row {i}");
        }
        assert!(strap_y() < cols_y(), "q={q}: the strapline runs into the card");
        let (x, cw) = pane_content(w);
        assert_eq!(x + cw + pane_pad(), w, "q={q}: the pane content leaves the window");
        let (card, shown) = list_card(w, h, "Recent", 12);
        assert!(card.y + card.h <= h && shown > 0, "q={q}: the card leaves the window");
    }
    set_scale(4);
}
