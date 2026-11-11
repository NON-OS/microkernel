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

//! The file manager's sidebar at 1x, 1.25x, 1.5x and 2x in a window grown by
//! the same scale: its width and row height are the 1x ones read through the
//! scale, every row fits the window, every row with somewhere to go is named
//! where it is drawn and none overlap, and a section label names nothing. Its
//! own binary, so setting the scale races no other test.

use apps_proofs::fm::layout::metrics;
use apps_proofs::fm::sidebar_model::SideKind;
use apps_proofs::fm::sidebar_rows::{side_at, side_rows};
use apps_proofs::fm::target::Target;
use apps_proofs::fm_check::named_apart;
use apps_proofs::fm_window::dims;
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::widgets::Rect;

#[test]
fn every_sidebar_row_with_a_place_is_named_where_it_is_drawn_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (m, s) = (metrics(), scale());
        assert_eq!((m.sidebar_w, m.side_row_h), (s.px(236), s.px(34)), "q={q}");
        let sizes = [dims(1100, 700), dims(820, 520), dims(1600, 1000)];
        for (favs, (_, h)) in
            [(vec![], sizes[0]), (vec!["/docs/a/", "/b/"], sizes[1]), (vec!["/c/"], sizes[2])]
        {
            let rows = side_rows(&favs, h);
            assert!(rows.iter().all(|r| r.y + r.h <= h), "q={q}: the rail fits {favs:?}");
            let mut cs = Vec::new();
            for (i, r) in rows.iter().enumerate().filter(|(_, r)| r.hit.is_some()) {
                cs.push((Target::Side(i), Rect { x: 0, y: r.y, w: m.sidebar_w, h: r.h }));
            }
            assert!(cs.len() >= 7 + favs.len(), "q={q}: {} rows for {favs:?}", cs.len());
            let what = format!("q={q} {favs:?}");
            named_apart(&cs, |_, y| side_at(&rows, y).map(Target::Side), &what);
            for r in rows.iter().filter(|r| r.kind == SideKind::Label) {
                assert_eq!(side_at(&rows, r.y + r.h / 2), None, "q={q}: the {} label", r.label);
            }
        }
    }
    set_scale(4);
}
