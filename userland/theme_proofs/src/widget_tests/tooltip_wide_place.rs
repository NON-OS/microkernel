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

//! Tooltip: one given a wider `max_width` stays in every box it is drawn in,
//! in every state at every scale, and `place` keeps it inside bounds both
//! wider and narrower than itself, centred on the anchor unless an edge
//! holds it back.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::widgets::{Rect, Tooltip};

use super::bounds::assert_bounded;
use super::placing::{anchors, fault, Across, Side};
use super::samples::LONG;
use super::serial::{serial, SCALES};

/// Anywhere by the anchor, centred on it unless an edge holds it back.
const FREE: (Side, Across) = (Side::Free, Across::Centre);

#[test]
fn tooltip_wide_stays_in_its_box_and_bounds() {
    let _g = serial();
    let tips = [120, 640, 900].map(|w| Tooltip::new(LONG).max_width(w));
    assert_bounded(&tips.map(|t| (format!("{t:?}"), t)));
    for q in SCALES {
        set_scale(q);
        for (w, t) in [scale().px(1000), 300].into_iter().flat_map(|w| tips.map(|t| (w, t))) {
            let bounds = Rect { x: 40, y: 30, w, h: 200 };
            for (anchor, _) in anchors(bounds, 20) {
                let why = fault(t.place(anchor, bounds), t.size(), anchor, bounds, FREE);
                assert_eq!(why, None, "{t:?} at {q}/4 by {anchor:?} in {bounds:?}");
            }
        }
    }
    set_scale(SCALES[0]);
}
