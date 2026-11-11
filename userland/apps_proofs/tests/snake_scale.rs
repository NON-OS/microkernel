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

//! Snake keeps logical metrics and draws them at the display scale: at 1x,
//! 1.25x, 1.5x and 2x every size is its 1x size through `scale().px` or
//! `scale().font`, and a window grown by the same scale holds the HUD, the
//! board, the rail and the footer apart and inside it. Its own binary, so
//! setting the scale races no other test.

use apps_proofs::snake::grid::{WIN_H, WIN_W};
use apps_proofs::snake::ui::metrics::metrics;
use apps_proofs::snake::ui::play_geom::{board, foot_band, hud_band, rail, stage};
use apps_proofs::snake::ui::play_geom_rows::{foot, rail_row, FOOT_BTNS};
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::widgets::Rect;

/// The rail rows the play screen paints: its heads, the tip, power and lives.
const PAINTED_RAIL_ROWS: usize = 8;

/// Every length, then every font size, the screens lay out with.
#[rustfmt::skip]
fn sizes() -> (Vec<u32>, Vec<f32>) {
    let m = metrics();
    let lengths = [
        m.pad, m.pad_tight, m.gap, m.gap_tight, m.gap_wide, m.radius_panel, m.radius_cell,
        m.rail_w, m.hud_h, m.hud_card_gap, m.foot_h, m.foot_btn_w, m.foot_btn_h, m.btn_h,
        m.btn_w, m.btn_gap, m.chip_h, m.chip_gap, m.modal_w, m.modal_h, m.row_h,
        m.table_head_h, m.icon_sm, m.icon_md,
    ];
    (lengths.to_vec(), vec![m.px_wordmark, m.px_title, m.px_head, m.px_body, m.px_label, m.px_stat])
}

fn inside(o: Rect, r: Rect) -> bool {
    r.x >= o.x && r.y >= o.y && r.x + r.w <= o.x + o.w && r.y + r.h <= o.y + o.h
}

#[test]
fn every_size_grows_with_the_scale_and_the_play_screen_still_fits() {
    set_scale(4);
    let ((lengths, fonts), cell_1x) = (sizes(), board(WIN_W, WIN_H).cell);
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (s, m) = (scale(), metrics());
        let px = lengths.iter().map(|&n| s.px(n)).collect();
        let want = (px, fonts.iter().map(|&f| s.font(f)).collect());
        assert_eq!(sizes(), want, "q={q}: a size is not its 1x size at the scale");
        let (w, h) = (s.px(WIN_W), s.px(WIN_H));
        let (win, b, st, r) = (Rect { x: 0, y: 0, w, h }, board(w, h), stage(w, h), rail(w, h));
        assert!(inside(st, Rect { x: b.x, y: b.y, w: b.w, h: b.h }), "q={q}: board leaves stage");
        assert!(b.cell + 1 >= s.px(cell_1x), "q={q}: the board does not grow with the scale");
        assert!(st.x + st.w + m.gap <= r.x && inside(win, r), "q={q}: stage meets the rail");
        assert_eq!(r.w, m.rail_w, "q={q}: the rail is not its full width");
        let (hud, band) = (hud_band(w, h), foot_band(w, h));
        assert!(hud.y + hud.h <= st.y && st.y + st.h <= band.y, "q={q}: the bands overlap");
        let rows_fit = (0..PAINTED_RAIL_ROWS).all(|i| inside(r, rail_row(w, h, i)));
        assert!(rows_fit, "q={q}: a row leaves the rail");
        for f in (0..FOOT_BTNS).map(|i| foot(w, h, i)) {
            assert!(inside(band, f) && (f.w, f.h) == (m.foot_btn_w, m.foot_btn_h), "q={q}: {f:?}");
        }
    }
    set_scale(4);
}
