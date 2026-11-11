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

//! Every size the process monitor lays out with is its 1x size at the display
//! scale, read once: at 1x, 1.25x, 1.5x and 2x each length is `scale().px` of
//! its 1x length and each type size `scale().font` of its 1x size, and the
//! shared table's rows and the findings' rows step by their own scaled
//! heights. Its own binary, so setting the scale races no other test.

use apps_proofs::metrics::{card_metrics, metrics, ROW_H};
use apps_proofs::pm::ui::screens::sec_geom::{self, ALERT_GAP, ALERT_H, BAND_GAP};
use apps_proofs::pm::ui::table_geom::{head_h, row_h, row_y};
use nonos_toolkit::design::scale::{scale, set_scale};

#[rustfmt::skip]
fn sizes() -> (Vec<u32>, Vec<f32>) {
    let (m, c) = (metrics(), card_metrics());
    let lengths = [
        m.sidebar_w, m.nav_top, m.nav_h, m.nav_gap, m.nav_pad_x, m.side_foot_h, m.side_foot_top,
        m.side_foot_line, m.pane_pad_x, m.pane_pad_top, m.head_h, m.status_h, m.status_gap,
        m.status_group_gap, m.search_w, m.search_min_w, m.search_h, m.search_meta_gap,
        m.inspector_w, m.inspector_pad, m.insp_field_h, m.insp_section_gap, m.insp_btn_h,
        m.insp_btn_bottom, m.chip_h, m.chip_pad_x, m.chip_gap, c.card_h, c.card_gap, c.card_pad,
        c.card_icon, c.card_label_gap, c.card_line_gap, c.panel_pad, c.panel_head_h, c.bar_h,
        c.bar_row_h, c.spark_h, c.risk_slot_w, c.risk_slot_h, c.risk_slot_gap,
    ];
    (lengths.to_vec(), vec![m.title_px, m.card_value_px, m.body_px, m.num_px])
}

/// Each size at 1x, then at each scale against it, and each row step at that
/// scale: one test, so no two set the scale at once.
#[test]
fn every_size_is_its_1x_size_and_the_rows_step_by_it_at_every_scale() {
    set_scale(4);
    let (lengths, fonts) = sizes();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let s = scale();
        let want = (lengths.iter().map(|&n| s.px(n)).collect(), fonts.iter().map(|&f| s.font(f)));
        assert_eq!(sizes(), (want.0, want.1.collect()), "q={q}: a size is not its 1x size");
        rows_step_by_their_own_scaled_heights(q);
    }
    set_scale(4);
}

fn rows_step_by_their_own_scaled_heights(q: u32) {
    let (s, c) = (scale(), card_metrics());
    assert_eq!((row_h(), head_h()), (s.px(ROW_H), s.px(ROW_H) + s.hair()), "q={q}");
    assert!((0..9).all(|k| row_y(k + 1) - row_y(k) == row_h()), "q={q}: table rows");
    assert_eq!(sec_geom::list_y(), c.card_h + s.px(BAND_GAP), "q={q}: findings card");
    let step = s.px(ALERT_H) + s.px(ALERT_GAP);
    assert!((0..9).all(|k| sec_geom::row_y(k + 1) - sec_geom::row_y(k) == step), "q={q}");
    assert_eq!(sec_geom::row_y(0), sec_geom::list_y() + c.panel_head_h, "q={q}");
}
