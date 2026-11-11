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

//! The file manager's selection band at 1x, 1.25x, 1.5x and 2x in a window
//! grown by the same scale: the strip's sizes are the 1x ones read through the
//! scale, the band and its actions fit, every action is named where it is
//! drawn and none overlap, and the band's ground holds a point but names
//! nothing. Its own binary, so setting the scale races no other test.

use apps_proofs::fm::header_slots::strip;
use apps_proofs::fm::layout::metrics;
use apps_proofs::fm::sel_geom::{action_y, band, band_at};
use apps_proofs::fm::sel_slots::{slots, SelSlot};
use apps_proofs::fm::target::Target;
use apps_proofs::fm_check::named_apart;
use apps_proofs::fm_window::dims;
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::widgets::Rect;

#[test]
fn every_band_action_is_named_where_it_is_drawn_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (t, s) = (strip(), scale());
        assert_eq!((t.h, t.icon_btn, t.glyph, t.px), (s.px(32), s.px(30), s.px(16), s.font(17.0)));
        for (w, h) in [dims(1100, 700), dims(820, 520), dims(1600, 1000)] {
            let (b, tool_h) = (band(w, h), t.h);
            let ay = action_y(&b);
            assert!(ay >= b.y && ay + tool_h <= b.y + b.h, "q={q}: the actions fit the band");
            assert!(b.y + b.h + metrics().footer_h <= h, "q={q}: the band clears the footer");
            let at = |s: &SelSlot| Rect { x: s.x, y: ay, w: s.w, h: tool_h };
            let cs: Vec<_> =
                slots(&b).iter().enumerate().map(|(i, s)| (Target::Band(i), at(s))).collect();
            assert_eq!(cs.len(), 5, "q={q} {w}x{h}");
            let last = cs.last().map(|(_, r)| r.x + r.w).unwrap_or(0);
            assert!(!b.labelled || last <= b.x + b.w, "q={q}: a labelled row fits the band");
            let what = format!("q={q} band {w}x{h}");
            named_apart(&cs, |x, y| band_at(w, h, x, y).flatten(), &what);
            assert_eq!(band_at(w, h, b.x + 1, b.y + 1), Some(None), "q={q}: the ground");
            assert_eq!(band_at(w, h, b.x + 1, b.y.saturating_sub(1)), None, "q={q}: above");
        }
    }
    set_scale(4);
}
