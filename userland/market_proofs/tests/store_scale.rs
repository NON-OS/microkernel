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

//! The marketplace window keeps logical metrics and draws them at the display
//! scale: at 1.25x, 1.5x and 2x every size is its 1x size through `scale().px`
//! or `scale().font`, and a window grown by the same scale holds the list and
//! the detail pane across, as many cards down, and each card's controls. Its
//! own binary, so setting the scale races no other test.

use nonos_market_proofs::store::ui::geometry::{action_rect, list_top, list_w, remove_rect};
use nonos_market_proofs::store::ui::metrics::{metrics, WIN_H, WIN_W};
use nonos_toolkit::design::scale::{scale, set_scale};

/// Every length, then every font size, the window lays out with.
#[rustfmt::skip]
fn sizes() -> (Vec<u32>, Vec<f32>) {
    let m = metrics();
    let lengths = [
        m.pad_x, m.pad_top, m.head_h, m.tab_h, m.tab_to_list, m.search_w, m.search_h, m.card_h,
        m.card_gap, m.card_pad, m.tile, m.tile_gap, m.scroll_gap, m.scroll_w, m.action_w,
        m.action_h, m.detail_w, m.detail_pad, m.gate_row_h, m.status_h, m.status_pad_x,
    ];
    (lengths.to_vec(), vec![m.title_px, m.name_px, m.body_px, m.small_px])
}

/// The cards a window `h` tall shows, as the frame counts them.
fn rows(h: u32) -> u32 {
    let m = metrics();
    let pane = h.saturating_sub(m.status_h + m.pad_x).saturating_sub(list_top());
    pane / (m.card_h + m.card_gap)
}

#[test]
fn every_size_grows_with_the_scale_and_the_window_still_fits() {
    set_scale(4);
    let ((lengths, fonts), rows_1x) = (sizes(), rows(WIN_H));
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (s, m) = (scale(), metrics());
        let px = lengths.iter().map(|&n| s.px(n)).collect();
        let want = (px, fonts.iter().map(|&f| s.font(f)).collect());
        assert_eq!(sizes(), want, "q={q}: a size is not its 1x size at the scale");
        let (w, lw) = (s.px(WIN_W), list_w(s.px(WIN_W)));
        assert_eq!(m.pad_x * 3 + lw + m.detail_w, w, "q={q}: list and pane fill the width");
        assert_eq!(rows(s.px(WIN_H)), rows_1x, "q={q}: as many cards show as at 1x");
        let text_x = m.pad_x + m.card_pad + m.tile + m.tile_gap;
        let (remove, action) = (remove_rect(m.pad_x, 0, lw), action_rect(m.pad_x, 0, lw));
        assert!(text_x < remove.0, "q={q}: Remove runs into the card's text");
        assert_eq!(action.0 + action.2 + m.card_pad, m.pad_x + lw, "q={q}");
        assert!(action.1 + action.3 <= m.card_h, "q={q}: the action leaves its card");
    }
    set_scale(4);
}
