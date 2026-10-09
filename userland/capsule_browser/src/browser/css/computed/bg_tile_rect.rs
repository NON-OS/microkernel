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

use crate::browser::layout::boxmodel::rel;

use super::bg_layer::{BgLayer, BgLen, BgSize};

impl BgLayer {
    /// Where the tile anchored by background-position sits for an image of
    /// natural size `nat` in the box `b` ([x, y, w, h]): its [x, y, w, h].
    /// Repeats step from it by its size in both directions.
    pub fn tile(&self, nat: (u32, u32), b: [i32; 4]) -> [i32; 4] {
        let (bw, bh) = (b[2].max(0) as f32, b[3].max(0) as f32);
        let (nw, nh) = (nat.0.max(1) as f32, nat.1.max(1) as f32);
        let (w, h) = match self.size {
            BgSize::Auto => (nw, nh),
            BgSize::Cover => scaled(nw, nh, (bw / nw).max(bh / nh)),
            BgSize::Contain => scaled(nw, nh, (bw / nw).min(bh / nh)),
            BgSize::Wh(a, c) => match (side(a, bw), side(c, bh)) {
                (Some(w), Some(h)) => (w, h),
                (Some(w), None) => (w, w * nh / nw),
                (None, Some(h)) => (h * nw / nh, h),
                (None, None) => (nw, nh),
            },
        };
        let (w, h) = ((w + 0.5).max(1.0) as i32, (h + 0.5).max(1.0) as i32);
        let x = b[0].saturating_add(rel(self.pos[0], b[2].saturating_sub(w)));
        let y = b[1].saturating_add(rel(self.pos[1], b[3].saturating_sub(h)));
        [x, y, w, h]
    }
}

fn scaled(w: f32, h: f32, s: f32) -> (f32, f32) {
    (w * s, h * s)
}

fn side(l: BgLen, base: f32) -> Option<f32> {
    match l {
        BgLen::Auto => None,
        BgLen::Px(p) => Some(p as f32),
        BgLen::Pct(pm) => Some(base * pm as f32 / 1000.0),
    }
}
