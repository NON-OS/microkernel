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

use super::grad_paint::GradPaint;
use super::raster::Raster;

/// What a fill or stroke lays down: one colour or a gradient.
pub(super) enum Shade {
    Solid(u32),
    Grad(GradPaint),
}

/// A shade and the clip coverage it is confined to.
pub(super) struct Brush<'a> {
    pub shade: Shade,
    pub clip: Option<&'a Raster>,
}

impl Brush<'_> {
    /// The colour at supersampled device pixel (x, y); None where the clip
    /// leaves it out.
    pub fn at(&self, x: i32, y: i32) -> Option<u32> {
        if self.clip.is_some_and(|m| m.alpha(x, y) == 0) {
            return None;
        }
        Some(match &self.shade {
            Shade::Solid(c) => *c,
            Shade::Grad(g) => g.at(x as f32 + 0.5, y as f32 + 0.5),
        })
    }
}

/// `c` with its alpha scaled by `k` in 0..=1.
pub(super) fn fade(c: u32, k: f32) -> u32 {
    let a = ((c >> 24) as f32 * k.clamp(0.0, 1.0) + 0.5) as u32;
    a.min(255) << 24 | (c & 0x00ff_ffff)
}
