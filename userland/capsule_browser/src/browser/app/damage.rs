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

use nonos_toolkit::decorations::Rect;

use crate::browser::omnibox::{rect_of, Damage};
use crate::browser::state::{State, CHROME_H};

/* Hand the runner the rect this repaint must cover, and remember which
 * parts the paint that follows draws. A repaint with nothing recorded, a
 * size not yet known, an empty rect or the settings panel open (it
 * overlays the page) draws everything. */
pub(super) fn take(state: &mut State) -> Option<Rect> {
    let t = &mut state.track;
    let mut parts = core::mem::take(&mut t.damage);
    t.painted_gen = t.paint_gen;
    if parts.is_empty() || state.settings_open || state.viewport_w == 0 {
        parts = Damage::FULL;
    }
    let r = rect_of(parts, state.viewport_w, state.viewport_h + CHROME_H);
    let Some(r) = r.filter(|r| r.w != 0 && r.h != 0) else {
        t.painting = Damage::FULL;
        return None;
    };
    t.painting = parts;
    Some(Rect { x: r.x, y: r.y, w: r.w, h: r.h })
}
