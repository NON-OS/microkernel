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

use super::clip::clip_mask;
use super::raster::Raster;
use super::state::Paint;
use super::walk::Walk;

/// How many clipped groups may nest, each holding a band-sized mask; a
/// clipped group deeper than that is not painted.
const MAX_MASKS: usize = 4;

impl Walk<'_, '_> {
    /// Push the mask of a group's `clip-path` value, cut to the clip it
    /// already sits in, and point `p` at it. None when the reference does
    /// not resolve, which hides the group. objectBoundingBox units resolve
    /// against the viewBox, the group's own box not being known before
    /// its children are walked.
    pub(super) fn group_clip(&mut self, value: &str, p: &mut Paint, cur: Paint) -> Option<()> {
        if self.masks.len() >= MAX_MASKS {
            return None;
        }
        let [x, y, w, h] = self.defs.view;
        let m = clip_mask(self.defs, value, &p.t, [x, y, x + w, y + h], self.r)?;
        let m = intersect(m, cur.clip.and_then(|i| self.masks.get(i)));
        self.masks.push(m);
        p.clip = Some(self.masks.len() - 1);
        Some(())
    }
}

/// `m` cut down to where `outer` also covers.
pub(super) fn intersect(mut m: Raster, outer: Option<&Raster>) -> Raster {
    if let Some(o) = outer {
        m.px.iter_mut().zip(&o.px).filter(|(_, o)| **o >> 24 == 0).for_each(|(p, _)| *p = 0);
    }
    m
}
