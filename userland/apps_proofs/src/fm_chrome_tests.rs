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

//! The file manager's selection band and sidebar: every action and every row
//! with somewhere to go is named where it is drawn and none overlap, while the
//! band's ground and the sidebar's labels hold a point but name nothing.

use nonos_toolkit::widgets::Rect;

use crate::fm::header_slots::TOOL_H;
use crate::fm::layout::SIDEBAR_W;
use crate::fm::sel_geom::{action_y, band, band_at};
use crate::fm::sel_slots::slots;
use crate::fm::sidebar_model::SideKind;
use crate::fm::sidebar_rows::{side_at, side_rows};
use crate::fm::target::Target;
use crate::fm_check::named_apart;

const SIZES: [(u32, u32); 3] = [(1100, 700), (820, 520), (1600, 1000)];

#[test]
fn every_band_action_is_named_where_it_is_drawn_and_its_ground_names_nothing() {
    for (w, h) in SIZES {
        let b = band(w, h);
        let ay = action_y(&b);
        let at = |s: &crate::fm::sel_slots::SelSlot| Rect { x: s.x, y: ay, w: s.w, h: TOOL_H };
        let cs: Vec<_> =
            slots(&b).iter().enumerate().map(|(i, s)| (Target::Band(i), at(s))).collect();
        assert_eq!(cs.len(), 5, "{w}x{h}");
        named_apart(&cs, |x, y| band_at(w, h, x, y).flatten(), &alloc::format!("band {w}x{h}"));
        assert_eq!(band_at(w, h, b.x + 1, b.y + 1), Some(None), "the ground holds the point");
        assert_eq!(band_at(w, h, b.x + 1, b.y.saturating_sub(1)), None, "above the band");
    }
}

#[test]
fn every_sidebar_row_with_a_place_is_named_and_a_label_names_nothing() {
    for (favs, (_, h)) in
        [(vec![], SIZES[0]), (vec!["/docs/a/", "/b/"], SIZES[1]), (vec!["/c/"], SIZES[2])]
    {
        let rows = side_rows(&favs, h);
        let mut cs = Vec::new();
        for (i, r) in rows.iter().enumerate().filter(|(_, r)| r.hit.is_some()) {
            cs.push((Target::Side(i), Rect { x: 0, y: r.y, w: SIDEBAR_W, h: r.h }));
        }
        assert!(cs.len() >= 7 + favs.len(), "{} rows for {favs:?}", cs.len());
        named_apart(&cs, |_, y| side_at(&rows, y).map(Target::Side), &alloc::format!("{favs:?}"));
        for r in rows.iter().filter(|r| r.kind == SideKind::Label) {
            assert_eq!(side_at(&rows, r.y + r.h / 2), None, "the {} label", r.label);
        }
    }
}
