// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! About keeps logical metrics and draws them at the display scale: at 1x,
//! 1.25x, 1.5x and 2x every size is its 1x size through the scale, a card's
//! head is the shared card's own, a notch is three arrow steps, and a window
//! grown by the scale holds the frame and the screens that never scroll. Its
//! own binary, so setting the scale races no other test.

use apps_proofs::about::scroll_wheel::{wheel_step, wheel_to};
use apps_proofs::about::section::SECTIONS;
use apps_proofs::about::ui::chrome::pane_rect;
use apps_proofs::about::ui::metrics::{metrics, WIN_H, WIN_W};
use apps_proofs::about::ui::nav_geom::{rect, row_w, row_x};
use apps_proofs::about::ui::screens::{display, overview, system};
use apps_proofs::about::ui::{card, gauge, kv};
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::widgets::Card;

/// Every length, then every font size, About lays out with.
#[rustfmt::skip]
fn sizes() -> (Vec<u32>, Vec<f32>) {
    let m = metrics();
    let lengths = [
        m.sidebar_w, m.nav_top, m.nav_h, m.nav_gap, m.nav_pad_x, m.pane_pad_x, m.pane_pad_top,
        m.head_h, m.head_meta_gap, m.status_h, m.card_pad, m.card_gap, m.card_foot, m.kv_row_h,
        m.kv_label_w, m.kv_gap, m.chip_gap, m.scroll_step, m.hero_h, m.pair_h, m.pair_line,
        m.chain_h, m.line_step, kv::row_h(), gauge::height(),
    ];
    (lengths.to_vec(), vec![m.title_px, m.value_px, m.body_px, m.num_px])
}

#[test]
fn every_size_grows_with_the_scale_and_the_window_still_holds_it() {
    set_scale(4);
    let (lengths, fonts) = sizes();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (s, m) = (scale(), metrics());
        let px = lengths.iter().map(|&n| s.px(n)).collect();
        let want = (px, fonts.iter().map(|&f| s.font(f)).collect());
        assert_eq!(sizes(), want, "q={q}: a size is not its 1x size at the scale");
        assert_eq!(card::content_top(), Card::new().title("x").head_h(), "q={q}: head rescaled");
        assert_eq!(card::overhead(), card::content_top() + m.card_pad, "q={q}");
        let step = wheel_step();
        assert_eq!(step, 3 * m.scroll_step, "q={q}: a notch is not three arrow steps");
        assert_eq!((wheel_to(0, 9999, -1), wheel_to(2 * step, 9999, 1)), (step, step), "q={q}");
        assert_eq!((wheel_to(10, 9999, 1), wheel_to(9990, 9999, -1)), (0, 9999), "q={q}");
        let (w, h) = (s.px(WIN_W), s.px(WIN_H));
        let pane = pane_rect(w, h);
        assert!(pane.x >= m.sidebar_w && pane.x + pane.w + m.pane_pad_x <= w, "q={q}: pane x");
        assert!(pane.y >= m.pane_pad_top + m.head_h, "q={q}: the pane meets the page head");
        assert!(pane.y + pane.h + m.status_h <= h, "q={q}: the pane meets the status strip");
        let last = rect(SECTIONS.len() - 1);
        assert!(last.y + last.h <= pane.y + pane.h, "q={q}: the sidebar's last entry leaves");
        assert!(row_x() + row_w() <= m.sidebar_w, "q={q}: a nav entry leaves the sidebar");
        let fixed = [overview::content_h, system::content_h, display::content_h].map(|f| f(&pane));
        assert!(fixed.iter().all(|&c| c <= pane.h), "q={q}: a fixed screen scrolls: {fixed:?}");
    }
    set_scale(4);
}
