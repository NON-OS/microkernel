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

//! The process monitor's frame fits at 1x, 1.25x, 1.5x and 2x in a window grown
//! by the same scale: nav rows above the sidebar's foot, the chips clear of the
//! search field, End Process above the status strip, each screen's rows in its
//! pane. Its own binary, so the scale races no other test.

use apps_proofs::metrics::metrics;
use apps_proofs::pm::state::SCREENS;
use apps_proofs::pm::ui::fit::{inspector, sidebar_w};
use apps_proofs::pm::ui::nav_geom::{row_w, row_x, row_y};
use apps_proofs::pm::ui::pane_geom::{pane_rect, table_rect};
use apps_proofs::pm::ui::screens::sec_geom::{self, ALERT_H};
use apps_proofs::pm::ui::table_geom::{head_h, row_h, row_y as table_row_y, visible_rows};
use apps_proofs::pm::ui::{chips, insp_geom, matrix_geom, search};
use apps_proofs::pm_window::size;
use nonos_toolkit::design::scale::{scale, set_scale};

fn the_frame_fits_the_window(q: u32) {
    let ((w, h), m) = (size(), metrics());
    let band = (m.pane_pad_top, m.pane_pad_top + m.head_h);
    let last = row_y(SCREENS.len() - 1) + m.nav_h;
    assert!(last <= h - m.side_foot_h && row_x() + row_w(w) <= sidebar_w(w), "q={q}: nav");
    for screen in SCREENS {
        let (docked, r) = (inspector(screen, w), pane_rect(w, h, inspector(screen, w)));
        let right = if docked { insp_geom::pane_x(w) } else { w };
        assert!(r.y == band.1 && r.x + r.w + m.pane_pad_x <= right, "q={q}: pane");
        assert!(r.y + r.h + m.status_h <= h, "q={q}: the pane meets the status strip");
        let (sx, sy, sw, sh) = search::rect(w, screen);
        assert!(sy >= band.0 && sy + sh <= band.1 && sx + sw <= right, "q={q}: search");
        if let Some(x) = chips::origin(w, screen) {
            assert!(x + chips::width() + m.search_meta_gap <= sx, "q={q}: chips meet search");
        }
    }
    let (bx, by, bw, bh) = insp_geom::btn(w, h);
    assert!(bx >= insp_geom::pane_x(w) && bx + bw <= w, "q={q}: End Process across");
    assert!(by + bh + m.status_h <= h, "q={q}: End Process meets the status strip");
}

fn every_screens_rows_fit_its_pane(q: u32) {
    let (s, (w, h)) = (scale(), size());
    let (pane, fits) = (pane_rect(w, h, true), |r_h| table_row_y(visible_rows(r_h)) <= r_h);
    let t = table_rect(&pane);
    assert!(fits(pane.h) && fits(t.h) && t.y + t.h == pane.y + pane.h, "q={q}: table rows");
    let grid = pane_rect(w, h, false);
    let n = matrix_geom::visible_rows(grid.h) as u32;
    assert!(head_h() + n * row_h() <= matrix_geom::grid_h(grid.h), "q={q}: matrix rows");
    let shown = sec_geom::visible(grid.h);
    assert!(shown > 0 && sec_geom::row_y(shown - 1) + s.px(ALERT_H) <= grid.h, "q={q}");
}

/// Both fit proofs at each scale in turn, in one test so no two set the scale at once.
#[test]
fn layout_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        the_frame_fits_the_window(q);
        every_screens_rows_fit_its_pane(q);
    }
    set_scale(4);
}
