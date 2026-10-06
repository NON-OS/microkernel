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

use crate::browser::css::Computed;

use super::super::display_list::DisplayList;

/// Mark what a gradient-masked box painted, the fragments from `start` on
/// with its own border box first: that one carries the mask, and paint
/// draws each of them through it. A fragment already inside an inner
/// masked box keeps that mask alone; the outer one is not stacked on it.
pub(crate) fn apply_fade(s: &Computed, frags: &mut DisplayList, start: usize) {
    /* A filter's color map goes on all it painted, an inner one kept. */
    if s.fx.tint != 0 {
        for f in frags.iter_mut().skip(start).filter(|f| f.tint == 0) {
            f.tint = s.fx.tint;
        }
    }
    if s.fx.fade == 0 {
        return;
    }
    let Some(own) = frags.get_mut(start) else { return };
    let node = own.node;
    if node == 0 {
        return;
    }
    (own.fade, own.fade_isect) = (s.fx.fade, s.fx.fade_isect);
    for f in frags.iter_mut().skip(start).filter(|f| f.fade_by == 0) {
        f.fade_by = node;
    }
}
